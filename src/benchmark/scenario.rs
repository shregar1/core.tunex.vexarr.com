//! Benchmark scenario contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Supported benchmark category.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkKind {
    /// Tokens per second benchmark.
    TokensPerSecond,
    /// Batch throughput benchmark.
    BatchThroughput,
    /// Queue latency benchmark.
    QueueLatency,
    /// Scheduler overhead benchmark.
    SchedulerOverhead,
    /// Serialization benchmark.
    Serialization,
    /// Memory allocation benchmark.
    MemoryAllocation,
    /// Checkpoint latency benchmark.
    CheckpointLatency,
    /// Custom benchmark category.
    Custom(String),
}

/// Benchmark scenario definition.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BenchmarkScenario {
    /// Stable scenario name.
    pub name: String,
    /// Benchmark category.
    pub kind: BenchmarkKind,
    /// Number of warmup iterations.
    pub warmup_iterations: u32,
    /// Number of measured iterations.
    pub measurement_iterations: u32,
    /// Batch size used by the scenario.
    pub batch_size: Option<usize>,
    /// Sequence length used by the scenario.
    pub sequence_length: Option<usize>,
    /// Payload bytes used by serialization or queue scenarios.
    pub payload_bytes: Option<usize>,
    /// Required feature labels or backend tags.
    pub tags: SmallVec<[String; 4]>,
}

impl BenchmarkScenario {
    /// Creates a benchmark scenario.
    #[must_use]
    pub fn new(name: impl Into<String>, kind: BenchmarkKind) -> Self {
        Self {
            name: name.into(),
            kind,
            warmup_iterations: 10,
            measurement_iterations: 100,
            batch_size: None,
            sequence_length: None,
            payload_bytes: None,
            tags: SmallVec::new(),
        }
    }

    /// Sets warmup iterations.
    #[must_use]
    pub const fn with_warmup_iterations(mut self, warmup_iterations: u32) -> Self {
        self.warmup_iterations = warmup_iterations;
        self
    }

    /// Sets measured iterations.
    #[must_use]
    pub const fn with_measurement_iterations(mut self, measurement_iterations: u32) -> Self {
        self.measurement_iterations = measurement_iterations;
        self
    }

    /// Sets batch size.
    #[must_use]
    pub const fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    /// Sets sequence length.
    #[must_use]
    pub const fn with_sequence_length(mut self, sequence_length: usize) -> Self {
        self.sequence_length = Some(sequence_length);
        self
    }

    /// Sets payload bytes.
    #[must_use]
    pub const fn with_payload_bytes(mut self, payload_bytes: usize) -> Self {
        self.payload_bytes = Some(payload_bytes);
        self
    }

    /// Adds a tag.
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}
