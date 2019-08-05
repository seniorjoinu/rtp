use std::future::Future;
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use stash::Stash;
use uuid::Uuid;

pub struct Connection {}

pub struct SendContext {}

pub struct ReceiveContext {}

pub struct RTPPacket<'a> {
    data: Box<[u8]>,
    addr: &'a SocketAddr,
}

pub enum RTPError {}

pub enum RTPSocketState {
    BOUND,
    CLOSED,
}

pub struct RTPSocket {
    state: RTPSocketState,
    socket: UdpSocket,
}

impl RTPSocket {
    fn bind(addr: &impl ToSocketAddrs) -> Result<RTPSocket, Error> {
        match UdpSocket::bind(addr) {
            Ok(s) => Ok(RTPSocket {
                state: RTPSocketState::BOUND,
                socket: s,
            }),
            Err(e) => Err(e),
        }
    }

    /// add to associated connection send queue
    fn send(&self, data: &[u8], addr: &SocketAddr) -> Box<dyn Future<Output = Result<(), Error>>> {
        unimplemented!()
    }

    /// add to send queue for each associated connection
    fn broadcast(
        &self,
        data: &[u8],
        addrs: &[&SocketAddr],
    ) -> Box<dyn Future<Output = Result<(), Error>>> {
        unimplemented!()
    }

    /// tries to retrieve a packet from receive queue
    fn receive(&self) -> Option<RTPPacket> {
        unimplemented!()
    }

    /// runs processing loop once (cleans everything up, sends everything, receives everything)
    fn run_once(&self) {
        unimplemented!()
    }

    /// get associated connection
    fn get_connection(&self, addr: &SocketAddr) -> &Connection {
        unimplemented!()
    }

    /// get all active send contexts
    fn get_send_contexts(&self) -> &Stash<&SendContext, Uuid> {
        unimplemented!()
    }

    /// get all active receive contexts
    fn get_receive_contexts(&self) -> &Stash<&ReceiveContext, Uuid> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::RTPSocket;

    #[test]
    fn able_to_create_rtp_socket() {
        let result = RTPSocket::bind(&"localhost:1234");
        assert!(result.is_ok())
    }
}
