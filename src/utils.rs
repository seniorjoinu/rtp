use std::net::SocketAddr;

pub type Md4Hash = [u8; 32];
pub type RetransmitCondition = dyn Fn(&SocketAddr, &[u8]) -> bool;
