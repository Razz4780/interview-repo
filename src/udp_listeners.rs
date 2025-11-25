use std::net::SocketAddr;

use bytes::Bytes;

pub mod listeners_1;
pub mod listeners_2;
pub mod listeners_3;

#[derive(Debug, Clone)]
pub struct IncomingPacket {
    pub local_addr: SocketAddr,
    pub peer_addr: SocketAddr,
    pub payload: Bytes,
}
