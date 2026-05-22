//! Checkpoint and resume policy contracts.

use serde::{Deserialize, Serialize};

/// Checkpoint creation policy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CheckpointPolicy {
    /// Create checkpoint every N steps.
    EveryNSteps(u64),
    /// Create checkpoint every N seconds.
    EveryNSeconds(u64),
    /// Create checkpoint when best loss improves.
    OnBestLoss,
    /// Create checkpoint during shutdown.
    OnShutdown,
    /// Create checkpoint manually.
    Manual,
}

/// Resume behavior for training.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ResumePolicy {
    /// Do not resume.
    None,
    /// Resume from the latest available checkpoint.
    Latest,
    /// Resume from a specific checkpoint id.
    FromCheckpoint(super::CheckpointId),
    /// Resume only model weights.
    WeightsOnly,
    /// Resume full trainer/runtime state.
    FullState,
}
