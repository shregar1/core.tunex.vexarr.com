# Error Model

TuneX Core uses one error model across all modules.

## Core Types

- `TunexError`
- `TunexResult<T>`
- `ErrorKind`
- `ErrorCode`
- `ErrorContext`
- `Diagnostic`
- `Recoverability`

## Error Kinds

Error kinds include config, plugin, dataset, tokenizer, packer, runtime,
trainer, checkpoint, distributed, metrics, I/O, serialization, memory,
cancelled, timeout, unsupported, and internal.

## Requirements

Errors must support:

- stable error codes
- recoverable, retryable, or fatal classification
- module, stage, worker, batch, and trace context
- source chaining
- human-readable diagnostics
- machine-readable metadata

The goal is consistent propagation from plugin code through pipeline,
scheduler, runtime, metrics, and dashboard layers.
