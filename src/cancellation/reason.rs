//! Cancellation and shutdown reason contracts.

use serde::{Deserialize, Serialize};

/// Reason cancellation was requested.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CancellationReason {
    /// Entire job was cancelled.
    JobCancelled {
        /// Job identifier.
        job_id: String,
    },
    /// A worker was cancelled.
    WorkerCancelled {
        /// Worker identifier.
        worker_id: String,
    },
    /// A stage was cancelled.
    StageCancelled {
        /// Stage name.
        stage: String,
    },
    /// Shutdown was requested.
    Shutdown,
    /// Operation exceeded a timeout.
    Timeout,
    /// Failure triggered cancellation.
    Failure {
        /// Human-readable failure reason.
        reason: String,
    },
    /// Parent scope cancelled this scope.
    Cascaded {
        /// Parent scope identifier.
        parent_scope: String,
    },
}

/// Shutdown behavior requested by orchestration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ShutdownMode {
    /// Stop as soon as possible.
    Immediate,
    /// Stop gracefully.
    Graceful,
    /// Finish the current batch before stopping.
    DrainCurrentBatch,
    /// Save a checkpoint before stopping.
    CheckpointThenStop,
    /// Restart the worker after shutdown.
    RestartWorker,
}

/// Policy for draining in-flight work.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrainPolicy {
    /// Shutdown mode.
    pub mode: ShutdownMode,
    /// Maximum drain timeout in milliseconds.
    pub timeout_millis: Option<u64>,
    /// Whether checkpoint should be attempted before exit.
    pub checkpoint_before_exit: bool,
    /// Whether in-flight batches should be drained.
    pub drain_in_flight: bool,
}

impl DrainPolicy {
    /// Creates a drain policy from a shutdown mode.
    #[must_use]
    pub const fn new(mode: ShutdownMode) -> Self {
        let checkpoint_before_exit = matches!(mode, ShutdownMode::CheckpointThenStop);
        let drain_in_flight = matches!(
            mode,
            ShutdownMode::Graceful
                | ShutdownMode::DrainCurrentBatch
                | ShutdownMode::CheckpointThenStop
                | ShutdownMode::RestartWorker
        );
        Self {
            mode,
            timeout_millis: None,
            checkpoint_before_exit,
            drain_in_flight,
        }
    }

    /// Adds a timeout in milliseconds.
    #[must_use]
    pub const fn with_timeout_millis(mut self, timeout_millis: u64) -> Self {
        self.timeout_millis = Some(timeout_millis);
        self
    }
}

impl Default for DrainPolicy {
    fn default() -> Self {
        Self::new(ShutdownMode::Graceful)
    }
}
