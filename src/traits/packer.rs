//! Sequence packer contract.

use async_trait::async_trait;

use crate::{
    types::{PackedBatch, TokenizedSample},
    TunexResult,
};

/// Packs tokenized samples into backend-ready batches.
#[async_trait]
pub trait SequencePacker: Send + Sync {
    /// Packs tokenized samples into one batch.
    ///
    /// # Errors
    ///
    /// Returns an error when the samples cannot be packed under the active policy.
    async fn pack(&self, samples: &[TokenizedSample]) -> TunexResult<PackedBatch>;
}
