//! Plugin registry contracts and default in-memory registry.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use super::{PluginCapability, PluginError, PluginId, PluginKind, PluginMetadata, PluginStatus};
use crate::{TunexError, TunexResult};

/// Base contract implemented by all `TuneX` plugins.
pub trait TunexPlugin: Send + Sync {
    /// Returns plugin metadata.
    fn metadata(&self) -> PluginMetadata;

    /// Returns advertised plugin capabilities.
    fn capabilities(&self) -> &[PluginCapability];

    /// Returns plugin health status.
    fn health(&self) -> PluginStatus;
}

/// Registry for loaded plugins.
#[derive(Clone, Default)]
pub struct PluginRegistry {
    plugins: BTreeMap<PluginId, Arc<dyn TunexPlugin>>,
}

impl PluginRegistry {
    /// Creates an empty plugin registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
        }
    }

    /// Registers a plugin.
    ///
    /// # Errors
    ///
    /// Returns an error when another plugin with the same id already exists.
    pub fn register(&mut self, plugin: Arc<dyn TunexPlugin>) -> TunexResult<()> {
        let id = plugin.metadata().id;
        if self.plugins.contains_key(&id) {
            return Err(plugin_error(&PluginError::AlreadyRegistered(id)));
        }

        self.plugins.insert(id, plugin);
        Ok(())
    }

    /// Removes a plugin by id.
    ///
    /// # Errors
    ///
    /// Returns an error when the plugin id is not registered.
    pub fn remove(&mut self, id: &PluginId) -> TunexResult<Arc<dyn TunexPlugin>> {
        self.plugins
            .remove(id)
            .ok_or_else(|| plugin_error(&PluginError::NotFound(id.clone())))
    }

    /// Returns a plugin by id.
    #[must_use]
    pub fn get(&self, id: &PluginId) -> Option<&Arc<dyn TunexPlugin>> {
        self.plugins.get(id)
    }

    /// Returns all plugins matching a plugin kind.
    #[must_use]
    pub fn lookup_by_kind(&self, kind: PluginKind) -> Vec<Arc<dyn TunexPlugin>> {
        self.plugins
            .values()
            .filter(|plugin| plugin.metadata().kind == kind)
            .cloned()
            .collect()
    }

    /// Returns all plugins that advertise a capability.
    #[must_use]
    pub fn lookup_by_capability(&self, capability: &PluginCapability) -> Vec<Arc<dyn TunexPlugin>> {
        self.plugins
            .values()
            .filter(|plugin| plugin.capabilities().contains(capability))
            .cloned()
            .collect()
    }

    /// Returns active plugins.
    #[must_use]
    pub fn list_active(&self) -> Vec<Arc<dyn TunexPlugin>> {
        self.plugins
            .values()
            .filter(|plugin| plugin.health() == PluginStatus::Active)
            .cloned()
            .collect()
    }

    /// Checks whether a plugin id satisfies a required version.
    ///
    /// # Errors
    ///
    /// Returns an error when the plugin is missing or incompatible.
    pub fn check_version_compatibility(
        &self,
        id: &PluginId,
        required: &super::PluginVersion,
    ) -> TunexResult<()> {
        let plugin = self
            .plugins
            .get(id)
            .ok_or_else(|| plugin_error(&PluginError::NotFound(id.clone())))?;
        let actual = plugin.metadata().version;

        if actual.is_compatible_with(required) {
            return Ok(());
        }

        Err(plugin_error(&PluginError::IncompatibleVersion {
            plugin_id: id.clone(),
            required: required.clone(),
            actual,
        }))
    }

    /// Validates that required capabilities are provided by plugins of each kind.
    ///
    /// # Errors
    ///
    /// Returns an error when a required capability is missing.
    pub fn validate_required_capabilities(
        &self,
        required: &[(PluginKind, PluginCapability)],
    ) -> TunexResult<()> {
        let provided = self.provided_capabilities_by_kind();

        for (kind, capability) in required {
            let has_capability = provided
                .get(kind)
                .is_some_and(|capabilities| capabilities.contains(capability));

            if !has_capability {
                return Err(plugin_error(&PluginError::MissingCapability {
                    kind: *kind,
                    capability: capability.clone(),
                }));
            }
        }

        Ok(())
    }

    fn provided_capabilities_by_kind(&self) -> BTreeMap<PluginKind, BTreeSet<PluginCapability>> {
        let mut provided: BTreeMap<PluginKind, BTreeSet<PluginCapability>> = BTreeMap::new();

        for plugin in self.plugins.values() {
            let metadata = plugin.metadata();
            provided
                .entry(metadata.kind)
                .or_default()
                .extend(plugin.capabilities().iter().cloned());
        }

        provided
    }
}

fn plugin_error(error: &PluginError) -> TunexError {
    TunexError::Plugin(error.to_string())
}
