//! Runtime state and mode contracts.

use serde::{Deserialize, Serialize};

/// Runtime lifecycle state.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeState {
    /// Runtime has been created.
    #[default]
    Created,
    /// Runtime is initializing.
    Initializing,
    /// Runtime is running.
    Running,
    /// Runtime is paused.
    Paused,
    /// Runtime is draining in-flight work.
    Draining,
    /// Runtime is stopping.
    Stopping,
    /// Runtime has stopped.
    Stopped,
    /// Runtime has failed.
    Failed,
}

impl RuntimeState {
    /// Returns true when a transition from `self` to `next` is valid.
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        match self {
            Self::Created => matches!(next, Self::Initializing | Self::Stopped | Self::Failed),
            Self::Initializing | Self::Paused => {
                matches!(next, Self::Running | Self::Failed | Self::Stopping)
            }
            Self::Running => matches!(
                next,
                Self::Paused | Self::Draining | Self::Stopping | Self::Failed
            ),
            Self::Draining => matches!(next, Self::Stopping | Self::Stopped | Self::Failed),
            Self::Stopping => matches!(next, Self::Stopped | Self::Failed),
            Self::Stopped | Self::Failed => false,
        }
    }
}

/// Runtime execution mode.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMode {
    /// Single-threaded runtime.
    SingleThreaded,
    /// Multi-threaded runtime.
    #[default]
    MultiThreaded,
    /// GPU-bound runtime.
    GpuBound,
    /// CPU-bound runtime.
    CpuBound,
    /// Distributed runtime.
    Distributed,
    /// Debug runtime.
    Debug,
    /// Benchmark runtime.
    Benchmark,
}

/// Signal sent to a worker by orchestration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WorkerSignal {
    /// Start or resume work.
    Start,
    /// Pause work without losing state.
    Pause,
    /// Drain in-flight work and stop.
    Drain,
    /// Stop as soon as possible.
    Stop,
    /// Restart the worker.
    Restart,
}

/// Reason a runtime or worker is shutting down.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ShutdownReason {
    /// Normal graceful shutdown.
    Graceful,
    /// User requested cancellation.
    Cancelled,
    /// A pipeline component failed.
    ComponentFailed(String),
    /// Memory pressure or out-of-memory condition.
    OutOfMemory,
    /// Worker restart requested.
    WorkerRestart,
}

/// Logical runtime layer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RuntimeLayer {
    /// Control plane coordination.
    ControlPlane,
    /// Pipeline orchestration runtime.
    PipelineRuntime,
    /// Worker pool execution.
    WorkerPool,
    /// Execution node.
    ExecutionNode,
}
