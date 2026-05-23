use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use bytes::Bytes;
use proptest::prelude::*;
use rstest::rstest;
use tunex_core::{
    checkpoint::{CheckpointId, CheckpointMetadata, CheckpointSnapshot},
    config::{ConfigOverride, OverrideSource, TunexConfig},
    testing::{
        assert_checkpoint_writer, assert_dataset_source, assert_sequence_packer, assert_tokenizer,
        assert_trainer_backend, AssertionLevel, ContractHarness, ContractScope, ContractTest,
        MockCheckpointWriter, MockDatasetSource, MockSequencePacker, MockTokenizer,
        MockTrainerBackend, TestAssertion, TestFixture,
    },
    traits::{CheckpointWriter, DatasetSource, SequencePacker, Tokenizer, TrainerBackend},
    types::{RawSample, TokenizedSample},
};

fn assert_send_sync<T: Send + Sync>() {}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

#[test]
fn testing_contract_types_are_send_sync() {
    assert_send_sync::<ContractTest>();
    assert_send_sync::<ContractHarness>();
    assert_send_sync::<MockDatasetSource>();
    assert_send_sync::<MockTokenizer>();
    assert_send_sync::<MockSequencePacker>();
    assert_send_sync::<MockTrainerBackend>();
    assert_send_sync::<MockCheckpointWriter>();
    assert_send_sync::<TestFixture>();
    assert_send_sync::<TestAssertion>();
}

#[test]
fn mock_modules_implement_core_traits() {
    assert_dataset_source::<MockDatasetSource>();
    assert_tokenizer::<MockTokenizer>();
    assert_sequence_packer::<MockSequencePacker>();
    assert_trainer_backend::<MockTrainerBackend>();
    assert_checkpoint_writer::<MockCheckpointWriter>();

    fn assert_object_safe<T: ?Sized>() {}
    assert_object_safe::<dyn DatasetSource>();
    assert_object_safe::<dyn Tokenizer>();
    assert_object_safe::<dyn SequencePacker>();
    assert_object_safe::<dyn TrainerBackend>();
    assert_object_safe::<dyn CheckpointWriter>();
}

#[rstest]
#[case("sample-a", "hello")]
#[case("sample-b", "TuneX")]
fn fixture_and_mocks_support_fake_pipeline_runs(#[case] id: &str, #[case] text: &str) {
    let fixture = TestFixture::new("fake-pipeline").with_text_sample(id, text);
    let dataset = MockDatasetSource::new(fixture.raw_samples.clone());
    let tokenizer = MockTokenizer;
    let packer = MockSequencePacker;
    let trainer = MockTrainerBackend::new();

    let raw = block_on(dataset.next_sample()).unwrap().unwrap();
    let tokenized = block_on(tokenizer.tokenize(&raw)).unwrap();
    let batch = block_on(packer.pack(&[tokenized])).unwrap();
    let metrics = block_on(trainer.train_step(&batch)).unwrap();

    assert_eq!(raw.id.as_deref(), Some(id));
    assert_eq!(dataset.remaining(), 0);
    assert_eq!(batch.sequence_count(), 1);
    assert_eq!(metrics.step, 0);
    assert_eq!(metrics.tokens, u64::try_from(text.len()).unwrap());
}

#[test]
fn checkpoint_writer_mock_records_metadata_and_snapshots() {
    let writer = MockCheckpointWriter::default();
    let metadata = CheckpointMetadata::new(CheckpointId::new("ckpt-1"), 1, "mock");
    let snapshot =
        CheckpointSnapshot::new(metadata.clone()).with_chunk(Bytes::from_static(b"state"));

    let written = block_on(writer.write_snapshot(snapshot)).unwrap();
    block_on(writer.write_metadata(metadata.clone())).unwrap();

    assert_eq!(written, metadata);
    assert_eq!(writer.snapshots().len(), 1);
    assert_eq!(writer.metadata().len(), 2);
}

#[test]
fn harness_verifies_config_runtime_error_and_registered_contracts() {
    let mut config = TunexConfig::default();
    config
        .apply_cli_overrides(&[
            ConfigOverride::new("dataset.plugin", "jsonl", OverrideSource::Cli),
            ConfigOverride::new("dataset.path", "train.jsonl", OverrideSource::Cli),
            ConfigOverride::new("tokenizer.plugin", "hf", OverrideSource::Cli),
            ConfigOverride::new("tokenizer.model", "tok", OverrideSource::Cli),
            ConfigOverride::new("trainer.backend", "mock", OverrideSource::Cli),
        ])
        .unwrap();
    let fixture = TestFixture::new("module-contracts")
        .with_config(config)
        .with_tokenized_sample(TokenizedSample::new(vec![1, 2, 3]));
    let harness = ContractHarness::new(fixture)
        .with_contract(ContractTest::new(
            "config-valid",
            ContractScope::ConfigValidation,
        ))
        .with_contract(ContractTest::new(
            "runtime-state",
            ContractScope::RuntimeState,
        ))
        .with_contract(ContractTest::new(
            "error-propagation",
            ContractScope::ErrorPropagation,
        ))
        .with_contract(
            ContractTest::new("plugin-registry", ContractScope::PluginRegistry)
                .with_required_capability("plugin:registry"),
        )
        .with_contract(ContractTest::new("event-bus", ContractScope::EventBus));

    let report = harness.verify();

    assert!(report.passed());
    assert_eq!(report.assertions.len(), 5);
    assert!(report
        .assertions
        .iter()
        .any(|assertion| assertion.name == "plugin-registry"));
}

#[test]
fn assertions_capture_required_and_advisory_failures() {
    let advisory = TestAssertion::failed("allocation-budget", "allocation budget exceeded")
        .with_level(AssertionLevel::Advisory);

    assert!(!advisory.passed);
    assert_eq!(advisory.level, AssertionLevel::Advisory);
    assert_eq!(TestAssertion::passed("traits", "ok").message, "ok");
}

proptest! {
    #[test]
    fn mock_tokenizer_preserves_payload_length_as_token_count(payload in proptest::collection::vec(any::<u8>(), 0..256)) {
        let raw = RawSample::new(Bytes::from(payload.clone()));
        let tokenized = block_on(MockTokenizer.tokenize(&raw)).unwrap();

        prop_assert_eq!(tokenized.token_ids.len(), payload.len());
        prop_assert_eq!(tokenized.attention_mask.len(), payload.len());
    }
}
