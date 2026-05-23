//! Resource inventory, limits, reservations, and usage contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

pub mod cpu;
pub mod disk;
pub mod gpu;
pub mod limits;
pub mod memory;
pub mod network;
pub mod usage;

pub use cpu::CpuResource;
pub use disk::DiskResource;
pub use gpu::GpuResource;
pub use limits::{OversubscriptionPolicy, ResourceLimit, ResourceReservation};
pub use memory::MemoryResource;
pub use network::NetworkResource;
pub use usage::{ResourceSnapshot, ResourceUsage};

/// Stable resource identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ResourceId(String);

impl ResourceId {
    /// Creates a resource id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the resource id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Kind of resource managed by `TuneX`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    /// CPU cores and affinity.
    Cpu,
    /// Accelerator device and memory.
    Gpu,
    /// System memory.
    Memory,
    /// Local or mounted disk.
    Disk,
    /// Network interface or link.
    Network,
    /// Custom resource.
    Custom,
}
