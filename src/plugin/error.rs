//! Plugin-specific error contract.

use thiserror::Error;

use super::{PluginCapability, PluginId, PluginKind, PluginVersion};

/// Errors returned by plugin registry and metadata validation.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PluginError {
    /// Plugin id already exists in the registry.
    #[error("plugin already registered: {0}")]
    AlreadyRegistered(PluginId),
    /// Plugin id does not exist in the registry.
    #[error("plugin not found: {0}")]
    NotFound(PluginId),
    /// Plugin version string is invalid.
    #[error("invalid plugin version: {0}")]
    InvalidVersion(String),
    /// Plugin version is not compatible with a required version.
    #[error(
        "plugin {plugin_id} version {actual} is incompatible with required version {required}"
    )]
    IncompatibleVersion {
        /// Plugin id.
        plugin_id: PluginId,
        /// Required version.
        required: PluginVersion,
        /// Actual registered version.
        actual: PluginVersion,
    },
    /// Required capability is missing for a plugin kind.
    #[error("missing capability {capability} for plugin kind {kind:?}")]
    MissingCapability {
        /// Plugin kind that should provide the capability.
        kind: PluginKind,
        /// Missing capability.
        capability: PluginCapability,
    },
}
