use async_trait::async_trait;
use bytes::Bytes;
use tunex_core::{
    checkpoint::{
        CheckpointFormat, CheckpointId, CheckpointMetadata, CheckpointPolicy, CheckpointReader,
        CheckpointSnapshot, CheckpointState, CheckpointWriter, ResumePolicy,
    },
    TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

#[test]
fn checkpoint_metadata_captures_partial_lora_and_full_state_modes() {
    let metadata = CheckpointMetadata::new(CheckpointId::new("ckpt-1"), 42, "unsloth")
        .with_format(CheckpointFormat::Safetensors)
        .with_format(CheckpointFormat::LoraAdapters)
        .partial()
        .lora_only();

    assert_eq!(metadata.id.as_str(), "ckpt-1");
    assert_eq!(metadata.step, 42);
    assert_eq!(metadata.backend, "unsloth");
    assert!(metadata.partial);
    assert!(metadata.lora_only);
    assert!(!metadata.full_state);
    assert_eq!(metadata.formats.len(), 2);

    let full = CheckpointMetadata::new(CheckpointId::new("ckpt-full"), 43, "torch").full_state();
    assert!(full.full_state);
}

#[test]
fn snapshot_chunks_are_zero_copy_buffers() {
    let bytes = Bytes::from_static(b"state");
    let snapshot = CheckpointSnapshot::new(CheckpointMetadata::new(
        CheckpointId::new("ckpt-2"),
        7,
        "backend",
    ))
    .with_chunk(bytes.clone())
    .compressed();
    let buffers = snapshot.buffers();

    assert!(snapshot.compressed);
    assert_eq!(snapshot.total_bytes(), 5);
    assert_eq!(buffers[0].bytes().as_ptr(), bytes.as_ptr());
}

#[test]
fn policies_cover_checkpoint_and_resume_modes() {
    let policies = [
        CheckpointPolicy::EveryNSteps(100),
        CheckpointPolicy::EveryNSeconds(60),
        CheckpointPolicy::OnBestLoss,
        CheckpointPolicy::OnShutdown,
        CheckpointPolicy::Manual,
    ];
    let resume = ResumePolicy::FromCheckpoint(CheckpointId::new("ckpt-3"));

    assert_eq!(policies.len(), 5);
    assert!(matches!(resume, ResumePolicy::FromCheckpoint(_)));
}

#[test]
fn checkpoint_contract_types_are_send_sync_and_object_safe() {
    assert_send_sync::<CheckpointId>();
    assert_send_sync::<CheckpointMetadata>();
    assert_send_sync::<CheckpointSnapshot>();
    assert_send_sync::<CheckpointPolicy>();
    assert_send_sync::<ResumePolicy>();
    assert_send_sync::<CheckpointState>();
    assert_object_safe::<dyn CheckpointWriter>();
    assert_object_safe::<dyn CheckpointReader>();
}

struct TestCheckpointStore;

#[async_trait]
impl CheckpointWriter for TestCheckpointStore {
    async fn write_snapshot(
        &self,
        snapshot: CheckpointSnapshot,
    ) -> TunexResult<CheckpointMetadata> {
        Ok(snapshot.metadata)
    }

    async fn write_metadata(&self, _metadata: CheckpointMetadata) -> TunexResult<()> {
        Ok(())
    }
}

#[async_trait]
impl CheckpointReader for TestCheckpointStore {
    async fn read_metadata(&self, id: &CheckpointId) -> TunexResult<CheckpointMetadata> {
        Ok(CheckpointMetadata::new(id.clone(), 1, "test"))
    }

    async fn read_snapshot(&self, id: &CheckpointId) -> TunexResult<CheckpointSnapshot> {
        Ok(CheckpointSnapshot::new(CheckpointMetadata::new(
            id.clone(),
            1,
            "test",
        )))
    }

    async fn resolve_resume(
        &self,
        policy: ResumePolicy,
    ) -> TunexResult<Option<CheckpointMetadata>> {
        Ok(match policy {
            ResumePolicy::None => None,
            ResumePolicy::FromCheckpoint(id) => Some(self.read_metadata(&id).await?),
            _ => Some(CheckpointMetadata::new(
                CheckpointId::new("latest"),
                1,
                "test",
            )),
        })
    }
}

#[test]
fn test_store_implements_reader_and_writer_contracts() {
    let _store = TestCheckpointStore;
}
