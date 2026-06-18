use std::{fmt, io::Write};

use crate::{
    gui_parser::{pbc_reply, pex_reply, pfk_reply, pgt_reply, pic_reply, pie_reply},
    map::{Egg, enw_string},
    server::{Client, ClientState, Player},
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
