//! Worker and pipeline state contracts.

use serde::{Deserialize, Serialize};

/// Lifecycle state for a pipeline worker.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum WorkerState {
    /// Worker has been created but has not started processing.
    #[default]
    Initialized,
    /// Worker is ready to accept work.
    Ready,
    /// Worker is actively processing work.
    Running,
    /// Worker is blocked by downstream backpressure.
    Backpressured,
    /// Worker is draining in-flight work before shutdown.
    Draining,
    /// Worker has stopped successfully.
    Stopped,
    /// Worker failed and requires external handling.
    Failed,
}

/// Event emitted by pipeline orchestration components.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PipelineEvent {
    /// Worker state changed.
    WorkerStateChanged {
        /// Stable worker identifier.
        worker_id: String,
        /// New worker state.
        state: WorkerState,
    },
    /// Backpressure was observed at a named pipeline stage.
    Backpressure {
        /// Pipeline stage name.
        stage: String,
        /// Queue depth at the time of observation.
        queue_depth: usize,
    },
    /// A checkpoint was completed.
    CheckpointCompleted {
        /// Stable checkpoint identifier.
        checkpoint_id: String,
        /// Training step captured by the checkpoint.
        step: u64,
    },
    /// A recoverable or fatal error occurred.
    Error {
        /// Component reporting the error.
        component: String,
        /// Error category.
        category: String,
        /// Human-readable message.
        message: String,
    },
}
