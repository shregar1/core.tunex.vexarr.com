//! Scheduling primitives for future runtime implementations.

use serde::{Deserialize, Serialize};

/// Stable task identifier.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TaskId(u64);

impl TaskId {
    /// Creates a task id.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw task id.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Runtime task kind.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TaskKind {
    /// Dataset task.
    Dataset,
    /// Tokenizer task.
    Tokenizer,
    /// Packer task.
    Packer,
    /// Trainer task.
    Trainer,
    /// Checkpoint task.
    Checkpoint,
    /// Metrics task.
    Metrics,
    /// Custom task.
    Custom(String),
}

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

/// CPU placement hint.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CpuAffinity {
    /// Preferred CPU cores.
    pub cores: Vec<usize>,
}

/// GPU placement hint.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GpuAffinity {
    /// Preferred GPU indices.
    pub devices: Vec<usize>,
}

/// NUMA node hint.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NumaNode(pub usize);

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

/// Resource usage hint for scheduled work.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceHint {
    /// CPU affinity.
    pub cpu: Option<CpuAffinity>,
    /// GPU affinity.
    pub gpu: Option<GpuAffinity>,
    /// NUMA node.
    pub numa: Option<NumaNode>,
    /// Estimated memory bytes.
    pub memory_bytes: Option<usize>,
}

/// Resource limits for scheduled work.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLimit {
    /// Maximum memory bytes.
    pub max_memory_bytes: Option<usize>,
    /// Maximum runtime milliseconds.
    pub timeout_millis: Option<u64>,
}

/// Scheduling policy selected by future runtime implementations.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum SchedulingPolicy {
    /// Optimize aggregate throughput.
    Throughput,
    /// Optimize end-to-end latency.
    Latency,
    /// Optimize end-to-end latency.
    LowLatency,
    /// Balance throughput and latency.
    #[default]
    Balanced,
    /// Prefer GPU-bound work.
    GpuPriority,
    /// Prefer CPU-bound work.
    CpuPriority,
    /// Account for memory pressure and placement.
    MemoryAware,
    /// Custom scheduler policy.
    Custom,
}

/// Scheduling decision produced by a scheduler implementation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SchedulingDecision {
    /// Selected worker id when available.
    pub worker_id: Option<String>,
    /// Selected priority.
    pub priority: TaskPriority,
    /// Selected affinity.
    pub affinity: WorkerAffinity,
    /// Resource limit.
    pub resource_limit: ResourceLimit,
    /// Whether cancellation was observed while deciding.
    pub cancellation_observed: bool,
}

impl Default for SchedulingDecision {
    fn default() -> Self {
        Self {
            worker_id: None,
            priority: TaskPriority::Normal,
            affinity: WorkerAffinity::default(),
            resource_limit: ResourceLimit::default(),
            cancellation_observed: false,
        }
    }
}
