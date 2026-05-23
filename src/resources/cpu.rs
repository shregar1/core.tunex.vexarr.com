//! CPU resource contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::ResourceId;

/// CPU resource inventory and placement hints.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CpuResource {
    /// Resource identity.
    pub id: ResourceId,
    /// Number of logical cores available.
    pub logical_cores: usize,
    /// Number of physical cores, when known.
    pub physical_cores: Option<usize>,
    /// Preferred CPU core ids.
    pub affinity_cores: SmallVec<[usize; 16]>,
    /// NUMA node id, when known.
    pub numa_node: Option<usize>,
}

impl CpuResource {
    /// Creates a CPU resource description.
    #[must_use]
    pub fn new(id: ResourceId, logical_cores: usize) -> Self {
        Self {
            id,
            logical_cores,
            physical_cores: None,
            affinity_cores: SmallVec::new(),
            numa_node: None,
        }
    }

    /// Sets physical core count.
    #[must_use]
    pub fn with_physical_cores(mut self, physical_cores: usize) -> Self {
        self.physical_cores = Some(physical_cores);
        self
    }

    /// Sets preferred CPU affinity cores.
    #[must_use]
    pub fn with_affinity_cores(mut self, cores: impl IntoIterator<Item = usize>) -> Self {
        self.affinity_cores = cores.into_iter().collect();
        self
    }

    /// Sets NUMA node.
    #[must_use]
    pub fn with_numa_node(mut self, numa_node: usize) -> Self {
        self.numa_node = Some(numa_node);
        self
    }
}
