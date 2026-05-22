//! Runtime control-plane contracts.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use serde::{Deserialize, Serialize};

/// Shared cancellation token for coordinated shutdown.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a new cancellation token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns true when cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
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
