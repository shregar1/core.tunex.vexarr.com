# Benchmarking

TuneX Core requires performance claims to be backed by benchmark evidence.

## Benchmark Contracts

The benchmark module defines:

- `BenchmarkScenario`
- `BenchmarkRunner`
- `BenchmarkResult`
- `BenchmarkReport`
- `ProfilerBackend`
- `ProfilerConfig`
- `PerformanceBaseline`
- `RegressionThreshold`

## Required Benchmark Categories

- tokens per second
- batch throughput
- queue latency
- scheduler overhead
- serialization overhead
- memory allocation count and bytes
- checkpoint latency

## Profilers

Supported profiler backend contracts include Criterion, perf, flamegraph,
tracing, eBPF, and custom backends.

## Rule

No optimization claim should be accepted without a benchmark result, baseline,
or report that explains what changed and how it was measured.
