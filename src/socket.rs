use std::future::Future;
use std::io::Result;
use std::net::SocketAddr;
use std::task::{Context, Poll};
use std::time::Duration;

use atomic_counter::{AtomicCounter, ConsistentCounter};
use mio::net::UdpSocket as MUDPSocket;
use mio::{Event, Events, Poll as MPoll, PollOpt, Ready, Token};

struct SendFuture {}

impl Future for SendFuture {
    type Output = ();

    fn poll(self, cx: &mut Context) -> Poll<Self::Output> {}
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

    fn poll(self, cx: &mut Context) -> Poll<Self::Output> {}
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

struct UDPReactor {
    poll: MPoll,
    events_buf: Events,
}

impl UDPReactor {
    fn new(socket: &MUDPSocket, buf_size: usize) -> Result<Self> {
        let poll = MPoll::new()?;
        let events_buf = Events::with_capacity(buf_size);

        poll.register(socket, Token(0), Ready::readable(), PollOpt::edge())?;
        poll.register(socket, Token(1), Ready::writable(), PollOpt::edge())?;

        Ok(UDPReactor { poll, events_buf })
    }

    fn make_progress(&mut self) -> Result<usize> {
        self.poll
            .poll(&mut self.events_buf, Some(Duration::new(0, 0)))
    }

    fn can_read(&self) -> usize {
        self.events_buf
            .iter()
            .filter(|it| it.readiness().is_readable())
            .count()
    }

    fn can_write(&self) -> usize {
        self.events_buf
            .iter()
            .filter(|it| it.readiness().is_writable())
            .count()
    }
}
