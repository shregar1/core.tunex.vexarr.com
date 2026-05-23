//! Profiler backend contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Supported profiler backend.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilerBackend {
    /// Criterion benchmark profiler.
    Criterion,
    /// Linux `perf` profiler.
    Perf,
    /// Flamegraph profiler.
    Flamegraph,
    /// `tracing`-based profiler.
    Tracing,
    /// `eBPF` profiler.
    EBPF,
    /// Custom profiler backend.
    Custom(String),
}

/// Profiler configuration for benchmark runs.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProfilerConfig {
    /// Active profiler backend.
    pub backend: ProfilerBackend,
    /// Whether profiling is enabled.
    pub enabled: bool,
    /// Sampling frequency in hertz.
    pub sample_rate_hz: Option<u32>,
    /// Output path for profiler artifacts.
    pub output_path: Option<String>,
    /// Extra backend-specific arguments.
    pub args: SmallVec<[String; 4]>,
}

impl ProfilerConfig {
    /// Creates profiler config.
    #[must_use]
    pub fn new(backend: ProfilerBackend) -> Self {
        Self {
            backend,
            enabled: true,
            sample_rate_hz: None,
            output_path: None,
            args: SmallVec::new(),
        }
    }

    /// Disables profiling while preserving backend metadata.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Sets sampling frequency.
    #[must_use]
    pub fn with_sample_rate_hz(mut self, sample_rate_hz: u32) -> Self {
        self.sample_rate_hz = Some(sample_rate_hz);
        self
    }

    /// Sets output path.
    #[must_use]
    pub fn with_output_path(mut self, output_path: impl Into<String>) -> Self {
        self.output_path = Some(output_path.into());
        self
    }

    /// Adds one backend-specific argument.
    #[must_use]
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }
}
