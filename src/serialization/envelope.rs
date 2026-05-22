//! Wire message envelope contracts.

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::{SchemaVersion, SerializationFormat};

/// Compression selected for an encoded message.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionKind {
    /// No compression.
    #[default]
    None,
    /// `zstd` compression hook.
    Zstd,
    /// `lz4` compression hook.
    Lz4,
    /// Custom compression hook.
    Custom(String),
}

/// Structured metadata entry carried by an envelope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageMetadataEntry {
    /// Metadata key.
    pub key: String,
    /// Metadata value.
    pub value: String,
}

impl MessageMetadataEntry {
    /// Creates a metadata entry.
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// Module-to-module message before final wire encoding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageEnvelope {
    /// Source module name.
    pub source_module: String,
    /// Optional destination module name.
    pub target_module: Option<String>,
    /// Logical message type.
    pub message_type: String,
    /// Schema version used by this message.
    pub schema_version: SchemaVersion,
    /// Preferred serialization format.
    pub format: SerializationFormat,
    /// Zero-copy friendly payload bytes.
    pub payload: Bytes,
    /// Optional checksum supplied by codec or caller.
    pub checksum: Option<String>,
    /// Compression hook selected for payload bytes.
    pub compression: CompressionKind,
    /// Optional trace identifier.
    pub trace_id: Option<String>,
    /// Optional job identifier.
    pub job_id: Option<String>,
    /// Structured metadata.
    pub metadata: SmallVec<[MessageMetadataEntry; 4]>,
}

impl MessageEnvelope {
    /// Creates a message envelope.
    #[must_use]
    pub fn new(
        source_module: impl Into<String>,
        message_type: impl Into<String>,
        schema_version: SchemaVersion,
        format: SerializationFormat,
        payload: Bytes,
    ) -> Self {
        Self {
            source_module: source_module.into(),
            target_module: None,
            message_type: message_type.into(),
            schema_version,
            format,
            payload,
            checksum: None,
            compression: CompressionKind::None,
            trace_id: None,
            job_id: None,
            metadata: SmallVec::new(),
        }
    }

    /// Sets destination module.
    #[must_use]
    pub fn with_target_module(mut self, target_module: impl Into<String>) -> Self {
        self.target_module = Some(target_module.into());
        self
    }

    /// Sets checksum.
    #[must_use]
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// Sets compression.
    #[must_use]
    pub fn with_compression(mut self, compression: CompressionKind) -> Self {
        self.compression = compression;
        self
    }

    /// Sets trace id.
    #[must_use]
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Adds metadata.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push(MessageMetadataEntry::new(key, value));
        self
    }
}

/// Encoded wire-ready message.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EncodedMessage {
    /// Serialization format.
    pub format: SerializationFormat,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Encoded bytes.
    pub bytes: Bytes,
    /// Optional checksum.
    pub checksum: Option<String>,
    /// Compression used for bytes.
    pub compression: CompressionKind,
}

impl EncodedMessage {
    /// Creates an encoded message.
    #[must_use]
    pub fn new(format: SerializationFormat, schema_version: SchemaVersion, bytes: Bytes) -> Self {
        Self {
            format,
            schema_version,
            bytes,
            checksum: None,
            compression: CompressionKind::None,
        }
    }

    /// Returns the encoded payload length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the encoded payload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Sets checksum.
    #[must_use]
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// Sets compression.
    #[must_use]
    pub fn with_compression(mut self, compression: CompressionKind) -> Self {
        self.compression = compression;
        self
    }
}

/// Encoding options selected by callers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EncodeOptions {
    /// Target serialization format.
    pub format: SerializationFormat,
    /// Target schema version.
    pub schema_version: SchemaVersion,
    /// Compression hook.
    pub compression: CompressionKind,
    /// Whether codecs should attach a checksum.
    pub checksum: bool,
    /// Whether streaming encoding is allowed.
    pub streaming: bool,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            format: SerializationFormat::Json,
            schema_version: SchemaVersion::new(0, 1, 0),
            compression: CompressionKind::None,
            checksum: false,
            streaming: false,
        }
    }
}

/// Decoding options selected by callers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DecodeOptions {
    /// Expected format, when the caller wants strict checking.
    pub expected_format: Option<SerializationFormat>,
    /// Reader schema version used for compatibility checks.
    pub reader_schema_version: SchemaVersion,
    /// Whether zero-copy decode paths are allowed.
    pub allow_zero_copy: bool,
    /// Whether checksums must be verified.
    pub verify_checksum: bool,
    /// Optional maximum encoded size in bytes.
    pub max_bytes: Option<usize>,
    /// Whether streaming decoding is allowed.
    pub streaming: bool,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            expected_format: None,
            reader_schema_version: SchemaVersion::new(0, 1, 0),
            allow_zero_copy: true,
            verify_checksum: false,
            max_bytes: None,
            streaming: false,
        }
    }
}
