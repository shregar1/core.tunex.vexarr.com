# Memory Model

TuneX Core favors explicit ownership, contiguous memory, and zero-copy-friendly
interfaces.

## Principles

- Prefer `Bytes` for shared immutable payloads.
- Prefer borrowed views for read-only batch access.
- Prefer contiguous buffers over nested vector graphs.
- Make ownership transfer explicit between pipeline stages.
- Avoid hidden clones in hot paths.

## Contracts

The memory module defines:

- `ZeroCopyBuffer`
- `OwnedBuffer`
- `SharedBuffer`
- `BufferView`
- `OwnedBatch`
- `BorrowedBatch`
- `BatchView`
- `AlignedBuffer`
- `MemoryArena`

These contracts let future modules design dataset, tokenizer, packer,
checkpoint, and GPU transfer paths without rewriting ownership rules.

## Out of Scope

Pinned GPU buffers, NUMA-aware arenas, and allocator implementations are future
implementation concerns. v0.1 defines the ownership vocabulary only.
