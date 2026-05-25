use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::fd::AsRawFd;

const LISTEN_ADDR: &str = "0.0.0.0:25565";

struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

pub struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
}

pub struct Ready {
    pub accept: bool,
    pub readable: Vec<usize>,
    pub disconnected: Vec<usize>,
}

pub struct Message {
    pub client: usize,
    pub addr: SocketAddr,
    pub text: String,
}

pub struct ReadOutcome {
    pub messages: Vec<Message>,
    pub disconnected: Vec<usize>,
}

pub fn start() -> io::Result<Server> {
    let listener = TcpListener::bind(LISTEN_ADDR)?;
    println!("Listening on {LISTEN_ADDR}");

    Ok(Server {
        listener,
        clients: Vec::new(),
    })
}

impl Server {
    pub fn poll(&self) -> io::Result<Ready> {
        let mut fds = Vec::with_capacity(self.clients.len() + 1);
        fds.push(libc::pollfd {
            fd: self.listener.as_raw_fd(),
            events: libc::POLLIN | libc::POLLERR | libc::POLLHUP | libc::POLLNVAL,
            revents: 0,
        });

        for client in &self.clients {
            fds.push(libc::pollfd {
                fd: client.stream.as_raw_fd(),
                events: libc::POLLIN | libc::POLLERR | libc::POLLHUP | libc::POLLNVAL,
                revents: 0,
            });
        }

        loop {
            let result = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, -1) };
            if result >= 0 {
                break;
            }

            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }

            return Err(err);
        }

        let mut readable = Vec::new();
        let mut disconnected = Vec::new();

        for (index, fd) in fds.iter().enumerate().skip(1) {
            let revents = fd.revents;
            let index = index - 1;

            if revents & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL) != 0 {
                disconnected.push(index);
                continue;
            }

            if revents & libc::POLLIN != 0 {
                readable.push(index);
            }
        }

        Ok(Ready {
            accept: fds[0].revents & libc::POLLIN != 0,
            readable,
            disconnected,
        })
    }

    pub fn accept(&mut self, ready: &Ready) -> io::Result<()> {
        if !ready.accept {
            return Ok(());
        }

        let (stream, addr) = self.listener.accept()?;
        println!("Client connected: {addr}");
        self.clients.push(Client { stream, addr });
        Ok(())
    }

    pub fn read(&mut self, ready: &Ready) -> io::Result<ReadOutcome> {
        let mut messages = Vec::new();
        let mut disconnected = ready.disconnected.clone();
        let mut buffer = [0u8; 1024];

        for &index in &ready.readable {
            if index >= self.clients.len() || Self::read_client(&mut self.clients[index], index, &mut buffer, &mut messages) {
                disconnected.push(index);
            }
        }

        Ok(ReadOutcome {
            messages,
            disconnected,
        })
    }

    pub fn answer(&mut self, messages: Vec<Message>) -> Vec<usize> {
        let mut disconnected = Vec::new();

        for message in messages {
            if let Some(client) = self.clients.get_mut(message.client) {
                let response = format!("{}: {}", message.addr, message.text);
                if let Err(err) = client.stream.write_all(response.as_bytes()) {
                    eprintln!("Write error to {}: {err}", client.addr);
                    disconnected.push(message.client);
                }
            }
        }

        disconnected
    }

    pub fn cleanup(&mut self, disconnected: impl IntoIterator<Item = usize>) {
        let mut disconnected = disconnected.into_iter().collect::<Vec<_>>();
        disconnected.sort_unstable();
        disconnected.dedup();

        for index in disconnected.into_iter().rev() {
            if index < self.clients.len() {
                let addr = self.clients.swap_remove(index).addr;
                println!("Client disconnected: {addr}");
            }
        }
    }

    fn read_client(
        client: &mut Client,
        index: usize,
        buffer: &mut [u8; 1024],
        messages: &mut Vec<Message>,
    ) -> bool {
        match client.stream.read(buffer) {
            Ok(0) => true,
            Ok(n) => {
                let text = String::from_utf8_lossy(&buffer[..n]).into_owned();
                println!("Received from {}: {}", client.addr, text.trim_end());
                messages.push(Message {
                    client: index,
                    addr: client.addr,
                    text,
                });
                false
            }
            Err(err) => {
                eprintln!("Read error from {}: {err}", client.addr);
                true
            }
        }
    }
}
