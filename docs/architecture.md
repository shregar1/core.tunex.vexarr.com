# Architecture

TuneX Core is the operating-system layer for TuneX modules. It defines contracts
for orchestration, data movement, error propagation, plugins, configuration,
events, resources, benchmarking, and testing.

## Contract Boundary

Core defines:

- public traits
- shared types
- config schemas
- plugin metadata
- pipeline graph contracts
- runtime and scheduler interfaces
- cancellation and channel contracts
- memory and serialization contracts
- diagnostics, events, and benchmarking contracts

Core does not implement:

- dataset readers
- tokenizers
- sequence packing algorithms
- GPU kernels
- distributed transport
- model training backends

## Module Flow

```text
dataset -> tokenizer -> packer -> trainer -> checkpoint
                              \-> metrics
```

Each stage is represented by a pipeline node and connected by bounded channels.
Runtime implementations decide how nodes execute; core only defines the
contracts they must satisfy.

## Public API

Downstream modules should import:

```rust
use tunex_core::prelude::*;
```

Stable public API metadata lives under `tunex_core::api`.
