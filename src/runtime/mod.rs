//! Runtime control-plane contracts.

use serde::{Deserialize, Serialize};

pub use crate::cancellation::{CancellationReason, CancellationToken, DrainPolicy, ShutdownMode};

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
