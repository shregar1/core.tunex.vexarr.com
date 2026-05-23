# RFC 0003: Plugin System

## Status

Accepted for v0.1 contracts.

## Summary

TuneX modules should register through a common plugin metadata and capability
layer.

## Decision

Core defines plugin identity, versioning, kinds, capabilities, status, registry
operations, handles, and loader contracts.

## Consequences

- Dataset, tokenizer, packer, trainer, checkpoint, metrics, runtime, scheduler,
  and distributed modules can be separate repositories.
- Future native, WASM, FFI, Python, and remote loaders can share the same
  metadata model.
