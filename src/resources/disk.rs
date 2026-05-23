//! Disk resource contracts.

use serde::{Deserialize, Serialize};

use super::ResourceId;

/// Disk resource inventory and capacity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiskResource {
    /// Resource identity.
    pub id: ResourceId,
    /// Mount path or storage namespace.
    pub mount_path: String,
    /// Total disk bytes.
    pub total_bytes: u64,
    /// Available disk bytes.
    pub available_bytes: u64,
    /// Optional read bandwidth in bytes per second.
    pub read_bytes_per_sec: Option<u64>,
    /// Optional write bandwidth in bytes per second.
    pub write_bytes_per_sec: Option<u64>,
}

impl DiskResource {
    /// Creates a disk resource description.
    #[must_use]
    pub fn new(id: ResourceId, mount_path: impl Into<String>, total_bytes: u64) -> Self {
        Self {
            id,
            mount_path: mount_path.into(),
            total_bytes,
            available_bytes: total_bytes,
            read_bytes_per_sec: None,
            write_bytes_per_sec: None,
        }
    }

    /// Sets available bytes.
    #[must_use]
    pub fn with_available_bytes(mut self, available_bytes: u64) -> Self {
        self.available_bytes = available_bytes;
        self
    }

    /// Sets bandwidth hints.
    #[must_use]
    pub fn with_bandwidth(mut self, read_bytes_per_sec: u64, write_bytes_per_sec: u64) -> Self {
        self.read_bytes_per_sec = Some(read_bytes_per_sec);
        self.write_bytes_per_sec = Some(write_bytes_per_sec);
        self
    }
}
