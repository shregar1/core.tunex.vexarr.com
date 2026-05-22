//! Distributed coordination configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Distributed coordination configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct DistributedConfig {
    /// Whether distributed coordination is enabled.
    pub enabled: bool,
    /// Optional distributed coordinator plugin id or name.
    pub plugin: Option<String>,
    /// Coordinator endpoint when applicable.
    pub endpoint: Option<String>,
}
