use std::error::Error;

use tunex_core::{
    config::TunexConfig,
    error::{ErrorKind, Recoverability},
    prelude::*,
    traits::{
        CheckpointWriter, DatasetSource, MetricsSink, SequencePacker, Tokenizer, TrainerBackend,
    },
    TunexError, TunexResult,
};

fn assert_object_safe<T: ?Sized>() {}

#[test]
fn v0_1_public_modules_are_exported() {
    let _ = TunexConfig::default();
    let _ = RawSample::default();
    let _ = StepMetrics::default();
}

#[test]
fn core_traits_are_object_safe() {
    assert_object_safe::<dyn DatasetSource>();
    assert_object_safe::<dyn Tokenizer>();
    assert_object_safe::<dyn SequencePacker>();
    assert_object_safe::<dyn TrainerBackend>();
    assert_object_safe::<dyn CheckpointWriter>();
    assert_object_safe::<dyn MetricsSink>();
}

#[test]
fn error_category_and_display_are_stable() {
    let error = TunexError::new(ErrorKind::Runtime, "worker stopped")
        .with_recoverability(Recoverability::Retryable);
    let dyn_error: &dyn Error = &error;

    assert_eq!(error.category(), "runtime");
    assert_eq!(error.code().as_str(), "TUNEX-RUNTIME-0000");
    assert_eq!(error.recoverability(), Recoverability::Retryable);
    assert_eq!(
        dyn_error.to_string(),
        "runtime error [TUNEX-RUNTIME-0000]: worker stopped"
    );
}

#[test]
fn tunex_result_alias_uses_tunex_error() {
    fn fail() -> TunexResult<()> {
        Err(TunexError::new(ErrorKind::Checkpoint, "missing metadata"))
    }

    assert_eq!(fail().unwrap_err().category(), "checkpoint");
}

#[test]
fn config_defaults_are_stable() {
    let config = TunexConfig::default();

    assert_eq!(config.version, "0.1");
    assert_eq!(config.packer.max_seq_len, 4096);
    assert_eq!(config.runtime.queues.dataset_to_tokenizer, 1024);
    assert_eq!(config.runtime.queues.tokenizer_to_packer, 512);
    assert!(config.checkpoint.enabled);
}
