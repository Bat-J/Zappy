use crate::{
    map::{bct_reply, mct_reply, msz_reply, sgt_reply, tna_reply},
    server::{Client, ClientState, Game, Orientation, Player, Server},
};
use std::{cell::RefCell, io::Write};

impl Server {
    fn gui_error(client: &mut Client, err: &str) -> bool {
        let _ = client.stream.write_all(err.as_bytes()).is_err();
        false
    }

    pub fn msz_parse(parts: Vec<&str>, client: &mut Client, game: &Game) -> bool {
        if parts.len() > 1 {
            return Self::gui_error(client, "sbp\n");
        }
        client
            .stream
            .write_all(msz_reply(&game.config).as_bytes())
            .is_err()
    }

    pub fn bct_parse(parts: Vec<&str>, client: &mut Client, game: &Game) -> bool {
        if parts.len() != 3 || parts[1].parse::<u32>().is_err() || parts[2].parse::<u32>().is_err()
        {
            return Self::gui_error(client, "sbp\n");
        }
        match bct_reply(
            parts[1].parse::<u32>().unwrap(),
            parts[2].parse::<u32>().unwrap(),
            &game.map,
            &game.config,
        ) {
            Some(string) => client.stream.write_all(string.as_bytes()).is_err(),
            None => Self::gui_error(client, "smg Failed to create bct reply\n"),
        }
    }

    pub fn mct_parse(parts: Vec<&str>, client: &mut Client, game: &Game) -> bool {
        if parts.len() > 1 {
            return Self::gui_error(client, "sbp\n");
        }
        match mct_reply(&game.config, &game.map) {
            Some(string) => client.stream.write_all(string.as_bytes()).is_err(),
            None => {
                let _ = client
                    .stream
                    .write_all("smg Failed to create mct reply\n".as_bytes())
                    .is_err();
                false
            }
        }
    }

    pub fn tna_parse(parts: Vec<&str>, client: &mut Client, game: &Game) -> bool {
        if parts.len() > 1 {
            return Self::gui_error(client, "sbp\n");
        }
        client
            .stream
            .write_all(tna_reply(&game.config).as_bytes())
            .is_err()
    }

    pub fn ppo_parse(
        parts: Vec<&str>,
        clients: &mut RefCell<&mut Vec<Client>>,
        client_id: u32,
    ) -> bool {
        if parts.len() != 2 || !parts[1].starts_with("#") {
            return Self::gui_error(&mut clients.borrow_mut()[client_id as usize], "sbp\n");
        }
        match parts[1].chars().next().map(|c| &parts[1][c.len_utf8()..]) {
            Some(string) => {
                let player_id = string;
                if player_id.parse::<u32>().is_err() {
                    return Self::gui_error(&mut clients.borrow_mut()[client_id as usize], "sbp\n");
                }
                for client in clients.borrow().iter() {
                    match &client.state {
                        ClientState::Ai { team: _, player } => {
                            if player.id != player_id.parse::<u32>().unwrap() {
                                continue;
                            }
                            return clients.borrow_mut()[client_id as usize]
                                .stream
                                .write_all(ppo_reply(player).as_bytes())
                                .is_err();
                        }
                        _ => continue,
                    }
                }
                true
            }
            None => {
                return Self::gui_error(&mut clients.borrow_mut()[client_id as usize], "sbp");
            }
        }
    }

    pub fn sgt_parse(parts: Vec<&str>, client: &mut Client, game: &Game) -> bool {
        if parts.len() > 1 {
            return Self::gui_error(client, "sbp\n");
        }
        client
            .stream
            .write_all(sgt_reply(&game.config).as_bytes())
            .is_err()
    }

    pub fn sst_parse(parts: Vec<&str>, client: &mut Client, game: &mut Game) -> bool {
        if parts.len() != 2 || parts[1].parse::<u32>().is_err() {
            return Self::gui_error(client, "sbp\n");
        }
        game.config.frequency = parts[1].parse::<u32>().unwrap();
        client
            .stream
            .write_all(("sst ".to_string() + &game.config.frequency.to_string() + "\n").as_bytes())
            .is_err()
    }

    pub fn bad_command(parts: Vec<&str>, client: &mut Client) -> bool {
        eprintln!("unknown command: {}", parts[0]);
        Self::gui_error(client, "suc\n")
    }
}

pub fn ppo_reply(player: &Player) -> String {
    let mut string = String::new();
    string.push_str("ppo #");
    string.push_str(&player.id.to_string());
    string.push(' ');
    string.push_str(&player.x.to_string());
    string.push(' ');
    string.push_str(&player.y.to_string());
    string.push(' ');
    match player.orientation {
        Orientation::North => string.push('N'),
        Orientation::East => string.push('E'),
        Orientation::South => string.push('S'),
        Orientation::West => string.push('W'),
    }
    string.push('\n');
    string
}
