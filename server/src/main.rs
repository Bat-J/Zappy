mod args;
mod command;
mod server;
mod time_manager;

fn main() -> std::io::Result<()> {
    let config = args::parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(84);
    });

    let mut server = server::start(config)?;

    loop {
        server.execute_ready_commands();
        let timeout_ms = server.next_poll_timeout_ms();
        let ready = server.poll(timeout_ms)?;
        server.accept(&ready)?;
        let read = server.read(&ready)?;
        server.cleanup(ready.disconnected.into_iter().chain(read.disconnected));
    }
}
