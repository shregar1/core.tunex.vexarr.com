//! Benchmark result, baseline, and regression contracts.

use serde::{Deserialize, Serialize};

use crate::events::EventTimestamp;

use super::BenchmarkKind;

/// Historical performance baseline for a benchmark scenario.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PerformanceBaseline {
    /// Scenario name this baseline applies to.
    pub scenario_name: String,
    /// Benchmark category.
    pub kind: BenchmarkKind,
    /// Baseline throughput in tokens per second.
    pub tokens_per_sec: Option<f64>,
    /// Baseline throughput in batches per second.
    pub batches_per_sec: Option<f64>,
    /// Baseline latency p50 in microseconds.
    pub latency_p50_micros: Option<u64>,
    /// Baseline latency p95 in microseconds.
    pub latency_p95_micros: Option<u64>,
    /// Baseline latency p99 in microseconds.
    pub latency_p99_micros: Option<u64>,
    /// Baseline allocation count.
    pub allocations: Option<u64>,
    /// Baseline allocated bytes.
    pub allocated_bytes: Option<u64>,
}

impl PerformanceBaseline {
    /// Creates a baseline for a scenario.
    #[must_use]
    pub fn new(scenario_name: impl Into<String>, kind: BenchmarkKind) -> Self {
        Self {
            scenario_name: scenario_name.into(),
            kind,
            tokens_per_sec: None,
            batches_per_sec: None,
            latency_p50_micros: None,
            latency_p95_micros: None,
            latency_p99_micros: None,
            allocations: None,
            allocated_bytes: None,
        }
    }
}

/// Allowed deviation from a performance baseline.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RegressionThreshold {
    /// Maximum allowed throughput drop in basis points.
    pub max_throughput_drop_basis_points: u16,
    /// Maximum allowed latency increase in basis points.
    pub max_latency_increase_basis_points: u16,
    /// Maximum allowed allocation increase in basis points.
    pub max_allocation_increase_basis_points: u16,
}

impl Default for RegressionThreshold {
    fn default() -> Self {
        Self {
            max_throughput_drop_basis_points: 500,
            max_latency_increase_basis_points: 500,
            max_allocation_increase_basis_points: 500,
        }
    }
}

/// Benchmark result for one scenario run.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BenchmarkResult {
    /// Scenario name.
    pub scenario_name: String,
    /// Benchmark category.
    pub kind: BenchmarkKind,
    /// Result timestamp.
    pub timestamp: EventTimestamp,
    /// Measured iterations.
    pub iterations: u64,
    /// Measured tokens per second.
    pub tokens_per_sec: Option<f64>,
    /// Measured batches per second.
    pub batches_per_sec: Option<f64>,
    /// Measured latency p50 in microseconds.
    pub latency_p50_micros: Option<u64>,
    /// Measured latency p95 in microseconds.
    pub latency_p95_micros: Option<u64>,
    /// Measured latency p99 in microseconds.
    pub latency_p99_micros: Option<u64>,
    /// Measured allocation count.
    pub allocations: Option<u64>,
    /// Measured allocated bytes.
    pub allocated_bytes: Option<u64>,
}

impl BenchmarkResult {
    /// Creates a benchmark result.
    #[must_use]
    pub fn new(scenario_name: impl Into<String>, kind: BenchmarkKind, iterations: u64) -> Self {
        Self {
            scenario_name: scenario_name.into(),
            kind,
            timestamp: EventTimestamp::now(),
            iterations,
            tokens_per_sec: None,
            batches_per_sec: None,
            latency_p50_micros: None,
            latency_p95_micros: None,
            latency_p99_micros: None,
            allocations: None,
            allocated_bytes: None,
        }
    }

    /// Sets measured token throughput.
    #[must_use]
    pub const fn with_tokens_per_sec(mut self, tokens_per_sec: f64) -> Self {
        self.tokens_per_sec = Some(tokens_per_sec);
        self
    }

    /// Sets measured batch throughput.
    #[must_use]
    pub const fn with_batches_per_sec(mut self, batches_per_sec: f64) -> Self {
        self.batches_per_sec = Some(batches_per_sec);
        self
    }

    /// Sets measured latency percentiles.
    #[must_use]
    pub const fn with_latency_micros(mut self, p50: u64, p95: u64, p99: u64) -> Self {
        self.latency_p50_micros = Some(p50);
        self.latency_p95_micros = Some(p95);
        self.latency_p99_micros = Some(p99);
        self
    }

    /// Sets allocation measurements.
    #[must_use]
    pub const fn with_allocations(mut self, allocations: u64, allocated_bytes: u64) -> Self {
        self.allocations = Some(allocations);
        self.allocated_bytes = Some(allocated_bytes);
        self
    }

    /// Returns whether the result regressed against a baseline.
    #[must_use]
    pub fn regressed_against(
        &self,
        baseline: &PerformanceBaseline,
        threshold: RegressionThreshold,
    ) -> bool {
        throughput_regressed(
            baseline.tokens_per_sec,
            self.tokens_per_sec,
            threshold.max_throughput_drop_basis_points,
        ) || throughput_regressed(
            baseline.batches_per_sec,
            self.batches_per_sec,
            threshold.max_throughput_drop_basis_points,
        ) || latency_regressed(
            baseline.latency_p95_micros,
            self.latency_p95_micros,
            threshold.max_latency_increase_basis_points,
        ) || count_regressed(
            baseline.allocations,
            self.allocations,
            threshold.max_allocation_increase_basis_points,
        ) || count_regressed(
            baseline.allocated_bytes,
            self.allocated_bytes,
            threshold.max_allocation_increase_basis_points,
        )
    }
}

fn throughput_regressed(baseline: Option<f64>, current: Option<f64>, drop_bps: u16) -> bool {
    match (baseline, current) {
        (Some(base), Some(value)) if base > 0.0 => {
            let allowed = base * (1.0 - f64::from(drop_bps) / 10_000.0);
            value < allowed
        }
        _ => false,
    }
}

fn latency_regressed(baseline: Option<u64>, current: Option<u64>, increase_bps: u16) -> bool {
    count_regressed(baseline, current, increase_bps)
}

fn count_regressed(baseline: Option<u64>, current: Option<u64>, increase_bps: u16) -> bool {
    match (baseline, current) {
        (Some(base), Some(value)) => {
            let allowed =
                base.saturating_add(base.saturating_mul(u64::from(increase_bps)) / 10_000);
            value > allowed
        }
        _ => false,
    }
}
