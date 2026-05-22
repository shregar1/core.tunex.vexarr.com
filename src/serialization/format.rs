//! Serialization format identifiers.

use serde::{Deserialize, Serialize};

/// Wire format used for encoded messages.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializationFormat {
    /// JSON encoding.
    Json,
    /// `MessagePack` encoding.
    MessagePack,
    /// `Protocol Buffers` encoding.
    Protobuf,
    /// `FlatBuffers` encoding.
    FlatBuffers,
    /// Apache Arrow IPC or stream encoding.
    Arrow,
    /// `rkyv` archived encoding.
    Rkyv,
    /// Custom encoding.
    Custom(String),
}

impl SerializationFormat {
    /// Returns whether the format can support zero-copy reads in future codecs.
    #[must_use]
    pub fn is_zero_copy_candidate(&self) -> bool {
        matches!(
            self,
            Self::FlatBuffers | Self::Arrow | Self::Rkyv | Self::Custom(_)
        )
    }
}
