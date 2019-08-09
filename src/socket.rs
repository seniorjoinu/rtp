use std::future::Future;
use std::io::Result;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::task::{Context, Poll};

struct SendFuture<'a> {
    socket: UdpSocket
}

impl Future for SendFuture {
    type Output = ();

    fn poll(self, cx: &mut Context) -> Poll<Self::Output> {
        if
    }
}

struct ReceiveFuture {}

impl Future for ReceiveFuture {
    type Output = Result<(usize, SocketAddr)>;

    fn poll(self, cx: &mut Context) -> Poll<Self::Output> {
        unimplemented!()
    }
}

struct RetransmitFuture {}

impl Future for RetransmitFuture {
    type Output = ();

    fn poll(self, cx: &mut Context) -> Poll<Self::Output> {
        cx.waker().
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

    /// Sets transmitter into relay mode: any received data will also be effectively retransmitted
    /// to a single remote host
    fn retransmit(&self, to: &SocketAddr) -> RetransmitFuture;

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

    /// Sets transmitter into relay mode: any received data will also be effectively retransmitted
    /// to multiple remote hosts
    fn retransmit(&self, to: &[&SocketAddr]) -> Box<[RetransmitFuture]>;

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
}