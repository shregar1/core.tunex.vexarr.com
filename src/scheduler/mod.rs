//! Scheduling primitives for future runtime implementations.

use serde::{Deserialize, Serialize};

/// Priority assigned to executable work.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TaskPriority {
    /// Background work.
    Low,
    /// Default priority.
    #[default]
    Normal,
    /// Latency-sensitive work.
    High,
    /// Runtime control or failure-handling work.
    Critical,
}

/// Priority assigned to a batch moving through the pipeline.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BatchPriority {
    /// Background batch.
    Low,
    /// Default batch priority.
    #[default]
    Normal,
    /// High-priority batch.
    High,
}

/// Placement hints for worker execution.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkerAffinity {
    /// Optional CPU worker or core index.
    pub cpu: Option<usize>,
    /// Optional GPU index.
    pub gpu: Option<usize>,
    /// Optional NUMA node index.
    pub numa: Option<usize>,
}

/// Scheduling policy selected by future runtime implementations.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum SchedulingPolicy {
    /// Optimize aggregate throughput.
    Throughput,
    /// Optimize end-to-end latency.
    Latency,
    /// Balance throughput and latency.
    #[default]
    Balanced,
    /// Prefer GPU-bound work.
    GpuPriority,
    /// Prefer CPU-bound work.
    CpuPriority,
    /// Account for memory pressure and placement.
    MemoryAware,
}
