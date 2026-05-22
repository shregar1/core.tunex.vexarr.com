//! Runtime orchestration configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Worker count configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct WorkerConfig {
    /// Dataset worker count.
    pub dataset: usize,
    /// Tokenizer worker count.
    pub tokenizer: usize,
    /// Packer worker count.
    pub packer: usize,
    /// Trainer worker count.
    pub trainer: usize,
    /// Checkpoint worker count.
    pub checkpoint: usize,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            dataset: 1,
            tokenizer: 1,
            packer: 1,
            trainer: 1,
            checkpoint: 1,
        }
    }
}

/// Bounded queue size configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct QueueConfig {
    /// Dataset to tokenizer queue capacity.
    pub dataset_to_tokenizer: usize,
    /// Tokenizer to packer queue capacity.
    pub tokenizer_to_packer: usize,
    /// Packer to trainer queue capacity.
    pub packer_to_trainer: usize,
    /// Trainer to checkpoint queue capacity.
    pub trainer_to_checkpoint: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            dataset_to_tokenizer: 1024,
            tokenizer_to_packer: 512,
            packer_to_trainer: 128,
            trainer_to_checkpoint: 16,
        }
    }
}

/// Runtime configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct RuntimeConfig {
    /// Whether runtime orchestration is enabled.
    pub enabled: bool,
    /// Worker counts.
    pub workers: WorkerConfig,
    /// Bounded queue capacities.
    pub queues: QueueConfig,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workers: WorkerConfig::default(),
            queues: QueueConfig::default(),
        }
    }
}
