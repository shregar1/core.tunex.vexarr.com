//! Pipeline stage contracts.

use serde::{Deserialize, Serialize};

/// Logical pipeline stage.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    /// Dataset ingestion stage.
    Dataset,
    /// Tokenization stage.
    Tokenizer,
    /// Sequence packing stage.
    Packer,
    /// Trainer backend stage.
    Trainer,
    /// Checkpoint persistence stage.
    Checkpoint,
    /// Metrics and observability stage.
    Metrics,
    /// User-defined extension stage.
    Custom,
}

impl PipelineStage {
    /// Returns true when an edge from this stage to `next` preserves normal ordering.
    #[must_use]
    pub const fn can_precede(self, next: Self) -> bool {
        matches!(self, Self::Custom)
            || matches!(next, Self::Custom | Self::Metrics)
            || self.order() <= next.order()
    }

    const fn order(self) -> u8 {
        match self {
            Self::Dataset => 0,
            Self::Tokenizer => 1,
            Self::Packer => 2,
            Self::Trainer => 3,
            Self::Checkpoint => 4,
            Self::Metrics | Self::Custom => u8::MAX,
        }
    }
}
