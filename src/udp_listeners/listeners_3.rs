//! Extend solution implemented in [`super::listeners_2`].
//!
//! The packets contain JSON-encoded (string->string) maps,
//! and the tasks are only interesting in some of them.
//!
//! 1. When a task calls [`super::listeners_2::IncomingUdpMux::start_receiving_on`] it provides an [`EventFilter`].
//! 2. Returned [`super::listeners_2::IncomingUdpShared`] should yield [`Event`]s instead of raw packets.
//! 3. Returned [`super::listeners_2::IncomingUdpShared`] should only yield [`Event`]s that match the filter.

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

/// Event decoded from [`super::IncomingPacket`].
#[derive(Clone, Debug)]
pub struct Event {
    pub local_addr: SocketAddr,
    pub peer_addr: SocketAddr,
    pub payload: Arc<HashMap<String, String>>,
}

/// Filter for incoming [`Event`].
///
/// An [`Event`] matches the filter when [`Event::payload`] is a superset of the filter.
pub type EventFilter = HashMap<String, String>;
