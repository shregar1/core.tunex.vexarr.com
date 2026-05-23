# RFC 0001: Core Architecture

## Status

Accepted for v0.1 contracts.

## Summary

TuneX Core is a contract-first crate. It defines shared interfaces and runtime
abstractions without implementing heavy training compute.

## Decision

Core owns stable contracts for traits, shared types, configuration, plugins,
pipeline DAGs, runtime interfaces, errors, events, resources, serialization,
benchmarking, and testing.

Module implementations live outside core.

## Consequences

- Module repositories can evolve independently.
- Public contract changes require tests and usually an RFC.
- Core stays small enough to remain a dependency of every TuneX module.
