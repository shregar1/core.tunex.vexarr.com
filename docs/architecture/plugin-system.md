# Plugin System

TuneX plugins are described by stable metadata and loaded through abstract
loader contracts.

The v0.1 plugin surface includes:

- `PluginMetadata`
- `PluginVersion`
- `PluginCapability`
- `PluginHandle`
- `PluginLoader`

The loader contract is intentionally generic. Future loaders may support native
Rust plugins, WASM components, remote plugins, or FFI-backed plugins without
changing callers that depend on the core contract.

Plugin validation, signing, sandboxing, ABI management, and discovery are not
implemented in this phase.
