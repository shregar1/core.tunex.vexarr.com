# Plugin System

The TuneX plugin layer lets independently developed modules register into the
core ecosystem through stable metadata and capability contracts.

## Plugin Metadata

Plugins declare:

- `PluginId`
- `PluginName`
- `PluginVersion`
- `PluginKind`
- `PluginStatus`
- `PluginCapability`

Plugin kinds include dataset, tokenizer, packer, trainer backend, checkpoint,
metrics, scheduler, distributed, and runtime.

## Registry Contract

The registry supports:

- registration
- removal
- lookup by kind
- lookup by capability
- version compatibility checks
- required capability validation
- active plugin listing

## Loading

`PluginLoader` and `PluginHandle` define future loading boundaries. Native,
WASM, FFI, Python, or remote loaders can implement these contracts without
changing core plugin metadata.
