//! Runtime statistics contracts.

use serde::{Deserialize, Serialize};

/// Runtime statistics snapshot.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeStats {
    /// Total submitted tasks.
    pub submitted_tasks: u64,
    /// Total completed tasks.
    pub completed_tasks: u64,
    /// Total failed tasks.
    pub failed_tasks: u64,
    /// Total emitted events.
    pub emitted_events: u64,
    /// Number of active workers.
    pub active_workers: usize,
    /// Number of active channels.
    pub active_channels: usize,
    /// Number of active backpressure signals.
    pub backpressure_events: u64,
    /// Whether cancellation has been requested.
    pub cancellation_requested: bool,
}
