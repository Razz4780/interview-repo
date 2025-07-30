use std::{collections::HashMap, net::SocketAddr, sync::Arc};

/// Shared handle to a set of UDP sockets.
///
/// The sockets are only used for receiving packets.
/// The packets contain events, which are JSON-encoded (string -> string) maps.
/// Malformed packets are ignored.
#[derive(Clone, Default)]
pub struct Listeners {
    // ...
}

impl Listeners {
    /// If this set does not have a socket bound to the given address,
    /// this function creates and binds the socket, returning a handle to it.
    ///
    /// If this set already has a socket bound to the given address, returns a handle to it.
    ///
    /// The socket is garbage-collected when the last handle to it is dropped.
    ///
    /// # Params
    ///
    /// * `addr` - the local address of the socket.
    /// * `filter` - allows for filtering out incoming events.
    ///   The returned [`Listener`] instance will only receive ([`Listener::recv`]) events that match the filter.
    ///   An event matches the filter if all of the filter keys are present in the event, with the same values.
    pub fn listen(
        &self,
        _addr: SocketAddr,
        _filter: HashMap<String, String>,
    ) -> anyhow::Result<Listener> {
        todo!()
    }
}

/// Handle to a UDP socket in the [`Listeners`] set.
pub struct Listener {
    // ...
}

impl Listener {
    /// Returns the next event from the related socket.
    pub async fn recv(&mut self) -> anyhow::Result<Event> {
        todo!()
    }
}

pub struct Event {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub data: Arc<HashMap<String, String>>,
}
