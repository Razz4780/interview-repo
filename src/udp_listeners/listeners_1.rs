use std::net::SocketAddr;

use crate::udp_listeners::IncomingPacket;

/// Stream of incoming UDP packets.
pub struct IncomingUdp {
    // ...todo
}

impl IncomingUdp {
    /// Creates a new instance - stream of packets incoming on the given local socket address.
    pub fn new(_socket_addr: SocketAddr) -> anyhow::Result<Self> {
        todo!()
    }

    /// Returns the next received packet.
    pub async fn recv(&mut self) -> anyhow::Result<IncomingPacket> {
        todo!()
    }
}

// Bonus points!
//
// Implement `futures::Stream` for `IncomingUdp`.
//
// impl Stream for IncomingUdp {
//
// }
