//! Metrics sink contract.

use async_trait::async_trait;

use crate::{types::StepMetrics, TunexResult};

/// Consumes standardized metrics emitted by pipeline components.
#[async_trait]
pub trait MetricsSink: Send + Sync {
    /// Records metrics for one training step.
    ///
    /// # Errors
    ///
    /// Returns an error when the sink cannot accept the metric sample.
    async fn record_step(&self, metrics: &StepMetrics) -> TunexResult<()>;
}
