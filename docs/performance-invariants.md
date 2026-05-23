# Performance Invariants

TuneX Core contracts are designed so future implementations can optimize
without changing public APIs. These invariants protect hot paths from accidental
regressions.

## Required Invariants

- No hidden deep clones in pipeline or memory contracts.
- No unbounded queues for pipeline transfer.
- No global locks in hot paths.
- No `Vec<Vec<T>>` for batch tensors or packed token buffers.
- No allocation inside tight loops unless justified by a benchmark or RFC.
- Zero-copy by default for shared byte payloads.
- Bounded memory by default.
- Backpressure must be represented explicitly.
- Scheduling decisions must carry enough resource metadata for later tuning.
- Performance claims require benchmark evidence.

## Batch Layout

Batch-shaped data should prefer:

- contiguous token buffers
- offset arrays
- borrowed views
- shared `Bytes` buffers for immutable payloads
- aligned buffers where SIMD or accelerator transfer is expected

Avoid pointer-heavy object graphs in hot paths.

## Concurrency

Runtime implementations should prefer bounded queues, sharded state, local
worker state, and explicit cancellation. A global lock is acceptable only for
control-plane metadata that is not in the steady-state data path.

## Benchmark Proof

Optimization changes should include one or more of:

- `BenchmarkResult`
- `BenchmarkReport`
- Criterion output
- allocation count comparison
- p95 or p99 latency comparison

No optimization claim should be merged without measurable evidence.
