//! Distributed worker placement contracts.

use serde::{Deserialize, Serialize};

use crate::{
    channel::QueuePressure,
    scheduler::{CpuAffinity, GpuAffinity, ResourceHint},
};

use super::{NodeId, NodeRole};

/// Strategy used to place work across nodes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementStrategy {
    /// Keep all work on one node.
    SingleNode,
    /// Prefer nodes local to the target GPU.
    GpuLocal,
    /// Prefer nodes local to the target CPU cores.
    CpuLocal,
    /// Prefer nodes with sufficient memory headroom.
    MemoryAware,
    /// Rotate placements evenly.
    RoundRobin,
    /// Prefer least loaded nodes.
    LeastLoaded,
    /// Custom placement strategy.
    Custom(String),
}

/// Placement decision for a logical worker.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WorkerPlacement {
    /// Logical worker identifier.
    pub worker_id: String,
    /// Node assigned to the worker.
    pub node_id: NodeId,
    /// Role required from the target node.
    pub role: NodeRole,
    /// Strategy that produced the placement.
    pub strategy: PlacementStrategy,
    /// Optional CPU affinity request.
    pub cpu_affinity: Option<CpuAffinity>,
    /// Optional GPU affinity request.
    pub gpu_affinity: Option<GpuAffinity>,
    /// Resource hint associated with the placement.
    pub resource_hint: ResourceHint,
    /// Queue pressure observed during placement.
    pub queue_pressure: QueuePressure,
}

impl WorkerPlacement {
    /// Creates a worker placement.
    #[must_use]
    pub fn new(
        worker_id: impl Into<String>,
        node_id: NodeId,
        role: NodeRole,
        strategy: PlacementStrategy,
    ) -> Self {
        Self {
            worker_id: worker_id.into(),
            node_id,
            role,
            strategy,
            cpu_affinity: None,
            gpu_affinity: None,
            resource_hint: ResourceHint::default(),
            queue_pressure: QueuePressure::Low,
        }
    }

    /// Sets queue pressure observed for this placement.
    #[must_use]
    pub const fn with_queue_pressure(mut self, queue_pressure: QueuePressure) -> Self {
        self.queue_pressure = queue_pressure;
        self
    }

    /// Sets resource hint.
    #[must_use]
    pub fn with_resource_hint(mut self, resource_hint: ResourceHint) -> Self {
        self.resource_hint = resource_hint;
        self
    }
}
