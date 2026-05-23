//! Benchmark runner contract.

use async_trait::async_trait;

use crate::TunexResult;

use super::{BenchmarkReport, BenchmarkResult, BenchmarkScenario, ProfilerConfig};

/// Runs benchmark scenarios and produces measurable results.
#[async_trait]
pub trait BenchmarkRunner: Send + Sync {
    /// Runs one benchmark scenario.
    ///
    /// # Errors
    ///
    /// Returns an error when the benchmark cannot be executed.
    async fn run_scenario(
        &self,
        scenario: BenchmarkScenario,
        profiler: Option<ProfilerConfig>,
    ) -> TunexResult<BenchmarkResult>;

    /// Runs a collection of scenarios into one report.
    ///
    /// # Errors
    ///
    /// Returns an error when any benchmark cannot be executed.
    async fn run_report(
        &self,
        title: &str,
        scenarios: &[BenchmarkScenario],
        profiler: Option<ProfilerConfig>,
    ) -> TunexResult<BenchmarkReport>;
}
