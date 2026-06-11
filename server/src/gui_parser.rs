use crate::{
    map::{bct_reply, mct_reply, msz_reply, sgt_reply, tna_reply},
    server::{Client, Game, Server},
};
use std::io::Write;

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
