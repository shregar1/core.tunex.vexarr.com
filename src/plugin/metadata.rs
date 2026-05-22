//! Plugin identity and metadata contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::{PluginCapability, PluginVersion};

/// Stable plugin identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PluginId(String);

impl PluginId {
    /// Creates a plugin id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the plugin id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PluginId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Human-readable plugin name.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PluginName(String);

impl PluginName {
    /// Creates a plugin name.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the plugin name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PluginName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Plugin family.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PluginKind {
    /// Dataset source plugin.
    Dataset,
    /// Tokenizer plugin.
    Tokenizer,
    /// Sequence packer plugin.
    Packer,
    /// Trainer backend plugin.
    TrainerBackend,
    /// Checkpoint writer plugin.
    Checkpoint,
    /// Metrics sink plugin.
    Metrics,
    /// Scheduler plugin.
    Scheduler,
    /// Distributed coordination plugin.
    Distributed,
    /// Runtime orchestration plugin.
    Runtime,
}

/// Plugin health status.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PluginStatus {
    /// Plugin is registered but not ready.
    Registered,
    /// Plugin is ready for use.
    Active,
    /// Plugin is temporarily unavailable.
    Degraded,
    /// Plugin failed health checks.
    Failed,
    /// Plugin has been removed or disabled.
    Disabled,
}

/// Static plugin metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PluginMetadata {
    /// Stable plugin identifier.
    pub id: PluginId,
    /// Human-readable plugin name.
    pub name: PluginName,
    /// Plugin version.
    pub version: PluginVersion,
    /// Plugin family.
    pub kind: PluginKind,
    /// Declared plugin capabilities.
    pub capabilities: Vec<PluginCapability>,
}

impl PluginMetadata {
    /// Creates plugin metadata.
    #[must_use]
    pub fn new(
        id: PluginId,
        name: PluginName,
        version: PluginVersion,
        kind: PluginKind,
        capabilities: Vec<PluginCapability>,
    ) -> Self {
        Self {
            id,
            name,
            version,
            kind,
            capabilities,
        }
    }
}
