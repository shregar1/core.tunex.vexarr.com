//! Trainer backend configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Trainer backend configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct TrainerConfig {
    /// Whether trainer execution is enabled.
    pub enabled: bool,
    /// Backend name or plugin id.
    pub backend: String,
    /// Optional trainer plugin id or name.
    pub plugin: Option<String>,
    /// `LoRA` rank for fine-tuning adapters.
    pub lora_rank: Option<usize>,
    /// Per-step batch size.
    pub batch_size: usize,
}

impl Default for TrainerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: String::new(),
            plugin: None,
            lora_rank: None,
            batch_size: 1,
        }
    }
}
