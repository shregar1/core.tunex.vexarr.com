//! Trainer backend contract.

use async_trait::async_trait;

use crate::{
    types::{PackedBatch, StepMetrics},
    TunexResult,
};

/// Backend-agnostic trainer execution contract.
#[async_trait]
pub trait TrainerBackend: Send + Sync {
    /// Executes one training step.
    ///
    /// # Errors
    ///
    /// Returns an error when the backend cannot complete the step.
    async fn train_step(&self, batch: &PackedBatch) -> TunexResult<StepMetrics>;
}
