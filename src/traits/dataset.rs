//! Dataset source contract.

use async_trait::async_trait;

use crate::{types::RawSample, TunexResult};

/// Produces raw samples for the training pipeline.
#[async_trait]
pub trait DatasetSource: Send + Sync {
    /// Returns the next sample or `None` when the source is exhausted.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot read or decode the next sample.
    async fn next_sample(&self) -> TunexResult<Option<RawSample>>;
}
