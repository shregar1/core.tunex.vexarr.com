//! Contract test declarations.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::traits::{
    CheckpointWriter, DatasetSource, MetricsSink, SequencePacker, Tokenizer, TrainerBackend,
};

/// Area covered by a contract test.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractScope {
    /// Trait compliance.
    TraitCompliance,
    /// Fake pipeline execution.
    PipelineRun,
    /// Configuration validation.
    ConfigValidation,
    /// Plugin registry behavior.
    PluginRegistry,
    /// Event bus behavior.
    EventBus,
    /// Runtime state behavior.
    RuntimeState,
    /// Error propagation behavior.
    ErrorPropagation,
}

/// Contract test metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractTest {
    /// Contract name.
    pub name: String,
    /// Scope covered by the contract.
    pub scope: ContractScope,
    /// Required module capability labels.
    pub required_capabilities: SmallVec<[String; 4]>,
}

impl ContractTest {
    /// Creates a contract test.
    #[must_use]
    pub fn new(name: impl Into<String>, scope: ContractScope) -> Self {
        Self {
            name: name.into(),
            scope,
            required_capabilities: SmallVec::new(),
        }
    }

    /// Adds a required capability label.
    #[must_use]
    pub fn with_required_capability(mut self, capability: impl Into<String>) -> Self {
        self.required_capabilities.push(capability.into());
        self
    }
}

/// Compile-time assertion that a type implements `DatasetSource`.
pub fn assert_dataset_source<T: DatasetSource + ?Sized>() {}

/// Compile-time assertion that a type implements `Tokenizer`.
pub fn assert_tokenizer<T: Tokenizer + ?Sized>() {}

/// Compile-time assertion that a type implements `SequencePacker`.
pub fn assert_sequence_packer<T: SequencePacker + ?Sized>() {}

/// Compile-time assertion that a type implements `TrainerBackend`.
pub fn assert_trainer_backend<T: TrainerBackend + ?Sized>() {}

/// Compile-time assertion that a type implements `CheckpointWriter`.
pub fn assert_checkpoint_writer<T: CheckpointWriter + ?Sized>() {}

/// Compile-time assertion that a type implements `MetricsSink`.
pub fn assert_metrics_sink<T: MetricsSink + ?Sized>() {}
