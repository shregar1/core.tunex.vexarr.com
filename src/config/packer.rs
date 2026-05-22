//! Sequence packer module configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Sequence packing strategy.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PackingStrategy {
    /// Preserve one sequence per packed record.
    None,
    /// First-fit packing.
    FirstFit,
    /// Best-fit packing.
    #[default]
    BestFit,
}

/// Sequence packer configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct PackerConfig {
    /// Whether the packer module is enabled.
    pub enabled: bool,
    /// Optional selected packer plugin id or name.
    pub plugin: Option<String>,
    /// Maximum sequence length.
    pub max_seq_len: usize,
    /// Packing strategy.
    pub packing_strategy: PackingStrategy,
}

impl Default for PackerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: None,
            max_seq_len: 4096,
            packing_strategy: PackingStrategy::BestFit,
        }
    }
}
