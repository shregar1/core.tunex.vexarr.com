//! Error kind, code, and recoverability contracts.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable error kind.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Configuration loading, parsing, validation, or override failure.
    Config,
    /// Plugin discovery, registration, compatibility, or execution failure.
    Plugin,
    /// Dataset source failure.
    Dataset,
    /// Tokenizer failure.
    Tokenizer,
    /// Sequence packer failure.
    Packer,
    /// Runtime orchestration failure.
    Runtime,
    /// Trainer backend failure.
    Trainer,
    /// Checkpoint failure.
    Checkpoint,
    /// Distributed coordination failure.
    Distributed,
    /// Metrics collection or export failure.
    Metrics,
    /// I/O failure.
    Io,
    /// Serialization or deserialization failure.
    Serialization,
    /// Memory layout, ownership, or allocation failure.
    Memory,
    /// Operation was cancelled.
    Cancelled,
    /// Operation timed out.
    Timeout,
    /// Unsupported operation or capability.
    Unsupported,
    /// Internal invariant or bug.
    Internal,
}

impl ErrorKind {
    /// Returns the stable lowercase category name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Config => "config",
            Self::Plugin => "plugin",
            Self::Dataset => "dataset",
            Self::Tokenizer => "tokenizer",
            Self::Packer => "packer",
            Self::Runtime => "runtime",
            Self::Trainer => "trainer",
            Self::Checkpoint => "checkpoint",
            Self::Distributed => "distributed",
            Self::Metrics => "metrics",
            Self::Io => "io",
            Self::Serialization => "serialization",
            Self::Memory => "memory",
            Self::Cancelled => "cancelled",
            Self::Timeout => "timeout",
            Self::Unsupported => "unsupported",
            Self::Internal => "internal",
        }
    }

    /// Returns the default stable error code for this kind.
    #[must_use]
    pub fn default_code(self) -> ErrorCode {
        let suffix = match self {
            Self::Config => "CONFIG",
            Self::Plugin => "PLUGIN",
            Self::Dataset => "DATASET",
            Self::Tokenizer => "TOKENIZER",
            Self::Packer => "PACKER",
            Self::Runtime => "RUNTIME",
            Self::Trainer => "TRAINER",
            Self::Checkpoint => "CHECKPOINT",
            Self::Distributed => "DISTRIBUTED",
            Self::Metrics => "METRICS",
            Self::Io => "IO",
            Self::Serialization => "SERIALIZATION",
            Self::Memory => "MEMORY",
            Self::Cancelled => "CANCELLED",
            Self::Timeout => "TIMEOUT",
            Self::Unsupported => "UNSUPPORTED",
            Self::Internal => "INTERNAL",
        };
        ErrorCode::new(format!("TUNEX-{suffix}-0000"))
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable machine-readable error code.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ErrorCode(String);

impl ErrorCode {
    /// Creates an error code.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the code as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Recoverability classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Recoverability {
    /// The operation can recover without retrying the same work.
    Recoverable,
    /// The operation may succeed if retried.
    Retryable,
    /// The operation should be treated as fatal.
    Fatal,
}
