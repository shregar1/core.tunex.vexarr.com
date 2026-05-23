use tunex_core::{
    api::{ApiVersion, CapabilitySet, CompatibilityLevel, CoreVersion, FeatureFlag, Stability},
    prelude::*,
};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn api_contract_types_are_send_sync() {
    assert_send_sync::<ApiVersion>();
    assert_send_sync::<CoreVersion>();
    assert_send_sync::<CompatibilityLevel>();
    assert_send_sync::<CapabilitySet>();
    assert_send_sync::<FeatureFlag>();
    assert_send_sync::<Stability>();
}

#[test]
fn api_versions_follow_semver_compatibility_rules() {
    let current = ApiVersion::new(0, 2, 1);
    let older_minor = ApiVersion::new(0, 1, 0);
    let newer_minor = ApiVersion::new(0, 3, 0);
    let different_major = ApiVersion::new(1, 0, 0);

    assert!(current.is_compatible_with(older_minor));
    assert!(!current.is_compatible_with(newer_minor));
    assert!(!current.is_compatible_with(different_major));
    assert_eq!(current.to_string(), "0.2.1");
}

#[test]
fn core_version_reports_current_crate_and_api_version() {
    let current = CoreVersion::current();

    assert_eq!(current.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(current.api_version, ApiVersion::new(0, 1, 0));
}

#[test]
fn capability_sets_support_feature_discovery_and_gating() {
    let stable = Stability::stable(ApiVersion::new(0, 1, 0));
    let capabilities = CapabilitySet::new("tunex-core", ApiVersion::new(0, 1, 0), stable)
        .with_feature(FeatureFlag::Tokio)
        .with_feature(FeatureFlag::Metrics)
        .with_feature(FeatureFlag::Metrics);

    assert!(capabilities.supports(&FeatureFlag::Tokio));
    assert!(capabilities.supports(&FeatureFlag::Metrics));
    assert_eq!(capabilities.features.len(), 2);
    assert!(capabilities.is_compatible_with(ApiVersion::new(0, 1, 0)));
    assert!(!capabilities.is_compatible_with(ApiVersion::new(0, 2, 0)));
}

#[test]
fn stability_tracks_experimental_and_deprecation_metadata() {
    let experimental = Stability::experimental(ApiVersion::new(0, 1, 0), "wasm")
        .deprecated(ApiVersion::new(0, 2, 0), "native-plugin");

    assert_eq!(experimental.level, CompatibilityLevel::Deprecated);
    assert_eq!(experimental.feature_gate.as_deref(), Some("wasm"));
    assert_eq!(
        experimental.deprecated_since,
        Some(ApiVersion::new(0, 2, 0))
    );
    assert_eq!(experimental.replacement.as_deref(), Some("native-plugin"));
    assert!(!experimental.is_public_stable());
}

#[test]
fn prelude_exposes_stable_public_surface() {
    let _error: Option<TunexError> = None;
    let _result: TunexResult<()> = Ok(());
    let _sample = RawSample::default();
    let _config = TunexConfig::default();
    let _plugin = PluginCapability::new("dataset:read");
    let _graph = PipelineGraph::new();
    let _runtime = RuntimeState::Created;
    let _event = EventKind::JobStarted;

    fn assert_dataset_source_object_safe<T: ?Sized>() {}
    assert_dataset_source_object_safe::<dyn DatasetSource>();
}
