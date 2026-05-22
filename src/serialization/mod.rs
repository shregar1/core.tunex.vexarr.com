//! Serialization and wire protocol contracts.

pub mod codec;
pub mod envelope;
pub mod error;
pub mod format;
pub mod schema;

pub use codec::{Codec, CodecRegistry};
pub use envelope::{
    CompressionKind, DecodeOptions, EncodeOptions, EncodedMessage, MessageEnvelope,
    MessageMetadataEntry,
};
pub use error::{SerializationError, SerializationErrorKind};
pub use format::SerializationFormat;
pub use schema::SchemaVersion;
