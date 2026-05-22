//! Codec and codec registry contracts.

use std::sync::Arc;

use crate::TunexResult;

use super::{DecodeOptions, EncodeOptions, EncodedMessage, MessageEnvelope, SerializationFormat};

/// Codec for one serialization format.
pub trait Codec: Send + Sync {
    /// Returns the format supported by this codec.
    fn format(&self) -> SerializationFormat;

    /// Encodes an envelope into wire bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when the envelope cannot be encoded.
    fn encode(
        &self,
        envelope: &MessageEnvelope,
        options: &EncodeOptions,
    ) -> TunexResult<EncodedMessage>;

    /// Decodes wire bytes into an envelope.
    ///
    /// # Errors
    ///
    /// Returns an error when the message cannot be decoded.
    fn decode(
        &self,
        message: EncodedMessage,
        options: &DecodeOptions,
    ) -> TunexResult<MessageEnvelope>;

    /// Returns whether this codec can decode without copying where the format allows it.
    fn supports_zero_copy_decode(&self) -> bool {
        self.format().is_zero_copy_candidate()
    }

    /// Returns whether this codec supports streaming encode/decode.
    fn supports_streaming(&self) -> bool {
        false
    }
}

/// Registry for serialization codecs.
pub trait CodecRegistry: Send + Sync {
    /// Registers a codec.
    ///
    /// # Errors
    ///
    /// Returns an error when the codec cannot be registered.
    fn register_codec(&self, codec: Arc<dyn Codec>) -> TunexResult<()>;

    /// Removes a codec by format.
    ///
    /// # Errors
    ///
    /// Returns an error when the codec cannot be removed.
    fn remove_codec(&self, format: &SerializationFormat) -> TunexResult<Option<Arc<dyn Codec>>>;

    /// Looks up a codec by format.
    fn get_codec(&self, format: &SerializationFormat) -> Option<Arc<dyn Codec>>;

    /// Lists registered formats.
    fn formats(&self) -> Vec<SerializationFormat>;
}
