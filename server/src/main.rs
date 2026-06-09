use crate::server::Game;

mod args;
mod command;
mod map;
mod server;
mod time_manager;

fn main() -> std::io::Result<()> {
    let config = args::parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(84);
    });

    let mut server = server::start(config.clone())?;

    let game = Game {
        config: config.clone(),
        map: map::create_map(&config),
        eggs: map::start_spawn_eggs(&config),
    };

    loop {
        server.execute_ready_commands();
        let timeout_ms = server.next_poll_timeout_ms();
        let ready = server.poll(timeout_ms)?;
        server.accept(&ready)?;
        let read = server.read(&ready, &game)?;
        server.cleanup(ready.disconnected.into_iter().chain(read.disconnected));
    }
}
