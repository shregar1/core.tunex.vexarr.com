//! Checkpoint writer contract.

use async_trait::async_trait;

use crate::{types::CheckpointMetadata, TunexResult};

/// Persists checkpoint metadata and backend-owned checkpoint artifacts.
#[async_trait]
pub trait CheckpointWriter: Send + Sync {
    /// Writes checkpoint metadata for an already-materialized checkpoint artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when checkpoint metadata cannot be persisted.
    async fn write_checkpoint(&self, metadata: &CheckpointMetadata) -> TunexResult<()>;
}
