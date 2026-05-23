# Unsafe Policy

TuneX Core defaults to no unsafe code.

The crate enforces:

```rust
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
```

This means v0.1 core contracts must remain safe Rust. If a future RFC proposes
an exception, the exception must be narrow, benchmarked when performance-related,
and reviewed as part of the public contract.

## Required Safety Comment

Every unsafe block must include a soundness explanation immediately adjacent to
the unsafe operation:

```rust
// SAFETY:
// The pointer was produced from a valid aligned slice, the lifetime does not
// outlive the source allocation, and no mutable aliases exist.
unsafe {
    // operation
}
```

The explanation must cover:

- provenance of raw pointers or references
- alignment requirements
- initialized memory requirements
- aliasing and mutability rules
- lifetime boundaries
- thread-safety assumptions
- why callers cannot violate the invariant

## Audit Requirements

Unsafe changes require:

- an RFC if the unsafe code affects public contracts
- focused tests around boundary conditions
- benchmark evidence if the unsafe code is performance-motivated
- `cargo geiger` output in CI
- maintainer review from someone familiar with Rust unsafe guidelines

## Current Policy

Unsafe code is not allowed in TuneX Core v0.1. Downstream modules that need
unsafe code for GPU transfers, FFI, SIMD, or memory mapping must keep it behind
their own audited boundaries.
