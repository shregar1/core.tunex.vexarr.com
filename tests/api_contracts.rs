use std::error::Error;

use tunex_core::{
    config::CoreConfig,
    prelude::*,
    traits::{
        CheckpointWriter, DatasetSource, MetricsSink, SequencePacker, Tokenizer, TrainerBackend,
    },
    TunexError, TunexResult,
};

fn assert_object_safe<T: ?Sized>() {}

#[test]
fn v0_1_public_modules_are_exported() {
    let _ = CoreConfig::default();
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
    let error = TunexError::Runtime("worker stopped".to_owned());
    let dyn_error: &dyn Error = &error;

    assert_eq!(error.category(), "runtime");
    assert_eq!(dyn_error.to_string(), "runtime error: worker stopped");
}

#[test]
fn tunex_result_alias_uses_tunex_error() {
    fn fail() -> TunexResult<()> {
        Err(TunexError::Checkpoint("missing metadata".to_owned()))
    }

    assert_eq!(fail().unwrap_err().category(), "checkpoint");
}

#[test]
fn config_defaults_are_stable() {
    let config = CoreConfig::default();

    assert_eq!(config.pipeline.max_tokens_per_batch, 4096);
    assert_eq!(config.pipeline.max_samples_per_batch, 64);
    assert!(config.metrics.enabled);
    assert_eq!(config.metrics.namespace.as_deref(), Some("tunex"));
    assert!(config.checkpoint.enabled);
}
