//! Runtime context and options contracts.

use serde::{Deserialize, Serialize};

use crate::{
    cancellation::CancellationToken, channel::ChannelStats, events::EventBusStats,
    pipeline::PipelineGraph, scheduler::SchedulingPolicy,
};

use super::{RuntimeMode, RuntimeStats};

/// Runtime options selected before startup.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeOptions {
    /// Runtime execution mode.
    pub mode: RuntimeMode,
    /// Scheduling policy.
    pub scheduling_policy: SchedulingPolicy,
    /// Maximum worker count.
    pub max_workers: usize,
    /// Whether event emission is enabled.
    pub events_enabled: bool,
    /// Whether metrics collection is enabled.
    pub metrics_enabled: bool,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            mode: RuntimeMode::MultiThreaded,
            scheduling_policy: SchedulingPolicy::Balanced,
            max_workers: 1,
            events_enabled: true,
            metrics_enabled: true,
        }
    }
}

/// Context required to start a runtime.
#[derive(Clone, Debug)]
pub struct RuntimeContext {
    /// Pipeline DAG.
    pub pipeline: PipelineGraph,
    /// Runtime options.
    pub options: RuntimeOptions,
    /// Cancellation token.
    pub cancellation: CancellationToken,
    /// Initial runtime stats.
    pub stats: RuntimeStats,
    /// Event bus stats snapshot.
    pub event_bus_stats: Option<EventBusStats>,
    /// Channel stats snapshots.
    pub channel_stats: Vec<ChannelStats>,
}

impl RuntimeContext {
    /// Creates a runtime context from a pipeline graph.
    #[must_use]
    pub fn new(pipeline: PipelineGraph) -> Self {
        Self {
            pipeline,
            options: RuntimeOptions::default(),
            cancellation: CancellationToken::new(),
            stats: RuntimeStats::default(),
            event_bus_stats: None,
            channel_stats: Vec::new(),
        }
    }

    /// Replaces runtime options.
    #[must_use]
    pub fn with_options(mut self, options: RuntimeOptions) -> Self {
        self.options = options;
        self
    }

    /// Replaces cancellation token.
    #[must_use]
    pub fn with_cancellation(mut self, cancellation: CancellationToken) -> Self {
        self.cancellation = cancellation;
        self
    }
}
