//! Resource usage snapshot contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::events::EventTimestamp;

use super::{
    CpuResource, DiskResource, GpuResource, MemoryResource, NetworkResource, ResourceId,
    ResourceKind, ResourceReservation,
};

/// Point-in-time usage for one resource.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceUsage {
    /// Resource identity.
    pub resource_id: ResourceId,
    /// Resource kind.
    pub kind: ResourceKind,
    /// Used logical units, such as CPU cores.
    pub used_units: Option<u64>,
    /// Used bytes, such as RAM, VRAM, or disk space.
    pub used_bytes: Option<u64>,
    /// Used bandwidth in bytes per second.
    pub used_bandwidth_bytes_per_sec: Option<u64>,
    /// Utilization in basis points from 0 to 10,000.
    pub utilization_basis_points: Option<u16>,
}

impl ResourceUsage {
    /// Creates a usage sample.
    #[must_use]
    pub fn new(resource_id: ResourceId, kind: ResourceKind) -> Self {
        Self {
            resource_id,
            kind,
            used_units: None,
            used_bytes: None,
            used_bandwidth_bytes_per_sec: None,
            utilization_basis_points: None,
        }
    }

    /// Sets used units.
    #[must_use]
    pub fn with_used_units(mut self, used_units: u64) -> Self {
        self.used_units = Some(used_units);
        self
    }

    /// Sets used bytes.
    #[must_use]
    pub fn with_used_bytes(mut self, used_bytes: u64) -> Self {
        self.used_bytes = Some(used_bytes);
        self
    }

    /// Sets utilization basis points, clamped to 100%.
    #[must_use]
    pub fn with_utilization_basis_points(mut self, basis_points: u16) -> Self {
        self.utilization_basis_points = Some(if basis_points > 10_000 {
            10_000
        } else {
            basis_points
        });
        self
    }
}

/// Point-in-time resource inventory and usage snapshot.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceSnapshot {
    /// Snapshot timestamp.
    pub timestamp: EventTimestamp,
    /// CPU resources.
    pub cpus: SmallVec<[CpuResource; 4]>,
    /// GPU resources.
    pub gpus: SmallVec<[GpuResource; 4]>,
    /// Memory resources.
    pub memory: SmallVec<[MemoryResource; 4]>,
    /// Disk resources.
    pub disks: SmallVec<[DiskResource; 4]>,
    /// Network resources.
    pub networks: SmallVec<[NetworkResource; 4]>,
    /// Usage samples.
    pub usage: SmallVec<[ResourceUsage; 16]>,
    /// Active reservations.
    pub reservations: SmallVec<[ResourceReservation; 16]>,
}

impl Default for ResourceSnapshot {
    fn default() -> Self {
        Self {
            timestamp: EventTimestamp::now(),
            cpus: SmallVec::new(),
            gpus: SmallVec::new(),
            memory: SmallVec::new(),
            disks: SmallVec::new(),
            networks: SmallVec::new(),
            usage: SmallVec::new(),
            reservations: SmallVec::new(),
        }
    }
}

impl ResourceSnapshot {
    /// Creates an empty resource snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a CPU resource.
    #[must_use]
    pub fn with_cpu(mut self, cpu: CpuResource) -> Self {
        self.cpus.push(cpu);
        self
    }

    /// Adds a GPU resource.
    #[must_use]
    pub fn with_gpu(mut self, gpu: GpuResource) -> Self {
        self.gpus.push(gpu);
        self
    }

    /// Adds a memory resource.
    #[must_use]
    pub fn with_memory(mut self, memory: MemoryResource) -> Self {
        self.memory.push(memory);
        self
    }

    /// Adds a disk resource.
    #[must_use]
    pub fn with_disk(mut self, disk: DiskResource) -> Self {
        self.disks.push(disk);
        self
    }

    /// Adds a network resource.
    #[must_use]
    pub fn with_network(mut self, network: NetworkResource) -> Self {
        self.networks.push(network);
        self
    }

    /// Adds a usage sample.
    #[must_use]
    pub fn with_usage(mut self, usage: ResourceUsage) -> Self {
        self.usage.push(usage);
        self
    }

    /// Adds a resource reservation.
    #[must_use]
    pub fn with_reservation(mut self, reservation: ResourceReservation) -> Self {
        self.reservations.push(reservation);
        self
    }

    /// Returns aggregate available RAM bytes.
    #[must_use]
    pub fn available_ram_bytes(&self) -> u64 {
        self.memory
            .iter()
            .map(|resource| resource.available_bytes)
            .sum()
    }

    /// Returns aggregate available GPU memory bytes.
    #[must_use]
    pub fn available_gpu_memory_bytes(&self) -> u64 {
        self.gpus
            .iter()
            .map(|resource| resource.memory_available_bytes)
            .sum()
    }
}
