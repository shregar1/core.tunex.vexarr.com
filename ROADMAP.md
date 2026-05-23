# Roadmap

## v0.1 Foundation

- Stable traits and shared pipeline types.
- Versioned config schema and validation.
- Plugin metadata and registry contracts.
- Pipeline DAG, runtime, scheduler, channel, cancellation, and event contracts.
- Memory, error, checkpoint, distributed, serialization, resources,
  benchmarking, testing, API, and prelude contracts.

## v0.2 Runtime Skeleton

- Bounded pipeline runtime implementation prototype.
- Metrics integration.
- Tracing integration.
- Basic local orchestration smoke tests.

## v0.3 DAG Execution

- Async scheduler prototype.
- Cancellation propagation.
- Pipeline graph execution harness.

## v0.4 Distributed Primitives

- Worker lifecycle implementation.
- Topology management.
- Heartbeat and placement prototypes.

## v0.5 Plugin Expansion

- WASM plugin contracts and sandbox prototype.
- Serialization compatibility tests.
- Stable public API review.
