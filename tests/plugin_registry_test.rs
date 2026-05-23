use std::sync::Arc;

use tunex_core::plugin::{
    PluginCapability, PluginId, PluginKind, PluginMetadata, PluginName, PluginRegistry,
    PluginStatus, PluginVersion, TunexPlugin,
};

#[derive(Clone)]
struct TestPlugin {
    metadata: PluginMetadata,
    capabilities: Vec<PluginCapability>,
    status: PluginStatus,
}

impl TestPlugin {
    fn new(id: &str, kind: PluginKind, capability: &str) -> Self {
        let capabilities = vec![PluginCapability::new(capability)];
        Self {
            metadata: PluginMetadata::new(
                PluginId::new(id),
                PluginName::new(id),
                PluginVersion::new(0, 1, 0),
                kind,
                capabilities.clone(),
            ),
            capabilities,
            status: PluginStatus::Active,
        }
    }
}

impl TunexPlugin for TestPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn capabilities(&self) -> &[PluginCapability] {
        &self.capabilities
    }

    fn health(&self) -> PluginStatus {
        self.status
    }
}

#[test]
fn registry_registers_discovers_and_validates_plugins() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Arc::new(TestPlugin::new(
            "tunex-dataset-mock",
            PluginKind::Dataset,
            "dataset:read",
        )))
        .expect("dataset plugin should register");
    registry
        .register(Arc::new(TestPlugin::new(
            "tunex-tokenizer-mock",
            PluginKind::Tokenizer,
            "tokenizer:encode",
        )))
        .expect("tokenizer plugin should register");

    assert_eq!(registry.lookup_by_kind(PluginKind::Dataset).len(), 1);
    assert_eq!(
        registry
            .lookup_by_capability(&PluginCapability::new("tokenizer:encode"))
            .len(),
        1
    );
    assert_eq!(registry.list_active().len(), 2);

    registry
        .validate_required_capabilities(&[
            (PluginKind::Dataset, PluginCapability::new("dataset:read")),
            (
                PluginKind::Tokenizer,
                PluginCapability::new("tokenizer:encode"),
            ),
        ])
        .expect("registered plugins should satisfy required capabilities");
}

#[test]
fn registry_rejects_duplicate_plugin_ids() {
    let mut registry = PluginRegistry::new();
    let plugin = Arc::new(TestPlugin::new(
        "tunex-dataset-mock",
        PluginKind::Dataset,
        "dataset:read",
    ));

    registry
        .register(plugin.clone())
        .expect("first registration should succeed");

    assert!(registry.register(plugin).is_err());
}
