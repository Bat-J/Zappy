mod args;
mod server;

fn main() -> std::io::Result<()> {
    let config = args::parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(84);
    });

    let mut server = server::start(config)?;

    loop {
        let ready = server.poll()?;
        server.accept(&ready)?;
        let read = server.read(&ready)?;
        server.cleanup(ready.disconnected.into_iter().chain(read.disconnected));
    }
}
