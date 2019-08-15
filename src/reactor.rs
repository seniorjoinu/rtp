use std::io::Result;
use std::time::Duration;

use mio::net::UdpSocket as MUDPSocket;
use mio::{Events, Poll as MPoll, PollOpt, Ready, Token};

pub struct UdpReactor {
    poll: MPoll,
    events_buf: Events,
}

impl UdpReactor {
    pub fn new(socket: &MUDPSocket, buf_size: usize) -> Result<Self> {
        let poll = MPoll::new()?;
        let events_buf = Events::with_capacity(buf_size);

        poll.register(
            socket,
            Token(0),
            Ready::readable() | Ready::writable(),
            PollOpt::edge(),
        )?;

        Ok(UdpReactor { poll, events_buf })
    }

    pub fn make_progress(&mut self) -> Result<usize> {
        self.poll
            .poll(&mut self.events_buf, Some(Duration::new(0, 0)))
    }

    pub fn can_read(&self) -> usize {
        self.events_buf
            .iter()
            .filter(|it| it.readiness().is_readable())
            .count()
    }

    pub fn can_write(&self) -> usize {
        self.events_buf
            .iter()
            .filter(|it| it.readiness().is_writable())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};

    use crate::reactor::UdpReactor;

    #[test]
    fn udp_reactor_works_fine() {
        let socket_addr = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8080);
        let socket = mio::net::UdpSocket::bind(&socket_addr).unwrap();
        let mut reactor = match UdpReactor::new(&socket, 10) {
            Ok(t) => t,
            Err(e) => panic!(e),
        };

        let events_count = reactor.make_progress();

        assert!(events_count.is_ok());

        let read_count = reactor.can_read();
        let write_count = reactor.can_write();

        assert!(read_count > 0 || write_count > 0)
    }
}
