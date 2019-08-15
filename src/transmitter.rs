use std::future::Future;
use std::io::Result;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use hashbrown::HashMap;
use mio::net::UdpSocket;
use wirehair_wrapper::wirehair::{WirehairDecoder, WirehairEncoder};

use crate::connection::Connection;
use crate::reactor::UdpReactor;
use crate::utils::Md4Hash;

struct SendFuture {}

impl Future for SendFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unimplemented!()
    }
}

struct ReceiveFuture {}

impl Future for ReceiveFuture {
    type Output = Result<(usize, SocketAddr)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unimplemented!()
    }
}

struct RetransmitFuture {}

impl Future for RetransmitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unimplemented!()
    }
}

/// An entity that can effectively and reliably transmit some raw data to a remote host
trait AsyncTransmitter {
    /// Receives some (possibly large) raw data from a single remote host
    fn receive(&self, buf: &mut [u8]) -> ReceiveFuture;

    /// Sends some (possibly large) raw data to a single remote host
    ///
    /// Transmission is performed in a multiplexed manner - it doesn't care about the order of
    /// the delivery (FIRO - first in, random out)
    fn transmit(&self, data: &[u8], to: &SocketAddr) -> SendFuture;

    /// Same as [retransmit](trait.AsyncTransmitter.retransmit), but data will be retransmitted
    /// only when condition is satisfied
    ///
    /// Condition is a closure (data_sender, data_digest) -> bool
    fn retransmit_if<C>(&self, condition: C, to: &SocketAddr) -> RetransmitFuture
    where
        C: Fn(&SocketAddr, &[u8]) -> bool;

    /// Sends some (possibly large) raw data to a single remote host
    ///
    /// Transmission is performed in a queued manner - delivery order matters (FIFO)
    fn transmit_sequential(&self, data: &[u8], to: &SocketAddr) -> SendFuture;

    fn retransmit_sequential_if<C>(&self, condition: C, to: &SocketAddr) -> RetransmitFuture
    where
        C: Fn(&SocketAddr, &[u8]) -> bool;
}

/// An entity that can effectively and reliably transmit some raw data to multiple remote hosts
trait AsyncMassTransmitter {
    /// Receives some (possibly large) raw data from multiple remote hosts
    fn receive(&self, buf: &mut [u8]) -> Box<[ReceiveFuture]>;

    /// Sends some (possibly large) raw data to multiple remote hosts
    ///
    /// Transmission is performed in a multiplexed manner - it doesn't care about the order of
    /// the delivery (FIRO - first in, random out)
    fn transmit(&self, data: &[u8], to: &[&SocketAddr]) -> Box<[SendFuture]>;

    /// Same as [retransmit](trait.AsyncMassTransmitter.retransmit), but data will be retransmitted
    /// only when condition is satisfied
    ///
    /// Condition is a closure (any_data_sender, data_digest) -> bool
    fn retransmit_if<C>(&self, condition: C, to: &[&SocketAddr]) -> Box<[RetransmitFuture]>
    where
        C: Fn(&SocketAddr, &[u8]) -> bool;

    /// Sends some (possibly large) raw data to multiple remote hosts
    ///
    /// Transmission is performed in a queued manner (but paralleled to each receiver) - delivery
    /// order matters (FIFO)
    fn transmit_sequential(&self, data: &[u8], to: &[&SocketAddr]) -> Box<[SendFuture]>;

    fn retransmit_sequential_if<C>(
        &self,
        condition: C,
        to: &[&SocketAddr],
    ) -> Box<[RetransmitFuture]>
    where
        C: Fn(&SocketAddr, &[u8]) -> bool;
}

struct Transmitter {
    socket: UdpSocket,
    reactor: UdpReactor,
    input_data_sources: HashMap<Md4Hash, WirehairDecoder>,
    output_data_sources: HashMap<Md4Hash, WirehairEncoder>,
    connections: HashMap<SocketAddr, Connection>,
}

impl Transmitter {
    fn new(socket: UdpSocket, event_capacity: usize) -> Result<Self> {
        let reactor = UdpReactor::new(&socket, event_capacity)?;

        Ok(Transmitter {
            socket,
            reactor,
            input_data_sources: HashMap::new(),
            output_data_sources: HashMap::new(),
            connections: HashMap::new(),
        })
    }
}

impl AsyncTransmitter for Transmitter {
    fn receive(&self, buf: &mut [u8]) -> ReceiveFuture {
        unimplemented!()
    }

    fn transmit(&self, data: &[u8], to: &SocketAddr) -> SendFuture {
        unimplemented!()
    }

    fn retransmit_if<C>(&self, condition: C, to: &SocketAddr) -> RetransmitFuture
    where
        C: Fn(&SocketAddr, &[u8]) -> bool,
    {
        unimplemented!()
    }

    fn transmit_sequential(&self, data: &[u8], to: &SocketAddr) -> SendFuture {
        unimplemented!()
    }

    fn retransmit_sequential_if<C>(&self, condition: C, to: &SocketAddr) -> RetransmitFuture
    where
        C: Fn(&SocketAddr, &[u8]) -> bool,
    {
        unimplemented!()
    }
}
