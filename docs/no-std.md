# `no_std` Compatibility

TuneX Core is designed to stay lightweight enough for low-level runtime,
accelerator, and embedded-adjacent modules. The crate now supports a `no_std`
crate boundary when default features are disabled:

```bash
cargo check --no-default-features
```

## Feature Model

Default features:

```toml
default = ["std", "alloc", "serde"]
```

Feature meanings:

- `std`: enables the current v0.1 contract surface and all standard-library
  integrations.
- `alloc`: enables use of allocation-aware contracts in future no-std contexts.
- `serde`: enables serialization derives and schema-facing types.

## What Works in `no_std`

With `--no-default-features`, the crate compiles as `#![no_std]`. This gives
downstream low-level modules a dependency-safe crate boundary while the v0.1
contract surface remains behind `std + serde`.

## What Requires `alloc`

Future alloc-only contract refinements should use:

- `alloc::vec::Vec`
- `alloc::string::String`
- `alloc::sync::Arc`
- `alloc::borrow::Cow`

These should be preferred over direct `std` imports in modules intended to move
out of the std-only surface.

## What Requires `std`

The current v0.1 public contract modules require `std`:

- `api`
- `benchmark`
- `cancellation`
- `channel`
- `checkpoint`
- `config`
- `distributed`
- `error`
- `events`
- `memory`
- `pipeline`
- `plugin`
- `resources`
- `runtime`
- `scheduler`
- `serialization`
- `testing`
- `topology`
- `traits`
- `types`

These modules use standard-library synchronization, error traits, environment
access, timestamps, or dependencies that currently assume `std`.

## Compatibility Rule

New contracts should avoid direct `std` imports unless they need operating
system services. Prefer `alloc` paths for owned containers and shared pointers
when the module can remain portable.
