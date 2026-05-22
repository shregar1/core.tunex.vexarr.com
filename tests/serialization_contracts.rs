use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use tunex_core::{
    error::ErrorKind,
    serialization::{
        Codec, CodecRegistry, CompressionKind, DecodeOptions, EncodeOptions, EncodedMessage,
        MessageEnvelope, SchemaVersion, SerializationError, SerializationErrorKind,
        SerializationFormat,
    },
    TunexError, TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

struct PassthroughCodec {
    format: SerializationFormat,
    zero_copy: bool,
    streaming: bool,
}

impl Codec for PassthroughCodec {
    fn format(&self) -> SerializationFormat {
        self.format.clone()
    }

    fn encode(
        &self,
        envelope: &MessageEnvelope,
        options: &EncodeOptions,
    ) -> TunexResult<EncodedMessage> {
        Ok(EncodedMessage::new(
            options.format.clone(),
            options.schema_version,
            envelope.payload.clone(),
        )
        .with_compression(options.compression.clone()))
    }

    fn decode(
        &self,
        message: EncodedMessage,
        options: &DecodeOptions,
    ) -> TunexResult<MessageEnvelope> {
        if let Some(max_bytes) = options.max_bytes {
            if message.len() > max_bytes {
                return Err(TunexError::new(
                    ErrorKind::Serialization,
                    "encoded message exceeds max size",
                ));
            }
        }

        Ok(MessageEnvelope::new(
            "codec",
            "decoded",
            message.schema_version,
            message.format,
            message.bytes,
        ))
    }

    fn supports_zero_copy_decode(&self) -> bool {
        self.zero_copy
    }

    fn supports_streaming(&self) -> bool {
        self.streaming
    }
}

#[derive(Default)]
struct InMemoryCodecRegistry {
    codecs: std::sync::Mutex<HashMap<SerializationFormat, Arc<dyn Codec>>>,
}

impl CodecRegistry for InMemoryCodecRegistry {
    fn register_codec(&self, codec: Arc<dyn Codec>) -> TunexResult<()> {
        self.codecs.lock().unwrap().insert(codec.format(), codec);
        Ok(())
    }

    fn remove_codec(&self, format: &SerializationFormat) -> TunexResult<Option<Arc<dyn Codec>>> {
        Ok(self.codecs.lock().unwrap().remove(format))
    }

    fn get_codec(&self, format: &SerializationFormat) -> Option<Arc<dyn Codec>> {
        self.codecs.lock().unwrap().get(format).cloned()
    }

    fn formats(&self) -> Vec<SerializationFormat> {
        self.codecs.lock().unwrap().keys().cloned().collect()
    }
}

#[test]
fn serialization_contract_types_are_send_sync_and_object_safe() {
    assert_send_sync::<SerializationFormat>();
    assert_send_sync::<SchemaVersion>();
    assert_send_sync::<MessageEnvelope>();
    assert_send_sync::<EncodedMessage>();
    assert_send_sync::<EncodeOptions>();
    assert_send_sync::<DecodeOptions>();
    assert_send_sync::<CompressionKind>();
    assert_send_sync::<SerializationError>();
    assert_send_sync::<SerializationErrorKind>();
    assert_object_safe::<dyn Codec>();
    assert_object_safe::<dyn CodecRegistry>();
}

#[test]
fn schema_versions_define_wire_compatibility() {
    let reader = SchemaVersion::new(1, 2, 0);
    let older_writer = SchemaVersion::new(1, 1, 5);
    let newer_writer = SchemaVersion::new(1, 3, 0);
    let other_major = SchemaVersion::new(2, 0, 0);

    assert!(reader.can_decode(older_writer));
    assert!(!reader.can_decode(newer_writer));
    assert!(!reader.can_decode(other_major));
    assert_eq!(reader.to_string(), "1.2.0");
}

#[test]
fn envelope_and_encoded_message_preserve_zero_copy_payloads() {
    let payload = Bytes::from_static(b"heartbeat");
    let payload_ptr = payload.as_ptr();
    let envelope = MessageEnvelope::new(
        "distributed",
        "node_heartbeat",
        SchemaVersion::new(0, 1, 0),
        SerializationFormat::Arrow,
        payload.clone(),
    )
    .with_target_module("runtime")
    .with_trace_id("trace-1")
    .with_checksum("sha256:abc")
    .with_compression(CompressionKind::Zstd)
    .with_metadata("node", "trainer-0");

    assert_eq!(envelope.payload.as_ptr(), payload_ptr);
    assert_eq!(envelope.target_module.as_deref(), Some("runtime"));
    assert_eq!(envelope.metadata.len(), 1);

    let encoded = EncodedMessage::new(
        SerializationFormat::Arrow,
        envelope.schema_version,
        envelope.payload.clone(),
    )
    .with_checksum("sha256:abc");

    assert_eq!(encoded.bytes.as_ptr(), payload_ptr);
    assert_eq!(encoded.len(), 9);
    assert!(!encoded.is_empty());
}

#[test]
fn codec_registry_registers_and_resolves_formats() {
    let registry = InMemoryCodecRegistry::default();
    let codec = Arc::new(PassthroughCodec {
        format: SerializationFormat::FlatBuffers,
        zero_copy: true,
        streaming: true,
    });

    registry.register_codec(codec).unwrap();
    let resolved = registry
        .get_codec(&SerializationFormat::FlatBuffers)
        .expect("codec should be registered");

    assert_eq!(resolved.format(), SerializationFormat::FlatBuffers);
    assert!(resolved.supports_zero_copy_decode());
    assert!(resolved.supports_streaming());
    assert_eq!(registry.formats(), vec![SerializationFormat::FlatBuffers]);
    assert!(registry
        .remove_codec(&SerializationFormat::FlatBuffers)
        .unwrap()
        .is_some());
    assert!(registry
        .get_codec(&SerializationFormat::FlatBuffers)
        .is_none());
}

#[test]
fn codec_contract_supports_encode_decode_options_and_size_limits() {
    let codec = PassthroughCodec {
        format: SerializationFormat::Json,
        zero_copy: false,
        streaming: false,
    };
    let envelope = MessageEnvelope::new(
        "events",
        "metric_emitted",
        SchemaVersion::new(0, 1, 0),
        SerializationFormat::Json,
        Bytes::from_static(br#"{"tokens_per_sec":10}"#),
    );
    let options = EncodeOptions {
        checksum: true,
        ..EncodeOptions::default()
    };

    let encoded = codec.encode(&envelope, &options).unwrap();
    assert_eq!(encoded.format, SerializationFormat::Json);
    assert_eq!(encoded.schema_version, SchemaVersion::new(0, 1, 0));

    let decoded = codec
        .decode(encoded.clone(), &DecodeOptions::default())
        .unwrap();
    assert_eq!(decoded.payload, encoded.bytes);

    let error = codec
        .decode(
            encoded,
            &DecodeOptions {
                max_bytes: Some(1),
                ..DecodeOptions::default()
            },
        )
        .unwrap_err();
    assert_eq!(error.category(), "serialization");
}
