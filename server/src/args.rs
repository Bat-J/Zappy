use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub width: u32,
    pub height: u32,
    pub teams: Vec<String>,
    pub clients_per_team: u32,
    pub frequency: u32,
}

const USAGE: &str =
    "USAGE: ./zappy_server -p port -x width -y height -n name1 name2 ... -c clientsNb -f freq";

const DEFAULT_PORT: u16 = 4242;
const DEFAULT_WIDTH: u32 = 10;
const DEFAULT_HEIGHT: u32 = 10;
const DEFAULT_CLIENTS: u32 = 1;
const DEFAULT_FREQ: u64 = 100;

pub fn parse() -> Result<Config, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help") {
        return Err(USAGE.to_string());
    }

    let mut port = DEFAULT_PORT;
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut teams = vec!["default".to_string()];
    let mut clients_per_team = DEFAULT_CLIENTS;
    let mut freq = DEFAULT_FREQ;

    let mut i = 0;
    while i < args.len() as u32 {
        match args[i as usize].as_str() {
            "-p" => port = next_value::<u16>(&args, &mut i, "-p")?,
            "-x" => width = next_value::<u32>(&args, &mut i, "-x")?,
            "-y" => height = next_value::<u32>(&args, &mut i, "-y")?,
            "-c" => clients_per_team = next_value::<u32>(&args, &mut i, "-c")?,
            "-f" => freq = next_value::<u64>(&args, &mut i, "-f")?,
            "-n" => teams = collect_names(&args, &mut i)?,
            flag => return Err(format!("unknown flag: {flag}\n{USAGE}")),
        }
        i += 1;
    }

    Ok(Config {
        port,
        width,
        height,
        teams,
        clients_per_team,
        frequency: freq as u32,
    })
}

fn next_value<T>(args: &[String], i: &mut u32, flag: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    *i += 1;
    args.get(*i as usize)
        .ok_or_else(|| format!("{flag} requires a value"))?
        .parse()
        .map_err(|e| format!("{flag}: {e}"))
}

fn collect_names(args: &[String], i: &mut u32) -> Result<Vec<String>, String> {
    let mut names = Vec::new();

    while let Some(next) = args.get((*i + 1) as usize) {
        if next.starts_with('-') {
            break;
        }
        *i += 1;
        names.push(args[*i as usize].clone());
    }

    if names.is_empty() {
        return Err(format!("-n requires at least one team name\n{USAGE}"));
    }

    Ok(names)
}
