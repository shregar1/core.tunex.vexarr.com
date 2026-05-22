# Pipeline Model

TuneX core models training workflows as bounded, graph-based pipelines.

Every executable module is represented as a pipeline node with one conceptual
shape:

```text
input -> process -> output
```

The core repository only defines contracts. Dataset loading, tokenization,
packing, trainer execution, checkpoint persistence, and metrics export remain
external implementations.

The intended execution split is:

- Tokio for orchestration and async coordination.
- Rayon for CPU-heavy worker pools.
- Bounded queues for transfer between stages.
- Event bus contracts for metrics, logs, dashboards, and tracing.
- Backend-specific async tasks for accelerator coordination.

The graph layer must remain implementation-agnostic so future modules can use
Tokio channels, crossbeam queues, lock-free queues, or remote transports without
changing pipeline contracts.
