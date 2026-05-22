//! Serializable configuration schema for core contracts.

use serde::{Deserialize, Serialize};

/// Top-level v0.1 core configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CoreConfig {
    /// Pipeline-facing settings.
    pub pipeline: PipelineConfig,
    /// Metrics sink settings.
    pub metrics: MetricsConfig,
    /// Checkpoint settings.
    pub checkpoint: CheckpointConfig,
}

/// Pipeline configuration shared by contract implementations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineConfig {
    /// Maximum token count per packed batch.
    pub max_tokens_per_batch: usize,
    /// Maximum samples consumed before building a packed batch.
    pub max_samples_per_batch: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_tokens_per_batch: 4096,
            max_samples_per_batch: 64,
        }
    }
}

/// Metrics configuration shared by sinks.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MetricsConfig {
    /// Whether metric emission is enabled.
    pub enabled: bool,
    /// Optional static namespace for emitted metrics.
    pub namespace: Option<String>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            namespace: Some("tunex".to_owned()),
        }
    }
}

/// Checkpoint configuration shared by writers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckpointConfig {
    /// Whether checkpoint metadata should be written.
    pub enabled: bool,
    /// Logical checkpoint destination.
    pub destination: String,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            destination: "checkpoints".to_owned(),
        }
    }
}
