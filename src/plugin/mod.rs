//! Plugin metadata, capability, loading, and registry contracts.

pub mod capability;
pub mod error;
pub mod metadata;
pub mod registry;
pub mod version;

pub use capability::PluginCapability;
pub use error::PluginError;
pub use metadata::{PluginId, PluginKind, PluginMetadata, PluginName, PluginStatus};
pub use registry::{PluginRegistry, TunexPlugin};
pub use version::PluginVersion;

use async_trait::async_trait;

use crate::TunexResult;

/// Loaded plugin handle returned by plugin loaders.
pub trait PluginHandle: Send + Sync {
    /// Returns immutable plugin metadata.
    fn metadata(&self) -> &PluginMetadata;
}

/// Plugin loader abstraction for future native, WASM, or remote loaders.
#[async_trait]
pub trait PluginLoader: Send + Sync {
    /// Loads a plugin from an implementation-defined location.
    ///
    /// # Errors
    ///
    /// Returns an error when the plugin cannot be loaded or validated.
    async fn load(&self, location: &str) -> TunexResult<Box<dyn PluginHandle>>;
}
