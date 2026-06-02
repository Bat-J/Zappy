use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward,
    Right,
    Left,
    Look,
    Inventory,
    Broadcast(String),
    Fork,
    Eject,
    Take(String),
    Set(String),
    Incantation,
}

impl Command {
    /// Parse a command from a string
    /// Returns (Command, time_cost) or error message
    pub fn parse(input: &str) -> Result<Self, String> {
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
            "Incantation" => Ok(Command::Incantation),
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
            Command::Fork => write!(f, "Fork"),
            Command::Eject => write!(f, "Eject"),
            Command::Take(obj) => write!(f, "Take {}", obj),
            Command::Set(obj) => write!(f, "Set {}", obj),
            Command::Incantation => write!(f, "Incantation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_commands() {
        assert_eq!(Command::parse("Forward").unwrap(), Command::Forward);
        assert_eq!(Command::parse("Right").unwrap(), Command::Right);
        assert_eq!(Command::parse("Left").unwrap(), Command::Left);
        assert_eq!(Command::parse("Look").unwrap(), Command::Look);
        assert_eq!(Command::parse("Inventory").unwrap(), Command::Inventory);
        assert_eq!(Command::parse("Fork").unwrap(), Command::Fork);
        assert_eq!(Command::parse("Eject").unwrap(), Command::Eject);
        assert_eq!(Command::parse("Incantation").unwrap(), Command::Incantation);
    }

    #[test]
    fn parse_commands_with_args() {
        assert_eq!(
            Command::parse("Broadcast hello world").unwrap(),
            Command::Broadcast("hello world".to_string())
        );
        assert_eq!(
            Command::parse("Take linemate").unwrap(),
            Command::Take("linemate".to_string())
        );
        assert_eq!(
            Command::parse("Set food").unwrap(),
            Command::Set("food".to_string())
        );
    }

    #[test]
    fn parse_whitespace_handling() {
        assert_eq!(Command::parse("  Forward  ").unwrap(), Command::Forward);
        assert_eq!(
            Command::parse("Broadcast   message with spaces").unwrap(),
            Command::Broadcast("message with spaces".to_string())
        );
    }

    #[test]
    fn parse_invalid_commands() {
        assert!(Command::parse("Unknown").is_err());
        assert!(Command::parse("").is_err());
        assert!(Command::parse("Broadcast").is_err());
        assert!(Command::parse("Take").is_err());
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
