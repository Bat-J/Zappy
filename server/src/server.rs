use std::cell::RefCell;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::fd::AsRawFd;
use std::os::raw::{c_int, c_short, c_ulong};

use crate::args::Config;
use crate::command::{
    Command, move_forward, send_ai_command_to_gui, send_string_to_all_guis, turn_left, turn_right,
};
use crate::gui_parser::{ebo_reply, pdi_reply};
use crate::map::{Egg, Tile};
use crate::map::{Resources, create_gui_connection_string};
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

#[derive(Clone, Default)]
#[allow(dead_code)]
pub enum Orientation {
    #[default]
    North = 1,
    East = 2,
    South = 3,
    West = 4,
}

#[derive(Clone, Default)]
pub struct Player {
    pub x: u32,
    pub y: u32,
    pub id: u32,
    pub orientation: Orientation,
    pub level: u8,
    pub inventory: Resources,
    pub food: u32,
    pub time_left: u8,
}

#[derive(Clone)]
pub enum ClientState {
    AwaitingTeamName,
    AwaitingEgg { team: String },
    Ai { team: String, player: Player },
    Gui,
}

pub struct Client {
    id: u32,
    pub stream: TcpStream,
    addr: SocketAddr,
    pub state: ClientState,
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
    team_slots: Vec<(String, u32)>,
    time_manager: TimeManager,
    next_client_id: u32,
}

pub struct Ready {
    pub accept: bool,
    pub readable: Vec<u32>,
    pub disconnected: Vec<u32>,
}

pub struct ReadOutcome {
    pub disconnected: Vec<u32>,
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

    pub fn execute_ready_commands(&mut self, config: &Config) {
        let ready = self.time_manager.tick();
        for scheduled in ready {
            println!(
                "EXECUTE: client {} - {}",
                scheduled.client_id, scheduled.command
            );

            let reference = RefCell::new(&mut self.clients);

            if let Some(client) = reference
                .borrow_mut()
                .iter_mut()
                .find(|c| c.id == scheduled.client_id)
            {
                let response = match scheduled.command {
                    Command::Forward => {
                        move_forward(client, config);
                        "ok\n".to_string()
                    }
                    Command::Right => {
                        turn_right(client);
                        "ok\n".to_string()
                    }
                    Command::Left => {
                        turn_left(client);
                        "ok\n".to_string()
                    }
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
                        for other in reference.borrow_mut().iter_mut() {
                            if let ClientState::Ai { .. } = other.state {
                                let _ = other.stream.write_all(broadcast.as_bytes());
                            }
                        }
                        continue;
                    }
                    _ => "ok\n".to_string(),
                };

                let _ = client.stream.write_all(response.as_bytes());

                if let ClientState::Ai { player, .. } = &client.state {
                    send_ai_command_to_gui(
                        scheduled.command,
                        player,
                        false, /* TODO implement check for incantation */
                        *reference.borrow_mut(),
                        &Egg::default(), /* TODO implement from fork */
                    );
                }
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
            let index = (index - 1) as u32;

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
            if index >= self.clients.len() as u32 {
                disconnected.push(index);
                continue;
            }

            let client_id = self.clients[index as usize].id;
            let drop = Self::read_client(
                &mut self.clients,
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
        let reference = RefCell::new(&mut self.clients);

        for client in reference.borrow_mut().iter_mut() {
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
                    x: egg.x,
                    y: egg.y,
                    id: egg.id,
                    orientation: Orientation::North,
                    level: 1,
                    inventory: Resources {
                        linemate: 0,
                        deraumere: 0,
                        sibur: 0,
                        mendiane: 0,
                        phiras: 0,
                        thystame: 0,
                    },
                    food: 10,
                    time_left: 0,
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
            send_string_to_all_guis(*reference.borrow_mut(), ebo_reply(&egg));
        }
    }

    pub fn cleanup(&mut self, disconnected: impl IntoIterator<Item = u32>) {
        let mut disconnected = disconnected.into_iter().collect::<Vec<_>>();
        disconnected.sort_unstable();
        disconnected.dedup();

        for index in disconnected.into_iter().rev() {
            if index < self.clients.len() as u32 {
                let client = self.clients.swap_remove(index as usize);
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

    pub fn decrement_food(&mut self) {
        let mut i = 0;
        let reference = RefCell::new(&mut self.clients);

        for client in reference.borrow_mut().iter_mut() {
            match client.state.clone() {
                ClientState::Ai { mut player, .. } => {
                    player.time_left -= 1;
                    if player.time_left == 0 {
                        player.food -= 1;
                        if player.food == 0 {
                            send_string_to_all_guis(*reference.borrow_mut(), pdi_reply(&player));
                            let _ = client.stream.shutdown(std::net::Shutdown::Both);
                            reference.borrow_mut().remove(i);
                            continue;
                        }
                    }
                    i += 1;
                }
                _ => continue,
            }
        }
    }

    fn read_client(
        clients: &mut Vec<Client>,
        buffer: &mut [u8; 1024],
        team_slots: &mut [(String, u32)],
        game: &mut Game,
        time_manager: &mut TimeManager,
        client_id: u32,
    ) -> bool {
        let n = match clients[client_id as usize].stream.read(buffer) {
            Ok(0) => return true,
            Ok(n) => n,
            Err(err) => {
                eprintln!(
                    "Read error from {}: {err}",
                    clients[client_id as usize].addr
                );
                return true;
            }
        };

        clients[client_id as usize]
            .pending
            .push_str(&String::from_utf8_lossy(&buffer[..n]));

        while let Some(pos) = clients[client_id as usize].pending.find('\n') {
            let line = clients[client_id as usize].pending[..pos]
                .trim_end_matches('\r')
                .to_string();
            clients[client_id as usize].pending.drain(..=pos);

            if Self::handle_line(clients, &line, team_slots, game, time_manager, client_id) {
                return true;
            }
        }

        false
    }

    fn handle_line(
        clients: &mut Vec<Client>,
        line: &str,
        team_slots: &mut [(String, u32)],
        game: &mut Game,
        time_manager: &mut TimeManager,
        client_id: u32,
    ) -> bool {
        let reference = RefCell::new(clients);
        match &mut reference.borrow_mut()[client_id as usize].state {
            ClientState::AwaitingTeamName => Self::handle_team_name(
                &mut reference.borrow_mut()[client_id as usize],
                line,
                team_slots,
                game,
            ),
            ClientState::Ai { player, .. } => Self::handle_ai_command(
                &mut reference.borrow_mut()[client_id as usize],
                &reference,
                line,
                time_manager,
                client_id,
                player,
            ),
            // Client is waiting for an egg to be laid — ignore all input until hatched
            ClientState::AwaitingEgg { .. } => false,
            ClientState::Gui => {
                Self::handle_gui_command(&mut reference.borrow_mut(), client_id, line, game)
            }
        }
    }

    fn handle_gui_command(
        clients: &mut Vec<Client>,
        client_id: u32,
        line: &str,
        game: &mut Game,
    ) -> bool {
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.is_empty() {
            eprintln!(
                "Parse error from {}: empty command",
                clients[client_id as usize].addr
            );
            return clients[client_id as usize]
                .stream
                .write_all("ko\n".as_bytes())
                .is_err();
        }
        match parts[0] {
            "msz" => Self::msz_parse(parts, &mut clients[client_id as usize], game),
            "bct" => Self::bct_parse(parts, &mut clients[client_id as usize], game),
            "mct" => Self::mct_parse(parts, &mut clients[client_id as usize], game),
            "tna" => Self::tna_parse(parts, &mut clients[client_id as usize], game),
            "ppo" => Self::ppo_parse(parts, &mut RefCell::new(clients), client_id),
            "plv" => Self::plv_parse(parts, &mut RefCell::new(clients), client_id),
            "pin" => Self::pin_parse(parts, &mut RefCell::new(clients), client_id),
            "sgt" => Self::sgt_parse(parts, &mut clients[client_id as usize], game),
            "sst" => Self::sst_parse(parts, &mut clients[client_id as usize], game),
            _ => Self::bad_command(parts, &mut clients[client_id as usize]),
        }
    }

    fn handle_ai_command(
        client: &mut Client,
        clients: &RefCell<&mut Vec<Client>>,
        line: &str,
        time_manager: &mut TimeManager,
        client_id: u32,
        player: &Player,
    ) -> bool {
        let mut list: Vec<&Player> = Vec::new();
        let mut content = clients.borrow_mut();
        for c in content.iter_mut() {
            match &c.state {
                ClientState::Ai { player, .. } => list.push(player),
                _ => continue,
            }
        }
        match Command::parse(line, &list, player, &mut clients.borrow_mut()) {
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
        team_slots: &mut [(String, u32)],
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
                            x: egg.x,
                            y: egg.y,
                            id: egg.id,
                            orientation: Orientation::North,
                            level: 1,
                            inventory: Resources {
                                linemate: 0,
                                deraumere: 0,
                                sibur: 0,
                                mendiane: 0,
                                phiras: 0,
                                thystame: 0,
                            },
                            food: 10,
                            time_left: 0,
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
