# Memory Model

TuneX core prefers explicit ownership and borrowed views over implicit cloning.

The v0.1 contracts define:

- `ZeroCopyBuffer` for shared byte buffers.
- `OwnedBatch<T>` for owned contiguous batch storage.
- `BorrowedBatch<'a, T>` for borrowed contiguous batch storage.
- `BatchView<'a, T>` for APIs that can operate on either form.

The goal is to keep hot paths cache-friendly and predictable:

- no unbounded queues
- no hidden deep copies
- no pointer-heavy batch graphs
- clear ownership transfer between stages

Concrete arena allocation, NUMA placement, and tensor memory ownership are left
for later phases.
