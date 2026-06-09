use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::fd::AsRawFd;
use std::os::raw::{c_int, c_short, c_ulong};

use crate::args::Config;
use crate::command::Command;
use crate::map::create_gui_connection_string;
use crate::map::{Egg, Tile};
use crate::time_manager::TimeManager;

type Nfds = c_ulong;

const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;
const POLLNVAL: c_short = 0x0020;

#[repr(C)]
struct PollFd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

unsafe extern "C" {
    fn poll(fds: *mut PollFd, nfds: Nfds, timeout: c_int) -> c_int;
}

enum ClientState {
    AwaitingTeamName,
    Ai { team: String },
    Gui,
}

struct Client {
    id: usize,
    stream: TcpStream,
    addr: SocketAddr,
    state: ClientState,
    pending: String,
}

pub struct Game {
    pub map: Vec<Vec<Tile>>,
    pub config: Config,
    pub eggs: Vec<Egg>,
}

pub struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
    team_slots: Vec<(String, usize)>,
    time_manager: TimeManager,
    next_client_id: usize,
}

pub struct Ready {
    pub accept: bool,
    pub readable: Vec<usize>,
    pub disconnected: Vec<usize>,
}

pub struct ReadOutcome {
    pub disconnected: Vec<usize>,
}

pub fn start(config: Config) -> io::Result<Server> {
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on {addr}");

    let team_slots = config
        .teams
        .into_iter()
        .map(|name| (name, config.clients_per_team))
        .collect();

    let time_manager = TimeManager::new(config.frequency);

    Ok(Server {
        listener,
        clients: Vec::new(),
        team_slots,
        time_manager,
        next_client_id: 0,
    })
}

impl Server {
    pub fn next_poll_timeout_ms(&self) -> c_int {
        self.time_manager.next_timeout_ms().unwrap_or(-1)
    }

    pub fn execute_ready_commands(&mut self) {
        let ready = self.time_manager.tick();
        for scheduled in ready {
            println!(
                "EXECUTE: client {} - {}",
                scheduled.client_id, scheduled.command
            );

            // Find the client by id
            if let Some(client) = self
                .clients
                .iter_mut()
                .find(|c| c.id == scheduled.client_id)
            {
                let response = match scheduled.command {
                    Command::Look => "[ player ]\n".to_string(),
                    Command::Inventory => {
                        "[food 0, linemate 0, deraumere 0, sibur 0, mendiane 0, phiras 0, thystame 0]\n".to_string()
                    }
                    Command::ConnectNbr => {
                        // find team name for this client
                        if let ClientState::Ai { ref team } = client.state {
                            if let Some(slot) = self.team_slots.iter().find(|(name, _)| name == team) {
                                format!("{}\n", slot.1)
                            } else {
                                "0\n".to_string()
                            }
                        } else {
                            "0\n".to_string()
                        }
                    }
                    Command::Broadcast(ref msg) => {
                        // Send ok to sender and broadcast message to all AI clients
                        let _ = client.stream.write_all(b"ok\n");
                        let broadcast = format!("{}\n", msg);
                        for other in self.clients.iter_mut() {
                            if let ClientState::Ai { .. } = other.state {
                                let _ = other.stream.write_all(broadcast.as_bytes());
                            }
                        }
                        continue; // already wrote responses
                    }
                    _ => "ok\n".to_string(),
                };

                let _ = client.stream.write_all(response.as_bytes());
            } else {
                eprintln!("Client {} not found for execution", scheduled.client_id);
            }
        }
    }

    pub fn poll(&self, timeout_ms: c_int) -> io::Result<Ready> {
        let mut fds = Vec::with_capacity(self.clients.len() + 1);
        fds.push(PollFd {
            fd: self.listener.as_raw_fd(),
            events: POLLIN | POLLERR | POLLHUP | POLLNVAL,
            revents: 0,
        });

        for client in &self.clients {
            fds.push(PollFd {
                fd: client.stream.as_raw_fd(),
                events: POLLIN | POLLERR | POLLHUP | POLLNVAL,
                revents: 0,
            });
        }

        loop {
            let result = unsafe { poll(fds.as_mut_ptr(), fds.len() as Nfds, timeout_ms) };
            if result >= 0 {
                break;
            }

            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }

            return Err(err);
        }

        let mut readable = Vec::new();
        let mut disconnected = Vec::new();

        for (index, fd) in fds.iter().enumerate().skip(1) {
            let revents = fd.revents;
            let index = index - 1;

            if revents & (POLLERR | POLLHUP | POLLNVAL) != 0 {
                disconnected.push(index);
                continue;
            }

            if revents & POLLIN != 0 {
                readable.push(index);
            }
        }

        Ok(Ready {
            accept: fds[0].revents & POLLIN != 0,
            readable,
            disconnected,
        })
    }

    pub fn accept(&mut self, ready: &Ready) -> io::Result<()> {
        if !ready.accept {
            return Ok(());
        }

        let (mut stream, addr) = self.listener.accept()?;
        stream.write_all(b"WELCOME\n")?;
        println!("Client connected: {addr}");

        let client_id = self.next_client_id;
        self.next_client_id += 1;
        self.clients.push(Client {
            id: client_id,
            stream,
            addr,
            state: ClientState::AwaitingTeamName,
            pending: String::new(),
        });

        Ok(())
    }

    pub fn read(&mut self, ready: &Ready, game: &Game) -> io::Result<ReadOutcome> {
        let mut disconnected = ready.disconnected.clone();
        let mut buffer = [0u8; 1024];

        for &index in &ready.readable {
            if index >= self.clients.len() {
                disconnected.push(index);
                continue;
            }

            let client_id = self.clients[index].id;
            let drop = Self::read_client(
                &mut self.clients[index],
                &mut buffer,
                &mut self.team_slots,
                game,
                &mut self.time_manager,
                client_id,
            );

            if drop {
                disconnected.push(index);
            }
        }

        Ok(ReadOutcome { disconnected })
    }

    pub fn cleanup(&mut self, disconnected: impl IntoIterator<Item = usize>) {
        let mut disconnected = disconnected.into_iter().collect::<Vec<_>>();
        disconnected.sort_unstable();
        disconnected.dedup();

        for index in disconnected.into_iter().rev() {
            if index < self.clients.len() {
                let client = self.clients.swap_remove(index);
                if let ClientState::Ai { ref team } = client.state
                    && let Some(slot) = self.team_slots.iter_mut().find(|(name, _)| name == team)
                {
                    slot.1 += 1;
                }
                println!("Client disconnected: {}", client.addr);
            }
        }
    }

    fn read_client(
        client: &mut Client,
        buffer: &mut [u8; 1024],
        team_slots: &mut [(String, usize)],
        game: &Game,
        time_manager: &mut TimeManager,
        client_id: usize,
    ) -> bool {
        let n = match client.stream.read(buffer) {
            Ok(0) => return true,
            Ok(n) => n,
            Err(err) => {
                eprintln!("Read error from {}: {err}", client.addr);
                return true;
            }
        };

        client
            .pending
            .push_str(&String::from_utf8_lossy(&buffer[..n]));

        while let Some(pos) = client.pending.find('\n') {
            let line = client.pending[..pos].trim_end_matches('\r').to_string();
            client.pending.drain(..=pos);

            if Self::handle_line(client, &line, team_slots, game, time_manager, client_id) {
                return true;
            }
        }

        false
    }

    fn handle_line(
        client: &mut Client,
        line: &str,
        team_slots: &mut [(String, usize)],
        game: &Game,
        time_manager: &mut TimeManager,
        client_id: usize,
    ) -> bool {
        match client.state {
            ClientState::AwaitingTeamName => Self::handle_team_name(client, line, team_slots, game),
            ClientState::Ai { .. } => {
                Self::handle_ai_command(client, line, time_manager, client_id)
            }
            ClientState::Gui => {
                println!("GUI command from {}: {line}", client.addr);
                false
            }
        }
    }

    fn handle_ai_command(
        client: &mut Client,
        line: &str,
        time_manager: &mut TimeManager,
        client_id: usize,
    ) -> bool {
        match Command::parse(line) {
            Ok(cmd) => {
                let execute_at = time_manager.schedule(cmd.clone(), client_id);
                println!(
                    "AI command from {}: {} (execute at {:?})",
                    client.addr, cmd, execute_at
                );
                false // don't drop connection; don't send immediate response
            }
            Err(err) => {
                eprintln!("Parse error from {}: {}", client.addr, err);
                let response = "ko\n";
                client.stream.write_all(response.as_bytes()).is_err()
            }
        }
    }

    fn handle_team_name(
        client: &mut Client,
        name: &str,
        team_slots: &mut [(String, usize)],
        game: &Game,
    ) -> bool {
        if name == "GRAPHIC" {
            client.state = ClientState::Gui;
            println!("{} identified as GUI", client.addr);
            match create_gui_connection_string(&game.config, &game.map, &game.eggs) {
                Some(message) => {
                    return client.stream.write_all(message.as_bytes()).is_err();
                }
                None => {
                    return false;
                }
            }
        }

        let slot = team_slots
            .iter_mut()
            .find(|(team, slots)| team == name && *slots > 0);

        match slot {
            Some(entry) => {
                entry.1 -= 1;
                let remaining = entry.1;
                let response = format!(
                    "{remaining}\n{} {}\n",
                    game.config.width, game.config.height
                );
                client.state = ClientState::Ai {
                    team: name.to_string(),
                };
                println!(
                    "{} joined team {name} ({remaining} slots left)",
                    client.addr
                );
                client.stream.write_all(response.as_bytes()).is_err()
            }
            None => {
                eprintln!(
                    "{} refused: unknown team or no slots for '{name}'",
                    client.addr
                );
                true
            }
        }
    }
}
