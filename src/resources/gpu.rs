//! GPU resource contracts.

use serde::{Deserialize, Serialize};

use super::ResourceId;

/// GPU resource inventory and memory limits.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GpuResource {
    /// Resource identity.
    pub id: ResourceId,
    /// Device index visible to the backend.
    pub device_index: usize,
    /// Device name, when known.
    pub name: Option<String>,
    /// Total device memory in bytes.
    pub memory_total_bytes: u64,
    /// Available device memory in bytes.
    pub memory_available_bytes: u64,
    /// NUMA node id, when known.
    pub numa_node: Option<usize>,
}

impl GpuResource {
    /// Creates a GPU resource description.
    #[must_use]
    pub fn new(id: ResourceId, device_index: usize, memory_total_bytes: u64) -> Self {
        Self {
            id,
            device_index,
            name: None,
            memory_total_bytes,
            memory_available_bytes: memory_total_bytes,
            numa_node: None,
        }
    }

    /// Sets device name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets available GPU memory.
    #[must_use]
    pub fn with_available_memory(mut self, memory_available_bytes: u64) -> Self {
        self.memory_available_bytes = memory_available_bytes;
        self
    }

    /// Sets NUMA node.
    #[must_use]
    pub fn with_numa_node(mut self, numa_node: usize) -> Self {
        self.numa_node = Some(numa_node);
        self
    }
}
