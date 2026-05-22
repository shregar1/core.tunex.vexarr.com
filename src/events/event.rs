//! Structured telemetry event contracts.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::Uuid;

/// Stable event identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EventId(String);

impl EventId {
    /// Creates an event id from text.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Creates an event id from a UUID.
    #[must_use]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }

    /// Returns the event id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Event timestamp represented as Unix epoch milliseconds.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EventTimestamp(i64);

impl EventTimestamp {
    /// Creates a timestamp from Unix epoch milliseconds.
    #[must_use]
    pub const fn from_unix_millis(value: i64) -> Self {
        Self(value)
    }

    /// Returns the current wall-clock timestamp.
    #[must_use]
    pub fn now() -> Self {
        Self(Utc::now().timestamp_millis())
    }

    /// Returns Unix epoch milliseconds.
    #[must_use]
    pub const fn as_unix_millis(self) -> i64 {
        self.0
    }
}

/// Event severity.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSeverity {
    /// Debug event.
    Debug,
    /// Informational event.
    Info,
    /// Warning event.
    Warning,
    /// Error event.
    Error,
    /// Critical event.
    Critical,
}

/// Event kind.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// Job started.
    JobStarted,
    /// Job completed.
    JobCompleted,
    /// Job failed.
    JobFailed,
    /// Stage started.
    StageStarted,
    /// Stage completed.
    StageCompleted,
    /// Stage failed.
    StageFailed,
    /// Worker started.
    WorkerStarted,
    /// Worker stopped.
    WorkerStopped,
    /// Batch created.
    BatchCreated,
    /// Batch processed.
    BatchProcessed,
    /// Queue stalled.
    QueueStalled,
    /// Backpressure applied.
    BackpressureApplied,
    /// Checkpoint started.
    CheckpointStarted,
    /// Checkpoint completed.
    CheckpointCompleted,
    /// Checkpoint failed.
    CheckpointFailed,
    /// Metric emitted.
    MetricEmitted,
    /// Plugin registered.
    PluginRegistered,
    /// Plugin failed.
    PluginFailed,
    /// GPU waiting.
    GpuWaiting,
    /// Out-of-memory condition.
    OutOfMemory,
    /// Operation cancelled.
    Cancelled,
}

/// Event source identity and correlation fields.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventSource {
    /// Module that emitted the event.
    pub module: Option<String>,
    /// Pipeline stage that emitted the event.
    pub stage: Option<String>,
    /// Trace identifier.
    pub trace_id: Option<String>,
    /// Job identifier.
    pub job_id: Option<String>,
    /// Worker identifier.
    pub worker_id: Option<String>,
    /// Batch identifier.
    pub batch_id: Option<String>,
}

impl EventSource {
    /// Creates an empty event source.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the module name.
    #[must_use]
    pub fn with_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }

    /// Sets the stage name.
    #[must_use]
    pub fn with_stage(mut self, stage: impl Into<String>) -> Self {
        self.stage = Some(stage.into());
        self
    }

    /// Sets the trace id.
    #[must_use]
    pub fn with_trace(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Sets the job id.
    #[must_use]
    pub fn with_job(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// Sets the worker id.
    #[must_use]
    pub fn with_worker(mut self, worker_id: impl Into<String>) -> Self {
        self.worker_id = Some(worker_id.into());
        self
    }

    /// Sets the batch id.
    #[must_use]
    pub fn with_batch(mut self, batch_id: impl Into<String>) -> Self {
        self.batch_id = Some(batch_id.into());
        self
    }
}

/// One metadata key/value pair.
pub type EventMetadataEntry = (String, String);

/// Low-allocation event metadata.
pub type EventMetadata = SmallVec<[EventMetadataEntry; 8]>;

/// Structured event emitted by `TuneX` components.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TunexEvent {
    /// Stable event id.
    pub id: EventId,
    /// Event timestamp.
    pub timestamp: EventTimestamp,
    /// Event kind.
    pub kind: EventKind,
    /// Event severity.
    pub severity: EventSeverity,
    /// Source and correlation fields.
    pub source: EventSource,
    /// Human-readable event message.
    pub message: Option<String>,
    /// Machine-readable metadata.
    pub metadata: EventMetadata,
}

impl TunexEvent {
    /// Creates an event with no message or metadata.
    #[must_use]
    pub fn new(id: EventId, kind: EventKind, severity: EventSeverity) -> Self {
        Self {
            id,
            timestamp: EventTimestamp::now(),
            kind,
            severity,
            source: EventSource::default(),
            message: None,
            metadata: EventMetadata::new(),
        }
    }

    /// Sets the event timestamp.
    #[must_use]
    pub const fn with_timestamp(mut self, timestamp: EventTimestamp) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the event source.
    #[must_use]
    pub fn with_source(mut self, source: EventSource) -> Self {
        self.source = source;
        self
    }

    /// Sets the human-readable event message.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Adds one metadata entry.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}
