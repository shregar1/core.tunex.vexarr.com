//! Checkpoint lifecycle state contracts.

use serde::{Deserialize, Serialize};

/// Checkpoint lifecycle state.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CheckpointState {
    /// Checkpoint has been planned but not started.
    #[default]
    Pending,
    /// Checkpoint is being written.
    Writing,
    /// Checkpoint is being finalized.
    Finalizing,
    /// Checkpoint is available for resume.
    Available,
    /// Checkpoint failed.
    Failed,
    /// Checkpoint was deleted.
    Deleted,
}
