//! Benchmark report contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::events::EventTimestamp;

use super::{BenchmarkResult, PerformanceBaseline, RegressionThreshold};

/// Aggregated benchmark report.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BenchmarkReport {
    /// Report title.
    pub title: String,
    /// Report creation timestamp.
    pub timestamp: EventTimestamp,
    /// Benchmark results.
    pub results: SmallVec<[BenchmarkResult; 8]>,
    /// Baselines used for comparison.
    pub baselines: SmallVec<[PerformanceBaseline; 8]>,
    /// Regression threshold used for report evaluation.
    pub threshold: RegressionThreshold,
}

impl BenchmarkReport {
    /// Creates an empty benchmark report.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            timestamp: EventTimestamp::now(),
            results: SmallVec::new(),
            baselines: SmallVec::new(),
            threshold: RegressionThreshold::default(),
        }
    }

    /// Sets regression threshold.
    #[must_use]
    pub fn with_threshold(mut self, threshold: RegressionThreshold) -> Self {
        self.threshold = threshold;
        self
    }

    /// Adds a benchmark result.
    #[must_use]
    pub fn with_result(mut self, result: BenchmarkResult) -> Self {
        self.results.push(result);
        self
    }

    /// Adds a performance baseline.
    #[must_use]
    pub fn with_baseline(mut self, baseline: PerformanceBaseline) -> Self {
        self.baselines.push(baseline);
        self
    }

    /// Returns whether any result regressed against a matching baseline.
    #[must_use]
    pub fn has_regression(&self) -> bool {
        self.results.iter().any(|result| {
            self.baselines
                .iter()
                .find(|baseline| {
                    baseline.scenario_name == result.scenario_name && baseline.kind == result.kind
                })
                .is_some_and(|baseline| result.regressed_against(baseline, self.threshold))
        })
    }
}
