use std::{fmt, io::Write};

use crate::{
    args::Config,
    gui_parser::{edi_reply, pbc_reply, pex_reply, pfk_reply, pgt_reply, pic_reply, pie_reply},
    map::{self, Egg, enw_string},
    server::{Client, ClientState, Game, Orientation, Player},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward,
    Right,
    Left,
    Look,
    Inventory,
    Broadcast(String),
    ConnectNbr,
    Fork,
    Eject,
    Take(String),
    Set(String),
    Incantation,
}

impl Command {
    /// Parse a command from a string
    /// Returns (Command, time_cost) or error message
    pub fn parse(
        input: &str,
        list: &Vec<&Player>,
        source: &Player,
        clients: &mut [Client],
    ) -> Result<Self, String> {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.is_empty() {
            return Err("empty command".to_string());
        }

        match parts[0] {
            "Forward" => Ok(Command::Forward),
            "Right" => Ok(Command::Right),
            "Left" => Ok(Command::Left),
            "Look" => Ok(Command::Look),
            "Inventory" => Ok(Command::Inventory),
            "Broadcast" => {
                if parts.len() < 2 {
                    return Err("Broadcast requires a message".to_string());
                }
                let message = parts[1..].join(" ");
                Ok(Command::Broadcast(message))
            }
            "Connect_nbr" => Ok(Command::ConnectNbr),
            "Fork" => Ok(Command::Fork),
            "Eject" => Ok(Command::Eject),
            "Take" => {
                if parts.len() < 2 {
                    return Err("Take requires an object".to_string());
                }
                let object = parts[1..].join(" ");
                Ok(Command::Take(object))
            }
            "Set" => {
                if parts.len() < 2 {
                    return Err("Set requires an object".to_string());
                }
                let object = parts[1..].join(" ");
                Ok(Command::Set(object))
            }
            "Incantation" => {
                send_string_to_all_guis(clients, pic_reply(source, list));
                Ok(Command::Incantation)
            }
            _ => Err(format!("unknown command: {}", parts[0])),
        }
    }

    /// Get the base execution time for this command (in deciseconds)
    pub fn execution_time(&self) -> u32 {
        match self {
            Command::Forward => 7,
            Command::Right => 7,
            Command::Left => 7,
            Command::Look => 7,
            Command::Inventory => 1,
            Command::Broadcast(_) => 7,
            Command::Fork => 42,
            Command::Eject => 7,
            Command::Take(_) => 7,
            Command::Set(_) => 7,
            Command::Incantation => 300,
            Command::ConnectNbr => 0,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Forward => write!(f, "Forward"),
            Command::Right => write!(f, "Right"),
            Command::Left => write!(f, "Left"),
            Command::Look => write!(f, "Look"),
            Command::Inventory => write!(f, "Inventory"),
            Command::Broadcast(msg) => write!(f, "Broadcast {}", msg),
            Command::ConnectNbr => write!(f, "Connect_nbr"),
            Command::Fork => write!(f, "Fork"),
            Command::Eject => write!(f, "Eject"),
            Command::Take(obj) => write!(f, "Take {}", obj),
            Command::Set(obj) => write!(f, "Set {}", obj),
            Command::Incantation => write!(f, "Incantation"),
        }
    }
}

fn convert_ressource_name_to_id(string: &str) -> u8 {
    match string {
        "linemate" => 1,
        "dismate" => 2,
        "sibur" => 3,
        "mendiane" => 4,
        "phiras" => 5,
        "thystame" => 6,
        _ => 0,
    }
}

pub fn send_string_to_all_guis(clients: &mut [Client], msg: String) {
    for client in clients.iter_mut() {
        match client.state {
            ClientState::Gui => {
                let _ = client.stream.write_all(msg.as_bytes()).is_err();
                continue;
            }
            _ => continue,
        }
    }
}

pub fn send_ai_command_to_gui(
    cmd: Command,
    player: &Player,
    incantation_success: bool,
    clients: &mut [Client],
    new_egg: &Egg,
) {
    match cmd {
        Command::Eject => send_string_to_all_guis(clients, pex_reply(player)),
        Command::Broadcast(string) => send_string_to_all_guis(clients, pbc_reply(player, &string)),
        Command::Incantation => {
            send_string_to_all_guis(clients, pie_reply(player.x, player.y, incantation_success))
        }
        Command::Fork => {
            send_string_to_all_guis(clients, pfk_reply(player));
            send_string_to_all_guis(clients, enw_string(new_egg));
        }
        Command::Take(string) => send_string_to_all_guis(
            clients,
            pgt_reply(player, convert_ressource_name_to_id(&string)),
        ),
        _ => (),
    }
}

pub fn take(client: &mut Client, game: &mut Game, resource: &str) -> String {
    let player = match &mut client.state {
        ClientState::Ai { player, .. } => player,
        _ => return "ko\n".to_string(),
    };
    let resource_id = convert_ressource_name_to_id(resource);
    match resource_id {
        1 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .linemate
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .linemate -= 1;
            player.inventory.linemate += 1;
            "ok\n".to_string()
        }
        2 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .deraumere
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .deraumere -= 1;
            player.inventory.deraumere += 1;
            "ok\n".to_string()
        }
        3 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .sibur
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .sibur -= 1;
            player.inventory.sibur += 1;
            "ok\n".to_string()
        }
        4 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .mendiane
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .mendiane -= 1;
            player.inventory.mendiane += 1;
            "ok\n".to_string()
        }
        5 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .phiras
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .phiras -= 1;
            player.inventory.phiras += 1;
            "ok\n".to_string()
        }
        6 => {
            if game.map[player.y as usize][player.x as usize]
                .resources
                .thystame
                == 0
            {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .thystame -= 1;
            player.inventory.thystame += 1;
            "ok\n".to_string()
        }
        _ => "ko\n".to_string(),
    }
}

pub fn set(client: &mut Client, game: &mut Game, resource: &str) -> String {
    let player = match &mut client.state {
        ClientState::Ai { player, .. } => player,
        _ => return "ko\n".to_string(),
    };
    let resource_id = convert_ressource_name_to_id(resource);
    match resource_id {
        1 => {
            if player.inventory.linemate == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .linemate += 1;
            player.inventory.linemate -= 1;
            "ok\n".to_string()
        }
        2 => {
            if player.inventory.deraumere == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .deraumere += 1;
            player.inventory.deraumere -= 1;
            "ok\n".to_string()
        }
        3 => {
            if player.inventory.sibur == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .sibur += 1;
            player.inventory.sibur -= 1;
            "ok\n".to_string()
        }
        4 => {
            if player.inventory.mendiane == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .mendiane += 1;
            player.inventory.mendiane -= 1;
            "ok\n".to_string()
        }
        5 => {
            if player.inventory.phiras == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .phiras += 1;
            player.inventory.phiras -= 1;
            "ok\n".to_string()
        }
        6 => {
            if player.inventory.thystame == 0 {
                return "ko\n".to_string();
            }
            game.map[player.y as usize][player.x as usize]
                .resources
                .thystame += 1;
            player.inventory.thystame -= 1;
            "ok\n".to_string()
        }
        _ => "ko\n".to_string(),
    }
}

pub fn fork(client: &mut Client, game: &mut Game, config: &Config) -> String {
    let player = match &client.state {
        ClientState::Ai { player, .. } => player,
        _ => return "ko\n".to_string(),
    };
    let team = match &client.state {
        ClientState::Ai { team, .. } => team,
        _ => return "ko\n".to_string(),
    };

    game.eggs.push(Egg {
        x: rand::random_range(0..config.width),
        y: rand::random_range(0..config.height),
        id: game.next_egg_id,
        player_id: player.id,
        is_starting: false,
        team: team.to_string(),
    });
    game.next_egg_id += 1;
    "ok\n".to_string()
}

pub fn eject(clients: &mut [Client], idx: u32, game: &mut Game, config: &Config) -> String {
    let (target_orientation, target_x, target_y) = {
        let player = match &clients[idx as usize].state {
            ClientState::Ai { player, .. } => player,
            _ => return "ko\n".to_string(),
        };

        let orientation = player.orientation.clone();
        let x = match orientation {
            Orientation::West => {
                if player.x == 0 {
                    config.width - 1
                } else {
                    player.x - 1
                }
            }
            Orientation::East => {
                if player.x >= config.width - 1 {
                    0
                } else {
                    player.x + 1
                }
            }
            _ => player.x,
        };
        let y = match orientation {
            Orientation::North => {
                if player.y == 0 {
                    config.height - 1
                } else {
                    player.y - 1
                }
            }
            Orientation::South => {
                if player.y >= config.height - 1 {
                    0
                } else {
                    player.y + 1
                }
            }
            _ => player.y,
        };

        (orientation, x, y)
    };

    game.map[target_y as usize][target_x as usize].ais.clear();
    for client in clients.iter_mut() {
        let player_to_move = match &mut client.state {
            ClientState::Ai { player, .. } => player,
            _ => continue,
        };
        if player_to_move.x == target_x && player_to_move.y == target_y {
            match target_orientation {
                Orientation::North => {
                    if player_to_move.y == 0 {
                        player_to_move.y = config.height - 1;
                    } else {
                        player_to_move.y -= 1;
                    }
                }
                Orientation::East => {
                    if player_to_move.x == config.width - 1 {
                        player_to_move.x = 0;
                    } else {
                        player_to_move.x += 1;
                    }
                }
                Orientation::West => {
                    if player_to_move.x == 0 {
                        player_to_move.x = config.width - 1;
                    } else {
                        player_to_move.x -= 1;
                    }
                }
                Orientation::South => {
                    if player_to_move.y == config.height - 1 {
                        player_to_move.y = 0;
                    } else {
                        player_to_move.y += 1;
                    }
                }
            }
        }
    }

    let mut to_remove = Vec::<u32>::new();

    for (i, egg) in game.eggs.iter().enumerate() {
        if egg.x == target_x && egg.y == target_y {
            send_string_to_all_guis(clients, edi_reply(egg));
            to_remove.push(i as u32);
        }
    }

    for i in to_remove.iter().rev() {
        game.eggs.remove(*i as usize);
    }

    "ok\n".to_string()
}

pub fn move_forward(client: &mut Client, config: &Config) -> String {
    match &mut client.state {
        ClientState::Ai { player, .. } => match player.orientation {
            Orientation::North => {
                if player.y == 0 {
                    player.y = config.height - 1;
                } else {
                    player.y -= 1;
                }
            }
            Orientation::East => player.x = (player.x + 1) % config.width,
            Orientation::South => player.y = (player.y + 1) % config.height,
            Orientation::West => {
                if player.x == 0 {
                    player.x = config.width - 1;
                } else {
                    player.x -= 1;
                }
            }
        },
        _ => return "ko\n".to_string(),
    }
    "ok\n".to_string()
}

pub fn turn_right(client: &mut Client) -> String {
    match &mut client.state {
        ClientState::Ai { player, .. } => {
            player.orientation = match player.orientation {
                Orientation::East => Orientation::South,
                Orientation::North => Orientation::East,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            }
        }
        _ => return "ko\n".to_string(),
    }
    "ok\n".to_string()
}

pub fn turn_left(client: &mut Client) -> String {
    match &mut client.state {
        ClientState::Ai { player, .. } => {
            player.orientation = match player.orientation {
                Orientation::East => Orientation::North,
                Orientation::North => Orientation::West,
                Orientation::South => Orientation::East,
                Orientation::West => Orientation::South,
            }
        }
        _ => return "ko\n".to_string(),
    }
    "ok\n".to_string()
}

fn forward(o: &Orientation) -> (i32, i32) {
    match o {
        Orientation::North => (0, -1),
        Orientation::East => (1, 0),
        Orientation::South => (0, 1),
        Orientation::West => (-1, 0),
    }
}

fn right(o: &Orientation) -> (i32, i32) {
    match o {
        Orientation::North => (1, 0),
        Orientation::East => (0, 1),
        Orientation::South => (-1, 0),
        Orientation::West => (0, -1),
    }
}

fn wrap(v: i32, max: i32) -> usize {
    (((v % max) + max) % max) as usize
}

fn tile_content(tile: &map::Tile) -> String {
    let mut items: Vec<&str> = Vec::new();

    items.extend(std::iter::repeat_n("player", tile.ais.len()));
    items.extend(std::iter::repeat_n("food", tile.food as usize));
    items.extend(std::iter::repeat_n(
        "linemate",
        tile.resources.linemate as usize,
    ));
    items.extend(std::iter::repeat_n(
        "deraumere",
        tile.resources.deraumere as usize,
    ));
    items.extend(std::iter::repeat_n("sibur", tile.resources.sibur as usize));
    items.extend(std::iter::repeat_n(
        "mendiane",
        tile.resources.mendiane as usize,
    ));
    items.extend(std::iter::repeat_n(
        "phiras",
        tile.resources.phiras as usize,
    ));
    items.extend(std::iter::repeat_n(
        "thystame",
        tile.resources.thystame as usize,
    ));

    items.join(" ")
}

pub fn look(client: &Client, map: &[Vec<map::Tile>]) -> String {
    let player = match &client.state {
        ClientState::Ai { player, .. } => player,
        _ => return "ko\n".to_string(),
    };
    let height = map.len() as i32;
    let width = if height > 0 { map[0].len() as i32 } else { 0 };
    if height == 0 || width == 0 {
        return "ko\n".to_string();
    }

    let view_distance = player.level as i32;

    let px = player.x as i32;
    let py = player.y as i32;

    let (fx, fy) = forward(&player.orientation);
    let (rx, ry) = right(&player.orientation);

    let mut cells: Vec<String> = Vec::new();
    {
        let x = wrap(px, width);
        let y = wrap(py, height);
        cells.push(tile_content(&map[y][x]));
    }
    for d in 1..view_distance {
        for offset in -(d - 1)..=d - 1 {
            let tx = px + fx * d + rx * offset;
            let ty = py + fy * d + ry * offset;
            let x = wrap(tx, width);
            let y = wrap(ty, height);
            cells.push(tile_content(&map[y][x]));
        }
    }

    format!("[{}]\n", cells.join(", "))
}

pub fn get_inventory(client: &Client) -> String {
    match &client.state {
        ClientState::Ai { player, .. } => {
            let mut inventory_string = String::new();
            inventory_string.push_str(&format!(
                "[food {}, linemate {}, deraumere {}, sibur {}, mendiane {}, phiras {}, thystame {}]\n",
                player.food,
                player.inventory.linemate,
                player.inventory.deraumere,
                player.inventory.sibur,
                player.inventory.mendiane,
                player.inventory.phiras,
                player.inventory.thystame
            ));
            inventory_string
        }
        _ => "ko\n".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_commands() {
        assert_eq!(
            Command::parse("Forward", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Forward
        );
        assert_eq!(
            Command::parse("Right", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Right
        );
        assert_eq!(
            Command::parse("Left", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Left
        );
        assert_eq!(
            Command::parse("Look", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Look
        );
        assert_eq!(
            Command::parse(
                "Inventory",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Inventory
        );
        assert_eq!(
            Command::parse("Fork", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Fork
        );
        assert_eq!(
            Command::parse("Eject", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Eject
        );
        assert_eq!(
            Command::parse(
                "Incantation",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Incantation
        );
    }

    #[test]
    fn parse_commands_with_args() {
        assert_eq!(
            Command::parse(
                "Broadcast hello world",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Broadcast("hello world".to_string())
        );
        assert_eq!(
            Command::parse(
                "Take linemate",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Take("linemate".to_string())
        );
        assert_eq!(
            Command::parse("Set food", &Vec::new(), &Player::default(), &mut Vec::new()).unwrap(),
            Command::Set("food".to_string())
        );
    }

    #[test]
    fn parse_whitespace_handling() {
        assert_eq!(
            Command::parse(
                "  Forward  ",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Forward
        );
        assert_eq!(
            Command::parse(
                "Broadcast   message with spaces",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .unwrap(),
            Command::Broadcast("message with spaces".to_string())
        );
    }

    #[test]
    fn parse_invalid_commands() {
        assert!(
            Command::parse("Unknown", &Vec::new(), &Player::default(), &mut Vec::new()).is_err()
        );
        assert!(Command::parse("", &Vec::new(), &Player::default(), &mut Vec::new()).is_err());
        assert!(
            Command::parse(
                "Broadcast",
                &Vec::new(),
                &Player::default(),
                &mut Vec::new()
            )
            .is_err()
        );
        assert!(Command::parse("Take", &Vec::new(), &Player::default(), &mut Vec::new()).is_err());
    }

    #[test]
    fn execution_times() {
        assert_eq!(Command::Forward.execution_time(), 7);
        assert_eq!(Command::Inventory.execution_time(), 1);
        assert_eq!(Command::Fork.execution_time(), 42);
        assert_eq!(Command::Incantation.execution_time(), 300);
    }

    #[test]
    fn command_display() {
        assert_eq!(format!("{}", Command::Forward), "Forward");
        assert_eq!(
            format!("{}", Command::Broadcast("test".to_string())),
            "Broadcast test"
        );
    }
}
