//! Network resource contracts.

use serde::{Deserialize, Serialize};

use super::ResourceId;

/// Network resource inventory and bandwidth.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NetworkResource {
    /// Resource identity.
    pub id: ResourceId,
    /// Interface or link name.
    pub interface: String,
    /// Receive bandwidth in bytes per second.
    pub rx_bytes_per_sec: Option<u64>,
    /// Transmit bandwidth in bytes per second.
    pub tx_bytes_per_sec: Option<u64>,
    /// Whether the link is routable across distributed nodes.
    pub distributed_reachable: bool,
}

impl NetworkResource {
    /// Creates a network resource description.
    #[must_use]
    pub fn new(id: ResourceId, interface: impl Into<String>) -> Self {
        Self {
            id,
            interface: interface.into(),
            rx_bytes_per_sec: None,
            tx_bytes_per_sec: None,
            distributed_reachable: false,
        }
    }

    /// Sets bandwidth hints.
    #[must_use]
    pub fn with_bandwidth(mut self, rx_bytes_per_sec: u64, tx_bytes_per_sec: u64) -> Self {
        self.rx_bytes_per_sec = Some(rx_bytes_per_sec);
        self.tx_bytes_per_sec = Some(tx_bytes_per_sec);
        self
    }

    /// Marks the network resource as distributed-reachable.
    #[must_use]
    pub fn distributed_reachable(mut self) -> Self {
        self.distributed_reachable = true;
        self
    }
}
