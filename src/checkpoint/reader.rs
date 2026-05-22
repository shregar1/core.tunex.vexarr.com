//! Checkpoint reader contract.

use async_trait::async_trait;

use super::{CheckpointId, CheckpointMetadata, CheckpointSnapshot, ResumePolicy};
use crate::TunexResult;

/// Reads checkpoints for resume.
#[async_trait]
pub trait CheckpointReader: Send + Sync {
    /// Reads checkpoint metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when metadata cannot be read.
    async fn read_metadata(&self, id: &CheckpointId) -> TunexResult<CheckpointMetadata>;

    /// Reads a checkpoint snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when the snapshot cannot be read.
    async fn read_snapshot(&self, id: &CheckpointId) -> TunexResult<CheckpointSnapshot>;

    /// Resolves a resume policy into checkpoint metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when resume metadata cannot be resolved.
    async fn resolve_resume(&self, policy: ResumePolicy)
        -> TunexResult<Option<CheckpointMetadata>>;
}
