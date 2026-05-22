//! Metrics module configuration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Metrics sink configuration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct MetricsConfig {
    /// Whether metrics are enabled.
    pub enabled: bool,
    /// Optional metrics plugin id or name.
    pub plugin: Option<String>,
    /// Static metrics namespace.
    pub namespace: String,
    /// Metrics exporters to enable.
    pub exporters: Vec<String>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin: None,
            namespace: "tunex".to_owned(),
            exporters: vec!["tracing".to_owned()],
        }
    }
}
