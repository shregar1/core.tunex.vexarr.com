//! Checkpoint identity and metadata contracts.

use std::fmt;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Stable checkpoint identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CheckpointId(String);

impl CheckpointId {
    /// Creates a checkpoint id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Checkpoint storage format.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CheckpointFormat {
    /// Safetensors checkpoint.
    Safetensors,
    /// `PyTorch` checkpoint.
    Torch,
    /// `LoRA` adapter checkpoint.
    LoraAdapters,
    /// Optimizer state checkpoint.
    OptimizerState,
    /// Runtime state checkpoint.
    RuntimeState,
    /// Custom checkpoint format.
    Custom(String),
}

/// Metadata describing a checkpoint artifact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckpointMetadata {
    /// Stable checkpoint id.
    pub id: CheckpointId,
    /// Training step captured by the checkpoint.
    pub step: u64,
    /// Backend that produced the checkpoint.
    pub backend: String,
    /// Checkpoint formats included in the artifact.
    pub formats: SmallVec<[CheckpointFormat; 4]>,
    /// Whether the checkpoint is partial.
    pub partial: bool,
    /// Whether the checkpoint stores only `LoRA` adapters.
    pub lora_only: bool,
    /// Whether the checkpoint contains full runtime/training state.
    pub full_state: bool,
    /// Optional content checksum.
    pub checksum: Option<String>,
    /// Optional checkpoint URI or path.
    pub location: Option<String>,
}

impl CheckpointMetadata {
    /// Creates checkpoint metadata.
    #[must_use]
    pub fn new(id: CheckpointId, step: u64, backend: impl Into<String>) -> Self {
        Self {
            id,
            step,
            backend: backend.into(),
            formats: SmallVec::new(),
            partial: false,
            lora_only: false,
            full_state: false,
            checksum: None,
            location: None,
        }
    }

    /// Adds a format to the metadata.
    #[must_use]
    pub fn with_format(mut self, format: CheckpointFormat) -> Self {
        self.formats.push(format);
        self
    }

    /// Marks the checkpoint as partial.
    #[must_use]
    pub const fn partial(mut self) -> Self {
        self.partial = true;
        self
    }

    /// Marks the checkpoint as `LoRA`-only.
    #[must_use]
    pub const fn lora_only(mut self) -> Self {
        self.lora_only = true;
        self
    }

    /// Marks the checkpoint as full-state.
    #[must_use]
    pub const fn full_state(mut self) -> Self {
        self.full_state = true;
        self
    }
}
