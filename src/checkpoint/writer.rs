//! Checkpoint writer contract.

use async_trait::async_trait;

use super::{CheckpointMetadata, CheckpointSnapshot};
use crate::TunexResult;

/// Persists checkpoints.
#[async_trait]
pub trait CheckpointWriter: Send + Sync {
    /// Writes a complete snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when the checkpoint cannot be written.
    async fn write_snapshot(&self, snapshot: CheckpointSnapshot)
        -> TunexResult<CheckpointMetadata>;

    /// Writes only checkpoint metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when metadata cannot be written.
    async fn write_metadata(&self, metadata: CheckpointMetadata) -> TunexResult<()>;
}
