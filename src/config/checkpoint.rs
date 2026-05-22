//! Checkpoint module configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Checkpoint writer configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct CheckpointConfig {
    /// Whether checkpointing is enabled.
    pub enabled: bool,
    /// Optional checkpoint plugin id or name.
    pub plugin: Option<String>,
    /// Checkpoint destination path or URI.
    pub destination: String,
    /// Number of training steps between checkpoints.
    pub interval_steps: usize,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: None,
            destination: "checkpoints".to_owned(),
            interval_steps: 100,
        }
    }
}
