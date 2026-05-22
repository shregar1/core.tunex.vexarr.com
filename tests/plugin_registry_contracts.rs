use std::{str::FromStr, sync::Arc};

use tunex_core::{
    plugin::{
        PluginCapability, PluginId, PluginKind, PluginMetadata, PluginName, PluginRegistry,
        PluginStatus, PluginVersion, TunexPlugin,
    },
    TunexError,
};

#[derive(Clone)]
struct TestPlugin {
    metadata: PluginMetadata,
    status: PluginStatus,
}

impl TestPlugin {
    fn new(
        id: &str,
        kind: PluginKind,
        version: PluginVersion,
        capabilities: Vec<PluginCapability>,
        status: PluginStatus,
    ) -> Self {
        Self {
            metadata: PluginMetadata::new(
                PluginId::new(id),
                PluginName::new(id),
                version,
                kind,
                capabilities,
            ),
            status,
        }
    }
}

impl TunexPlugin for TestPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn capabilities(&self) -> &[PluginCapability] {
        &self.metadata.capabilities
    }

    fn health(&self) -> PluginStatus {
        self.status
    }
}

#[test]
fn plugin_version_parses_and_checks_compatibility() {
    let actual = PluginVersion::from_str("1.4.2").unwrap();

    assert_eq!(actual.to_string(), "1.4.2");
    assert!(actual.is_compatible_with(&PluginVersion::new(1, 3, 0)));
    assert!(!actual.is_compatible_with(&PluginVersion::new(2, 0, 0)));
    assert!(PluginVersion::from_str("1.2").is_err());
}

#[test]
fn registry_registers_removes_and_rejects_duplicates() {
    let plugin = Arc::new(TestPlugin::new(
        "dataset.local",
        PluginKind::Dataset,
        PluginVersion::new(0, 1, 0),
        vec![PluginCapability::new("dataset.read")],
        PluginStatus::Active,
    ));
    let mut registry = PluginRegistry::new();

    registry.register(plugin.clone()).unwrap();
    assert!(registry.register(plugin).is_err());

    let removed = registry.remove(&PluginId::new("dataset.local")).unwrap();
    assert_eq!(removed.metadata().id.as_str(), "dataset.local");
    assert!(registry.remove(&PluginId::new("dataset.local")).is_err());
}

#[test]
fn registry_looks_up_by_kind_capability_and_status() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Arc::new(TestPlugin::new(
            "tokenizer.bpe",
            PluginKind::Tokenizer,
            PluginVersion::new(1, 0, 0),
            vec![PluginCapability::new("tokenize.bpe")],
            PluginStatus::Active,
        )))
        .unwrap();
    registry
        .register(Arc::new(TestPlugin::new(
            "metrics.prom",
            PluginKind::Metrics,
            PluginVersion::new(1, 0, 0),
            vec![PluginCapability::new("metrics.prometheus")],
            PluginStatus::Degraded,
        )))
        .unwrap();

    assert_eq!(registry.lookup_by_kind(PluginKind::Tokenizer).len(), 1);
    assert_eq!(
        registry
            .lookup_by_capability(&PluginCapability::new("tokenize.bpe"))
            .len(),
        1
    );
    assert_eq!(registry.list_active().len(), 1);
}

#[test]
fn registry_checks_versions_and_required_capabilities() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Arc::new(TestPlugin::new(
            "trainer.local",
            PluginKind::TrainerBackend,
            PluginVersion::new(1, 2, 0),
            vec![PluginCapability::new("train.step")],
            PluginStatus::Active,
        )))
        .unwrap();

    registry
        .check_version_compatibility(
            &PluginId::new("trainer.local"),
            &PluginVersion::new(1, 1, 0),
        )
        .unwrap();
    assert!(registry
        .check_version_compatibility(
            &PluginId::new("trainer.local"),
            &PluginVersion::new(2, 0, 0)
        )
        .is_err());

    registry
        .validate_required_capabilities(&[(
            PluginKind::TrainerBackend,
            PluginCapability::new("train.step"),
        )])
        .unwrap();
    let missing = registry
        .validate_required_capabilities(&[(
            PluginKind::TrainerBackend,
            PluginCapability::new("train.distributed"),
        )])
        .unwrap_err();

    assert!(matches!(missing, TunexError::Plugin(_)));
}
