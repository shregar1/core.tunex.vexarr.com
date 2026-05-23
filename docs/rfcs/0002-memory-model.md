# RFC 0002: Memory Model

## Status

Accepted for v0.1 contracts.

## Summary

TuneX Core uses explicit ownership and zero-copy-friendly buffers to keep hot
paths predictable.

## Decision

Core defines shared byte buffers, owned buffers, borrowed views, aligned buffers,
batch views, and arena hooks. Implementations decide allocation strategy later.

## Consequences

- Dataset and tokenizer modules can pass payloads without unnecessary cloning.
- Packer and checkpoint modules can operate on contiguous buffers.
- GPU modules can later add pinned or aligned transfer paths without changing
  high-level contracts.
