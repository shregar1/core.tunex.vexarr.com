use bytes::Bytes;
use tunex_core::types::{
    CheckpointMetadata, PackedBatch, PipelineEvent, RawSample, StepMetrics, TokenizedSample,
    WorkerState,
};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn raw_sample_clone_shares_payload_bytes() {
    let original = RawSample::with_id("sample-1", Bytes::from_static(b"hello"));
    let cloned = original.clone();

    assert_eq!(original.id.as_deref(), Some("sample-1"));
    assert_eq!(original.payload.as_ptr(), cloned.payload.as_ptr());
    assert_eq!(original.payload, cloned.payload);
}

#[test]
fn tokenized_sample_builds_default_attention_mask() {
    let sample = TokenizedSample::new(vec![10, 20, 30]);

    assert_eq!(sample.token_ids, [10, 20, 30]);
    assert_eq!(sample.attention_mask, [1, 1, 1]);
}

#[test]
fn packed_batch_reports_shape() {
    let batch = PackedBatch::new(vec![1, 2, 3, 4], vec![0, 2], vec![1, 2, 3, 4]);

    assert_eq!(batch.token_count(), 4);
    assert_eq!(batch.sequence_count(), 2);
    assert!(!batch.is_empty());
}

#[test]
fn shared_types_are_send_sync() {
    assert_send_sync::<RawSample>();
    assert_send_sync::<TokenizedSample>();
    assert_send_sync::<PackedBatch>();
    assert_send_sync::<StepMetrics>();
    assert_send_sync::<CheckpointMetadata>();
    assert_send_sync::<WorkerState>();
    assert_send_sync::<PipelineEvent>();
}
