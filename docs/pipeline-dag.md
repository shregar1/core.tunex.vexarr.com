# Pipeline DAG

TuneX pipelines are directed acyclic graphs. Each module appears as a
`PipelineNode`, and each connection appears as a `PipelineEdge`.

## Node Kinds

- Dataset
- Tokenizer
- Packer
- Trainer
- Checkpoint
- Metrics
- Custom

## Validation

Pipeline graph validation checks:

- duplicate node ids
- cycles
- input/output compatibility
- stage ordering

## Example

```text
dataset -> tokenizer -> packer -> trainer -> checkpoint
                         |
                         v
                       metrics
```

The DAG contract is intentionally independent from execution. A runtime may use
Tokio tasks, Rayon workers, bounded channels, distributed workers, or benchmark
executors while preserving the same graph model.
