use std::sync::Arc;

use tunex_core::prelude::*;

struct FakePlugin {
    metadata: PluginMetadata,
    capabilities: Vec<PluginCapability>,
}

impl TunexPlugin for FakePlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn capabilities(&self) -> &[PluginCapability] {
        &self.capabilities
    }

    fn health(&self) -> PluginStatus {
        PluginStatus::Active
    }
}

fn main() -> TunexResult<()> {
    let capability = PluginCapability::new("dataset:read");
    let plugin = Arc::new(FakePlugin {
        metadata: PluginMetadata::new(
            PluginId::new("tunex-dataset-fake"),
            PluginName::new("Fake Dataset"),
            PluginVersion::new(0, 1, 0),
            PluginKind::Dataset,
            vec![capability.clone()],
        ),
        capabilities: vec![capability.clone()],
    });

    let mut registry = PluginRegistry::new();
    registry.register(plugin)?;
    registry.validate_required_capabilities(&[(PluginKind::Dataset, capability)])?;

    println!(
        "registered {} active plugin(s)",
        registry.list_active().len()
    );
    Ok(())
}
