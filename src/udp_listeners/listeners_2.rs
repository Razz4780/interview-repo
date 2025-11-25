use std::net::SocketAddr;

use crate::udp_listeners::IncomingPacket;

/// Multiplexes for streams of incoming UDP packets.
///
/// Allows multiple independent task to consume incoming UDP packets from the same local socket addresses.
///
/// # Example
///
/// ```
/// use std::net::SocketAddr;
/// use tokio::{net::UdpSocket, task::JoinSet};
/// use interview::udp_listeners::listeners_2::IncomingUdpMux;
///
/// async fn receive_one(mux: IncomingUdpMux, receive_on: SocketAddr) {
///     let mut stream = mux.start_receiving_on(receive_on).unwrap();
///     let packet = stream.recv().await.unwrap();
///     assert_eq!(packet.payload.as_ref(), b"hello there");
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let address = "127.0.0.1:55555".parse::<SocketAddr>().unwrap();
///     let mux = IncomingUdpMux::default();
///
///     let mut tasks = JoinSet::new();
///     tasks.spawn(receive_one(mux.clone(), address));
///     tasks.spawn(receive_one(mux.clone(), address));
///
///     let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
///     sender.send_to(b"hello there", address).await.unwrap();
///
///     tasks.join_all().await;
/// }
/// ```
pub struct IncomingUdpMux {
    // ...todo
}

impl IncomingUdpMux {
    /// Returns a [`IncomingUdpShared`] handle that will yield packets coming to the given socket address.
    ///
    /// The underlying UDP socket should be garbage-collected when the last [`IncomingUdpShared`] for it is dropped.
    pub fn start_receiving_on(
        &self,
        _socket_addr: SocketAddr,
    ) -> anyhow::Result<IncomingUdpShared> {
        todo!()
    }
}

impl Clone for IncomingUdpMux {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Default for IncomingUdpMux {
    fn default() -> Self {
        todo!()
    }
}

/// Opaque handle for a shared stream of incoming UDP packets.
pub struct IncomingUdpShared {
    // ...todo
}

impl IncomingUdpShared {
    /// Returns the next received packet.
    pub async fn recv(&mut self) -> anyhow::Result<IncomingPacket> {
        todo!()
    }
}

// Bonus points!
//
// Implement `futures::Stream` for `IncomingUdpShared`.
//
// impl Stream for IncomingUdpShared {
//
// }
