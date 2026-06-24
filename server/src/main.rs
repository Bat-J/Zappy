use crate::server::Game;
use std::time::{Duration, Instant};

mod args;
mod command;
mod gui_parser;
mod map;
mod server;
mod time_manager;

fn main() -> std::io::Result<()> {
    let config = args::parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(84);
    });

    let mut server = server::start(config.clone())?;

    let mut map = map::create_map(&config);
    map::spawn_resources(&config, &mut map);

    let mut game = Game {
        next_egg_id: config.clients_per_team * config.teams.len() as u32,
        config: config.clone(),
        map,
        eggs: map::start_spawn_eggs(&config),
        next_player_id: 0,
    };

    let spawn_interval_ms = (20 * 1000) / config.frequency as u64;
    let mut next_spawn = Instant::now() + Duration::from_millis(spawn_interval_ms);

    loop {
        server.execute_ready_commands(&config, &mut game);
        if let Some(winner) = server.check_win_condition() {
            server.end_game(&winner);
        }
        if server.is_game_over() {
            break;
        }

        let ms_to_spawn = next_spawn
            .saturating_duration_since(Instant::now())
            .as_millis()
            .min(i32::MAX as u128) as i32;
        let timeout_ms = match server.next_poll_timeout_ms() {
            -1 => ms_to_spawn,
            t => t.min(ms_to_spawn),
        };

        let ready = server.poll(timeout_ms)?;
        server.accept(&ready)?;
        server.decrement_food();
        server.try_hatch_pending(&mut game);
        let read = server.read(&ready, &mut game)?;
        server.cleanup(ready.disconnected.into_iter().chain(read.disconnected));

        if Instant::now() >= next_spawn {
            map::refill_resources(&game.config, &mut game.map);
            next_spawn += Duration::from_millis(spawn_interval_ms);
        }
    }
    Ok(())
}
