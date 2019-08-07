use std::borrow::{Borrow, BorrowMut};
use std::future::Future;
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use hashbrown::HashMap;
use uuid::Uuid;

const DEFAULT_MTU: u16 = 1200;

pub struct RTPConnection {
    mtu: u16,
    packets_sent_total: u32,
    packets_received_total: u32,
    packets_sent_lost_total: u32,
    packets_received_lost_total: u32,
    average_latency: f32,
    active_send_contexts: HashMap<Uuid, RTPSendContext>,
    active_receive_contexts: HashMap<Uuid, RTPReceiveContext>,
}

impl RTPConnection {
    fn new() -> RTPConnection {
        RTPConnection {
            mtu: DEFAULT_MTU,
            packets_sent_total: 0,
            packets_received_total: 0,
            packets_sent_lost_total: 0,
            packets_received_lost_total: 0,
            average_latency: 0.0,
            active_send_contexts: HashMap::new(),
            active_receive_contexts: HashMap::new(),
        }
    }
}

pub struct RTPSendContext {}

pub struct RTPReceiveContext {}

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
    connections: HashMap<SocketAddr, RTPConnection>,
}

impl RTPSocket {
    pub fn bind(addr: &impl ToSocketAddrs) -> Result<RTPSocket, Error> {
        match UdpSocket::bind(addr) {
            Ok(s) => Ok(RTPSocket {
                state: RTPSocketState::BOUND,
                socket: s,
                connections: HashMap::new(),
            }),
            Err(e) => Err(e),
        }
    }

    /// add to associated connection send queue
    pub fn send(
        &self,
        data: &[u8],
        addr: &SocketAddr,
    ) -> Box<dyn Future<Output = Result<(), Error>>> {
        unimplemented!()
    }

    /// add to send queue for each associated connection
    pub fn broadcast(
        &self,
        data: &[u8],
        addrs: &[&SocketAddr],
    ) -> Box<dyn Future<Output = Result<(), Error>>> {
        unimplemented!()
    }

    /// tries to retrieve a packet from receive queue
    pub fn receive(&self) -> Option<RTPPacket> {
        unimplemented!()
    }

    /// runs processing loop once (cleans everything up, sends everything, receives everything)
    pub fn run_once(&self) {
        unimplemented!()
    }

    /// get associated connection
    pub fn get_connection(&self, addr: &SocketAddr) -> Option<&RTPConnection> {
        unimplemented!()
    }

    pub fn create_connection(&mut self, addr: &SocketAddr) -> &RTPConnection {
        self.connections
            .entry(addr.clone())
            .or_insert_with(RTPConnection::new)
    }

    /// get all active send contexts
    fn get_send_contexts(&self) -> &HashMap<RTPSendContext, Uuid> {
        unimplemented!()
    }

    /// get all active receive contexts
    fn get_receive_contexts(&self) -> &HashMap<RTPReceiveContext, Uuid> {
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
