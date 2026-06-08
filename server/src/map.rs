#[derive(Default, Clone)]
pub struct Tile {
    food: Food,
    ais: Vec<usize>,
    width: u32,
    height: u32,
}

#[derive(Default, Clone)]
pub struct Food {
    linemate: u32,
    deraumere: u32,
    sibur: u32,
    mendiane: u32,
    phiras: u32,
    thystame: u32,
}

pub fn create_map(width: u32, height: u32) -> Vec<Vec<Tile>> {
    let mut map = vec![vec![Tile::default(); width as usize]; height as usize];
    map[0][0].height = height;
    map[0][0].width = width;
    map
}

pub fn remove_ai_from_tile(x: u32, y: u32, map: &mut Vec<Vec<Tile>>, id: usize) {
    if x >= map[0][0].width
        || y >= map[0][0].height
        || !map[x as usize][y as usize].ais.contains(&id)
    {
        return;
    }
    map[x as usize][y as usize].ais.retain(|&x| x != id);
}

pub fn add_ai_to_tile(x: u32, y: u32, map: &mut Vec<Vec<Tile>>, id: usize) {
    if x >= map[0][0].width
        || y >= map[0][0].height
        || map[x as usize][y as usize].ais.contains(&id)
    {
        return;
    }
    map[x as usize][y as usize].ais.push(id);
}
