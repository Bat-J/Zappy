use crate::args::Config;
use rand::RngExt;

#[derive(Default, Clone)]
pub struct Tile {
    pub food: u32,
    pub resources: Resources,
    pub ais: Vec<u32>,
}

#[derive(Default, Clone)]
pub struct Resources {
    pub linemate: u32,
    pub deraumere: u32,
    pub sibur: u32,
    pub mendiane: u32,
    pub phiras: u32,
    pub thystame: u32,
}

#[derive(Default)]
pub struct Egg {
    pub x: u32,
    pub y: u32,
    pub id: u32,
    pub player_id: u32,
    pub is_starting: bool,
    pub team: String,
}

pub fn create_map(config: &Config) -> Vec<Vec<Tile>> {
    let map = vec![vec![Tile::default(); config.width as usize]; config.height as usize];
    map
}

pub fn _remove_ai_from_tile(x: u32, y: u32, map: &mut [Vec<Tile>], config: &Config, id: u32) {
    if x >= config.width || y >= config.height || !map[y as usize][x as usize].ais.contains(&id) {
        return;
    }
    map[y as usize][x as usize].ais.retain(|&x| x != id);
}

pub fn _add_ai_to_tile(x: u32, y: u32, map: &mut [Vec<Tile>], config: &Config, id: u32) {
    if x >= config.width || y >= config.height || map[x as usize][y as usize].ais.contains(&id) {
        return;
    }
    map[y as usize][x as usize].ais.push(id);
}

pub fn msz_reply(config: &Config) -> String {
    let mut string =
        String::with_capacity(8 + config.width.ilog10() as usize + config.height.ilog10() as usize);
    string.push_str("msz ");
    string.push_str(&config.width.to_string());
    string.push(' ');
    string.push_str(&config.height.to_string());
    string.push('\n');
    string
}

pub fn sgt_reply(config: &Config) -> String {
    let mut string = String::with_capacity(6 + config.frequency.ilog10() as usize);
    string.push_str("sgt ");
    string.push_str(&config.frequency.to_string());
    string.push('\n');
    string
}

pub fn bct_reply(x: u32, y: u32, map: &[Vec<Tile>], config: &Config) -> Option<String> {
    if x >= config.width || y >= config.height {
        return None;
    }
    let mut string = String::with_capacity(
        17 + x.checked_ilog10().unwrap_or(0) as usize
            + y.checked_ilog10().unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .food
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .linemate
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .deraumere
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .sibur
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .mendiane
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .phiras
                .checked_ilog10()
                .unwrap_or(0) as usize
            + map[y as usize][x as usize]
                .resources
                .thystame
                .checked_ilog10()
                .unwrap_or(0) as usize,
    );
    string.push_str("bct ");
    string.push_str(&x.to_string());
    string.push(' ');
    string.push_str(&y.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].food.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.linemate.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.deraumere.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.sibur.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.mendiane.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.phiras.to_string());
    string.push(' ');
    string.push_str(&map[y as usize][x as usize].resources.thystame.to_string());
    string.push('\n');
    Some(string)
}

pub fn tna_reply(config: &Config) -> String {
    let mut string = String::new();
    for team in config.teams.iter() {
        string.push_str("tna ");
        string.push_str(team);
        string.push('\n');
    }
    string
}

pub fn mct_reply(config: &Config, map: &[Vec<Tile>]) -> Option<String> {
    let mut map_string = String::new();
    for y in 0..config.height {
        for x in 0..config.width {
            match bct_reply(x, y, map, config) {
                Some(string) => map_string.push_str(&string),
                None => return None,
            }
        }
    }
    Some(map_string)
}

pub fn create_gui_connection_string(
    config: &Config,
    map: &[Vec<Tile>],
    eggs: &Vec<Egg>,
) -> Option<String> {
    let mut map_string = String::new();

    map_string.push_str(&msz_reply(config));
    map_string.push_str(&sgt_reply(config));
    match mct_reply(config, map) {
        Some(string) => map_string.push_str(&string),
        None => return None,
    }
    map_string.push_str(&tna_reply(config));
    for egg in eggs {
        map_string.push_str(&enw_string(egg));
    }
    Some(map_string)
}

pub fn enw_string(egg: &Egg) -> String {
    let mut string = String::new();
    string.push_str("enw #");
    string.push_str(&egg.id.to_string());
    string.push_str(" #");
    if egg.is_starting {
        string.push_str("-1");
    } else {
        string.push_str(&egg.player_id.to_string());
    }
    string.push(' ');
    string.push_str(&egg.x.to_string());
    string.push(' ');
    string.push_str(&egg.y.to_string());
    string.push('\n');
    string
}

pub fn start_spawn_eggs(config: &Config) -> Vec<Egg> {
    let mut eggs = Vec::new();
    for id in 0..config.clients_per_team * config.teams.len() as u32 {
        let egg = Egg {
            x: rand::rng().random_range(0..config.width),
            y: rand::rng().random_range(0..config.height),
            id,
            player_id: 0,
            is_starting: true,
            team: String::new(),
        };
        eggs.push(egg);
    }
    eggs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn enw_basic() {
        assert_eq!(
            enw_string(&Egg {
                x: 5,
                y: 6,
                id: 0,
                player_id: 0,
                is_starting: false,
                team: String::new(),
            }),
            "enw #0 #0 5 6\n"
        );
    }

    #[test]
    fn enw_starting() {
        assert_eq!(
            enw_string(&Egg {
                x: 5,
                y: 6,
                id: 0,
                player_id: 0,
                is_starting: true,
                team: String::new(),
            }),
            "enw #0 #-1 5 6\n"
        );
    }

    #[test]
    fn tna_basic() {
        assert_eq!(
            tna_reply(&Config {
                clients_per_team: 0,
                port: 0,
                width: 0,
                height: 0,
                teams: vec!["team1".to_string(), "team2".to_string()],
                frequency: 0
            }),
            "tna team1\ntna team2\n"
        );
    }

    #[test]
    fn sgt_basic() {
        assert_eq!(
            sgt_reply(&Config {
                port: 0,
                width: 0,
                height: 0,
                teams: vec![],
                clients_per_team: 0,
                frequency: 10
            }),
            "sgt 10\n"
        );
    }

    #[test]
    fn msz_basic() {
        assert_eq!(
            msz_reply(&Config {
                port: 0,
                width: 10,
                height: 5,
                teams: vec![],
                clients_per_team: 0,
                frequency: 0
            }),
            "msz 10 5\n"
        );
    }
}
