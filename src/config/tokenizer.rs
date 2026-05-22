//! Tokenizer module configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Tokenizer configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct TokenizerConfig {
    /// Whether the tokenizer module is enabled.
    pub enabled: bool,
    /// Selected tokenizer plugin id or name.
    pub plugin: String,
    /// Tokenizer model identifier or path.
    pub model: String,
    /// Optional truncation length.
    pub max_length: Option<usize>,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: String::new(),
            model: String::new(),
            max_length: None,
        }
    }
}
