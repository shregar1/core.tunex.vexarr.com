# Memory Safety

TuneX Core is a safe-Rust contract layer. Memory safety depends on explicit
ownership, bounded transfer, and clear lifetime boundaries.

## Ownership Rules

- Owned buffers transfer ownership across module boundaries.
- Borrowed views must not outlive their source allocation.
- Shared immutable payloads should use reference-counted byte buffers.
- Mutable access must be exclusive.
- Batch slicing must not clone payload data unless explicitly requested.

## Zero-Copy Rules

Zero-copy APIs must preserve:

- source allocation lifetime
- byte offset and length correctness
- alignment metadata where relevant
- immutable aliasing rules
- clear conversion boundaries for typed views

## Bounded Memory

Pipeline stages must use bounded queues and explicit backpressure metadata.
Unbounded producer growth is a correctness risk, not only a performance issue.

## Unsafe Boundaries

Core v0.1 denies unsafe code. If future modules use unsafe code for FFI, GPU
transfer, SIMD, `mmap`, or raw pointer views, they must document the boundary
with `// SAFETY:` comments and keep the unsafe operation isolated from the
public contract where possible.

## Testing Expectations

Memory-related contract changes should include tests for:

- clone behavior
- slicing without payload copies
- empty buffers
- alignment checks
- send and sync behavior
- ownership transfer through mock pipeline paths
