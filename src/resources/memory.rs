//! System memory resource contracts.

use serde::{Deserialize, Serialize};

use super::ResourceId;

/// RAM resource inventory and limits.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemoryResource {
    /// Resource identity.
    pub id: ResourceId,
    /// Total RAM bytes.
    pub total_bytes: u64,
    /// Available RAM bytes.
    pub available_bytes: u64,
    /// NUMA node id, when scoped to a NUMA node.
    pub numa_node: Option<usize>,
}

impl MemoryResource {
    /// Creates a memory resource description.
    #[must_use]
    pub fn new(id: ResourceId, total_bytes: u64) -> Self {
        Self {
            id,
            total_bytes,
            available_bytes: total_bytes,
            numa_node: None,
        }
    }

    /// Sets available memory.
    #[must_use]
    pub fn with_available_bytes(mut self, available_bytes: u64) -> Self {
        self.available_bytes = available_bytes;
        self
    }

    /// Sets NUMA node.
    #[must_use]
    pub fn with_numa_node(mut self, numa_node: usize) -> Self {
        self.numa_node = Some(numa_node);
        self
    }
}
