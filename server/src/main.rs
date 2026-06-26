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

    let mut current_frequency = config.frequency;
    let mut next_spawn =
        Instant::now() + Duration::from_millis((20 * 1000) / current_frequency as u64);
    let mut next_food_tick =
        Instant::now() + Duration::from_millis((1000 / current_frequency as u64).max(1));

    loop {
        server.execute_ready_commands(&config, &mut game);
        if let Some(winner) = server.check_win_condition() {
            server.end_game(&winner);
        }
        if server.is_game_over() {
            break;
        }

        let new_frequency = game.config.frequency;
        if new_frequency != current_frequency {
            let now = Instant::now();
            next_food_tick = now + Duration::from_millis((1000 / new_frequency as u64).max(1));
            next_spawn = now + Duration::from_millis((20 * 1000) / new_frequency as u64);
            current_frequency = new_frequency;
        }

        let spawn_interval_ms = (20 * 1000) / current_frequency as u64;
        let food_tick_interval_ms = (1000 / current_frequency as u64).max(1);

        let now = Instant::now();
        let ms_to_spawn = next_spawn
            .saturating_duration_since(now)
            .as_millis()
            .min(i32::MAX as u128) as i32;
        let ms_to_food = next_food_tick
            .saturating_duration_since(now)
            .as_millis()
            .min(i32::MAX as u128) as i32;
        let timeout_ms = match server.next_poll_timeout_ms() {
            -1 => ms_to_spawn.min(ms_to_food),
            t => t.min(ms_to_spawn).min(ms_to_food),
        };

        let ready = server.poll(timeout_ms)?;
        server.accept(&ready)?;
        server.try_hatch_pending(&mut game);
        let read = server.read(&ready, &mut game)?;
        server.cleanup(ready.disconnected.into_iter().chain(read.disconnected));

        let now = Instant::now();
        if now >= next_food_tick {
            server.decrement_food();
            next_food_tick += Duration::from_millis(food_tick_interval_ms);
        }

        if now >= next_spawn {
            // The reference server tops the map back up to its target densities
            // but does not push tile updates to the GUI on refill, so we don't
            // either — the GUI stays in sync through action-driven bct/pgt/pdr.
            map::refill_resources(&game.config, &mut game.map);
            next_spawn += Duration::from_millis(spawn_interval_ms);
        }
    }
    Ok(())
}
