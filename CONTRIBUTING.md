# Contributing

TuneX Core is a contract crate. Contributions should preserve stability,
composability, and implementation independence.

## Development Rules

- Keep core implementation-free unless the module is explicitly a contract
  utility, mock, or test harness.
- Prefer stable traits and shared types over concrete subsystem behavior.
- Avoid introducing heavy dependencies into the normal runtime dependency graph.
- Preserve object safety for public trait contracts.
- Add tests for every public contract change.
- Keep memory ownership explicit and avoid hidden clone paths.
- Use `tunex_core::prelude::*` in examples intended for downstream modules.

## Required Checks

Run these before committing:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo bench --no-run
```

## RFCs

Use an RFC for changes that affect:

- public traits
- shared types
- pipeline graph semantics
- runtime state transitions
- plugin compatibility
- memory ownership contracts
- error codes or recoverability semantics
- serialization and wire compatibility

RFCs live in `docs/rfcs/`.
