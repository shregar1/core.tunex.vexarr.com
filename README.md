# TuneX Core

TuneX Core is the foundational orchestration and abstraction crate for the
TuneX ecosystem. It defines stable Rust-native contracts for fine-tuning and AI
pipeline infrastructure: traits, shared types, pipeline graphs, runtime
interfaces, plugin metadata, configuration, events, errors, resource contracts,
serialization, benchmarking, and test harness utilities.

The core crate is intentionally not a monolithic trainer. It is the shared
contract layer that lets future modules evolve independently.

## What TuneX Core Is

- A stable public API for TuneX modules.
- A trait system for datasets, tokenizers, packers, trainers, checkpoints, and
  metrics.
- A pipeline DAG and runtime abstraction layer.
- A plugin and module registry contract.
- A zero-copy-oriented memory and batch model.
- A common error, diagnostics, event, and telemetry model.
- A resource, distributed coordination, serialization, benchmark, and testing
  contract layer.

## What TuneX Core Is Not

- It is not a dataset loader implementation.
- It is not a tokenizer implementation.
- It is not a GPU kernel library.
- It is not a tensor runtime or autograd engine.
- It is not a distributed training implementation.
- It does not own heavy compute in v0.1.

## Module Architecture

```text
Python SDK / CLI / Dashboard
          |
          v
    tunex-core
          |
          +-- dataset modules
          +-- tokenizer modules
          +-- packer modules
          +-- trainer backend modules
          +-- checkpoint modules
          +-- metrics modules
          +-- scheduler/runtime modules
          +-- distributed modules
```

Every module depends on core contracts and exposes implementations through
traits and plugin metadata. The intended import path for module authors is:

```rust
use tunex_core::prelude::*;
```

## Trait Examples

```rust
use async_trait::async_trait;
use tunex_core::prelude::*;

struct JsonlDataset;

#[async_trait]
impl DatasetSource for JsonlDataset {
    async fn next_sample(&self) -> TunexResult<Option<RawSample>> {
        Ok(None)
    }
}
```

```rust
use async_trait::async_trait;
use tunex_core::prelude::*;

struct Backend;

#[async_trait]
impl TrainerBackend for Backend {
    async fn train_step(&self, batch: &PackedBatch) -> TunexResult<StepMetrics> {
        Ok(StepMetrics::new(0, batch.token_count() as u64, batch.sequence_count() as u64, 1))
    }
}
```

## Performance Principles

Core contracts are designed around measurable performance:

- bounded queues instead of unbounded memory growth
- zero-copy-friendly `Bytes` and borrowed views
- explicit ownership and state transitions
- low allocation metadata paths
- structured metrics and event contracts
- benchmark contracts for throughput, latency, allocation, checkpoint, and
  serialization measurements

No optimization claim should be accepted without a benchmark result or report.

## Repository Status

Current status: v0.1 foundation contracts.

Implemented contract layers include:

- public API and prelude
- traits and shared pipeline types
- configuration and validation
- plugin registry metadata
- memory and zero-copy contracts
- error and diagnostics model
- event bus contracts
- scheduler, pipeline, channel, cancellation, runtime contracts
- checkpoint, distributed, serialization, resources contracts
- benchmarking and testing harness contracts

Heavy compute modules are intentionally out of scope for this repository.

## Contributing

Start with `CONTRIBUTING.md`, then read the docs in `docs/` and the RFCs under
`docs/rfcs/`. Changes to stable contracts should include tests and, for larger
design changes, an RFC.

Before opening a change, run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo bench --no-run
```
