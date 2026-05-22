//! Dataset module configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Dataset source configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct DatasetConfig {
    /// Whether the dataset module is enabled.
    pub enabled: bool,
    /// Selected dataset plugin id or name.
    pub plugin: String,
    /// Input dataset path or URI.
    pub path: String,
    /// Optional dataset split.
    pub split: Option<String>,
}

impl Default for DatasetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: String::new(),
            path: String::new(),
            split: None,
        }
    }
}
