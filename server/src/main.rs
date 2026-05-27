mod server;

fn main() -> std::io::Result<()> {
    let mut server = server::start()?;

    loop {
        let ready = server.poll()?;
        server.accept(&ready)?;
        let read = server.read(&ready)?;
        let write_failures = server.answer(read.messages);
        server.cleanup(
            ready
                .disconnected
                .into_iter()
                .chain(read.disconnected)
                .chain(write_failures),
        );
    }
}
