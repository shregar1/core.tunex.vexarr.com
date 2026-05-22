//! Error types shared across `TuneX` core modules.

use thiserror::Error;

/// Convenient result alias for fallible `TuneX` core operations.
pub type TunexResult<T> = Result<T, TunexError>;

/// Top-level error type for `TuneX` core.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum TunexError {
    /// Configuration loading, parsing, validation, or override failures.
    #[error("config error: {0}")]
    Config(String),
    /// Runtime lifecycle, worker, cancellation, or orchestration failures.
    #[error("runtime error: {0}")]
    Runtime(String),
    /// Scheduler policy, queue, task, or affinity failures.
    #[error("scheduler error: {0}")]
    Scheduler(String),
    /// Plugin discovery, registration, capability, or execution failures.
    #[error("plugin error: {0}")]
    Plugin(String),
    /// Distributed coordination, membership, heartbeat, or consensus failures.
    #[error("distributed error: {0}")]
    Distributed(String),
    /// Encoding, decoding, schema, or wire format failures.
    #[error("serialization error: {0}")]
    Serialization(String),
    /// Checkpoint read, write, restore, or consistency failures.
    #[error("checkpoint error: {0}")]
    Checkpoint(String),
    /// Backend interoperability or execution failures.
    #[error("backend error: {0}")]
    Backend(String),
    /// Metrics collection, export, or aggregation failures.
    #[error("metrics error: {0}")]
    Metrics(String),
    /// Memory ownership, layout, or capacity failures.
    #[error("memory error: {0}")]
    Memory(String),
    /// Topology graph validation or execution plan failures.
    #[error("topology error: {0}")]
    Topology(String),
    /// WASM plugin sandboxing, loading, or execution failures.
    #[error("wasm error: {0}")]
    Wasm(String),
    /// FFI boundary or ABI failures.
    #[error("ffi error: {0}")]
    Ffi(String),
}

impl TunexError {
    /// Returns the stable category name for this error.
    #[must_use]
    pub const fn category(&self) -> &'static str {
        match self {
            Self::Config(_) => "config",
            Self::Runtime(_) => "runtime",
            Self::Scheduler(_) => "scheduler",
            Self::Plugin(_) => "plugin",
            Self::Distributed(_) => "distributed",
            Self::Serialization(_) => "serialization",
            Self::Checkpoint(_) => "checkpoint",
            Self::Backend(_) => "backend",
            Self::Metrics(_) => "metrics",
            Self::Memory(_) => "memory",
            Self::Topology(_) => "topology",
            Self::Wasm(_) => "wasm",
            Self::Ffi(_) => "ffi",
        }
    }
}
