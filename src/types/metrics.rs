//! Pipeline metrics and checkpoint metadata contracts.

use serde::{Deserialize, Serialize};

/// Metrics produced by one training step.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StepMetrics {
    /// Training step index.
    pub step: u64,
    /// Number of tokens processed by the step.
    pub tokens: u64,
    /// Number of samples represented in the step.
    pub samples: u64,
    /// Step wall time in microseconds.
    pub elapsed_micros: u64,
    /// Optional loss value reported by the backend.
    pub loss: Option<f64>,
}

impl StepMetrics {
    /// Creates metrics for a completed step.
    #[must_use]
    pub const fn new(step: u64, tokens: u64, samples: u64, elapsed_micros: u64) -> Self {
        Self {
            step,
            tokens,
            samples,
            elapsed_micros,
            loss: None,
        }
    }
}

/// Metadata describing a checkpoint artifact.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckpointMetadata {
    /// Training step captured by the checkpoint.
    pub step: u64,
    /// Stable checkpoint identifier.
    pub checkpoint_id: String,
    /// Backend that produced the checkpoint.
    pub backend: String,
    /// Optional content checksum.
    pub checksum: Option<String>,
}

impl CheckpointMetadata {
    /// Creates checkpoint metadata.
    #[must_use]
    pub fn new(step: u64, checkpoint_id: impl Into<String>, backend: impl Into<String>) -> Self {
        Self {
            step,
            checkpoint_id: checkpoint_id.into(),
            backend: backend.into(),
            checksum: None,
        }
    }
}
