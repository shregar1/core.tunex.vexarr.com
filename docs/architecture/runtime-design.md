# Runtime Design

The runtime topology is split into four logical layers:

```text
control plane
  -> pipeline runtime
    -> worker pools
      -> execution nodes
```

The control plane owns orchestration decisions. The pipeline runtime owns graph
execution. Worker pools own CPU or async work scheduling. Execution nodes own
the actual contract implementation.

This phase intentionally does not implement a runtime. It defines:

- cancellation tokens
- worker signals
- shutdown reasons
- runtime layer identifiers
- bounded channel contracts
- event bus contracts
- scheduling primitives

Concrete worker loops, Tokio tasks, Rayon pools, GPU coordination, distributed
execution, and retry behavior are future implementation layers.
