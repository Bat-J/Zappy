use crate::args::Config;

#[derive(Default, Clone)]
pub struct Tile {
    food: u32,
    resources: Resources,
    ais: Vec<usize>,
}

#[derive(Default, Clone)]
pub struct Resources {
    linemate: u32,
    deraumere: u32,
    sibur: u32,
    mendiane: u32,
    phiras: u32,
    thystame: u32,
}

pub fn create_map(config: Config) -> Vec<Vec<Tile>> {
    let map = vec![vec![Tile::default(); config.width as usize]; config.height as usize];
    map
}

pub fn remove_ai_from_tile(x: u32, y: u32, map: &mut Vec<Vec<Tile>>, config: &Config, id: usize) {
    if x >= config.width || y >= config.height || !map[y as usize][x as usize].ais.contains(&id) {
        return;
    }
    map[y as usize][x as usize].ais.retain(|&x| x != id);
}

pub fn add_ai_to_tile(x: u32, y: u32, map: &mut Vec<Vec<Tile>>, config: &Config, id: usize) {
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

pub fn bct_reply(x: u32, y: u32, map: &Vec<Vec<Tile>>, config: &Config) -> Option<String> {
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

pub fn mct_reply(config: &Config, map: &Vec<Vec<Tile>>) -> Option<String> {
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

pub fn create_map_string(config: &Config, map: &Vec<Vec<Tile>>) -> Option<String> {
    let mut map_string = String::new();

    map_string.push_str(&msz_reply(config));
    map_string.push_str(&sgt_reply(config));
    match mct_reply(config, map) {
        Some(string) => map_string.push_str(&string),
        None => return None,
    }
    map_string.push_str(&tna_reply(config));
    Some(map_string)
}
