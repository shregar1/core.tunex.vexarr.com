//! Serialization-specific error metadata.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Serialization contract error kind.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializationErrorKind {
    /// Requested format is unsupported.
    UnsupportedFormat,
    /// Schema versions are incompatible.
    SchemaMismatch,
    /// No codec exists for the requested format.
    CodecUnavailable,
    /// Encoding failed.
    EncodeFailed,
    /// Decoding failed.
    DecodeFailed,
    /// Checksum validation failed.
    ChecksumMismatch,
    /// Message exceeded configured size limits.
    MessageTooLarge,
}

/// Serialization contract error.
#[derive(Clone, Debug, Deserialize, Error, Eq, PartialEq, Serialize)]
#[error("{kind:?}: {message}")]
pub struct SerializationError {
    /// Error kind.
    pub kind: SerializationErrorKind,
    /// Human-readable message.
    pub message: String,
}

impl SerializationError {
    /// Creates a serialization error.
    #[must_use]
    pub fn new(kind: SerializationErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}
