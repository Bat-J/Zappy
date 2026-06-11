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

struct Player {
    x: usize,
    y: usize,
    orientation: u8,
    level: u8,
    inventory: [u32; 7],
}

enum ClientState {
    AwaitingTeamName,
    AwaitingEgg { team: String },
    Ai { team: String, player: Player },
    Gui,
}

pub struct Client {
    id: u32,
    pub stream: TcpStream,
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
    next_client_id: u32,
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
                        if let ClientState::Ai { ref team, .. } = client.state {
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
                        let _ = client.stream.write_all(b"ok\n");
                        let broadcast = format!("{}\n", msg);
                        for other in self.clients.iter_mut() {
                            if let ClientState::Ai { .. } = other.state {
                                let _ = other.stream.write_all(broadcast.as_bytes());
                            }
                        }
                        continue;
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

    pub fn read(&mut self, ready: &Ready, game: &mut Game) -> io::Result<ReadOutcome> {
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

    /// Call this whenever a new egg is added to `game.eggs`, to hatch any
    /// clients that were queued waiting for one.
    pub fn try_hatch_pending(&mut self, game: &mut Game) {
        for client in self.clients.iter_mut() {
            let team = match &client.state {
                ClientState::AwaitingEgg { team } => team.clone(),
                _ => continue,
            };

            let egg_idx = match game.eggs.iter().position(|e| e.team == team) {
                Some(idx) => idx,
                None => continue,
            };

            let egg = game.eggs.swap_remove(egg_idx);
            let remaining = self
                .team_slots
                .iter()
                .find(|(n, _)| n == &team)
                .map(|(_, s)| *s)
                .unwrap_or(0);

            client.id = egg.player_id;
            client.state = ClientState::Ai {
                team: team.clone(),
                player: Player {
                    x: egg.x as usize,
                    y: egg.y as usize,
                    orientation: 1,
                    level: 1,
                    inventory: [0; 7],
                },
            };

            let response = format!(
                "{remaining}\n{} {}\n",
                game.config.width, game.config.height
            );
            println!(
                "{} hatched egg at ({}, {}) for team {team} ({remaining} slots left)",
                client.addr, egg.x, egg.y
            );
            let _ = client.stream.write_all(response.as_bytes());
        }
    }

    pub fn cleanup(&mut self, disconnected: impl IntoIterator<Item = usize>) {
        let mut disconnected = disconnected.into_iter().collect::<Vec<_>>();
        disconnected.sort_unstable();
        disconnected.dedup();

        for index in disconnected.into_iter().rev() {
            if index < self.clients.len() {
                let client = self.clients.swap_remove(index);
                // Restore the slot for both active AI players and queued clients
                let team = match &client.state {
                    ClientState::Ai { team, .. } => Some(team.as_str()),
                    ClientState::AwaitingEgg { team } => Some(team.as_str()),
                    _ => None,
                };
                if let Some(team) = team
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
        game: &mut Game,
        time_manager: &mut TimeManager,
        client_id: u32,
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
        game: &mut Game,
        time_manager: &mut TimeManager,
        client_id: u32,
    ) -> bool {
        match client.state {
            ClientState::AwaitingTeamName => Self::handle_team_name(client, line, team_slots, game),
            ClientState::Ai { .. } => {
                Self::handle_ai_command(client, line, time_manager, client_id)
            }
            // Client is waiting for an egg to be laid — ignore all input until hatched
            ClientState::AwaitingEgg { .. } => false,
            ClientState::Gui => Self::handle_gui_command(client, line, game),
        }
    }

    fn handle_gui_command(client: &mut Client, line: &str, game: &mut Game) -> bool {
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.is_empty() {
            eprintln!("Parse error from {}: empty command", client.addr);
            return client.stream.write_all("ko\n".as_bytes()).is_err();
        }
        match parts[0] {
            "msz" => Self::msz_parse(parts, client, game),
            "bct" => Self::bct_parse(parts, client, game),
            "mct" => Self::mct_parse(parts, client, game),
            "tna" => Self::tna_parse(parts, client, game),
            "ppo" => false, //TODO implement ppo
            "plv" => false, // TODO implement plv
            "pin" => false, // TODO implement pin
            "sgt" => Self::sgt_parse(parts, client, game),
            "sst" => Self::sst_parse(parts, client, game),
            _ => Self::bad_command(parts, client),
        }
    }

    fn handle_ai_command(
        client: &mut Client,
        line: &str,
        time_manager: &mut TimeManager,
        client_id: u32,
    ) -> bool {
        match Command::parse(line) {
            Ok(cmd) => {
                let execute_at = time_manager.schedule(cmd.clone(), client_id);
                println!(
                    "AI command from {}: {} (execute at {:?})",
                    client.addr, cmd, execute_at
                );
                false
            }
            Err(err) => {
                eprintln!("Parse error from {}: {}", client.addr, err);
                client.stream.write_all(b"ko\n").is_err()
            }
        }
    }

    fn handle_team_name(
        client: &mut Client,
        name: &str,
        team_slots: &mut [(String, usize)],
        game: &mut Game,
    ) -> bool {
        if name == "GRAPHIC" {
            client.state = ClientState::Gui;
            println!("{} identified as GUI", client.addr);
            match create_gui_connection_string(&game.config, &game.map, &game.eggs) {
                Some(message) => {
                    return client.stream.write_all(message.as_bytes()).is_err();
                }
                None => return false,
            }
        }

        let slot = team_slots
            .iter_mut()
            .find(|(team, slots)| team == name && *slots > 0);

        match slot {
            Some(entry) => {
                entry.1 -= 1;
                let remaining = entry.1;

                if let Some(egg_idx) = game.eggs.iter().position(|egg| egg.team == name) {
                    // An egg is available: hatch it immediately
                    let egg = game.eggs.swap_remove(egg_idx);

                    client.id = egg.player_id;
                    client.state = ClientState::Ai {
                        team: name.to_string(),
                        player: Player {
                            x: egg.x as usize,
                            y: egg.y as usize,
                            orientation: 1,
                            level: 1,
                            inventory: [0; 7],
                        },
                    };

                    println!(
                        "{} hatched egg at ({}, {}) for team {name} ({remaining} slots left)",
                        client.addr, egg.x, egg.y
                    );
                } else {
                    // No egg yet: park the client until one is laid
                    client.state = ClientState::AwaitingEgg {
                        team: name.to_string(),
                    };
                    println!(
                        "{} waiting for an egg for team {name} ({remaining} slots left)",
                        client.addr
                    );
                }

                let response = format!(
                    "{remaining}\n{} {}\n",
                    game.config.width, game.config.height
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
