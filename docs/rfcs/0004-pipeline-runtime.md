# RFC 0004: Pipeline Runtime

## Status

Accepted for v0.1 contracts.

## Summary

TuneX pipelines are DAGs executed by runtime implementations through stable
runtime, scheduler, channel, cancellation, and event contracts.

## Decision

Core defines graph validation, runtime engine interfaces, scheduler primitives,
bounded channel contracts, backpressure metadata, cancellation tokens, shutdown
modes, and event publishing contracts.

## Consequences

- Local, distributed, debug, benchmark, and GPU-bound runtimes can share one DAG
  contract.
- Full runtime behavior can evolve without changing module trait contracts.
- Pipeline failures can propagate through common event and error models.
