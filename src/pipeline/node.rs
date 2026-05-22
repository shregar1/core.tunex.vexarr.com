//! Pipeline node contracts.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::PipelineStage;
use crate::TunexResult;

/// Stable identifier for a pipeline node.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PipelineNodeId(u64);

impl PipelineNodeId {
    /// Creates a pipeline node identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw node identifier value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Pipeline node family.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineNodeKind {
    /// Dataset source node.
    Dataset,
    /// Tokenizer node.
    Tokenizer,
    /// Sequence packer node.
    Packer,
    /// Trainer backend node.
    Trainer,
    /// Checkpoint writer node.
    Checkpoint,
    /// Metrics node.
    Metrics,
    /// Custom node kind.
    Custom(String),
}

impl PipelineNodeKind {
    /// Returns the expected stage for this node kind.
    #[must_use]
    pub const fn default_stage(&self) -> PipelineStage {
        match self {
            Self::Dataset => PipelineStage::Dataset,
            Self::Tokenizer => PipelineStage::Tokenizer,
            Self::Packer => PipelineStage::Packer,
            Self::Trainer => PipelineStage::Trainer,
            Self::Checkpoint => PipelineStage::Checkpoint,
            Self::Metrics => PipelineStage::Metrics,
            Self::Custom(_) => PipelineStage::Custom,
        }
    }
}

/// Static pipeline node metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineNode {
    /// Stable node identifier.
    pub id: PipelineNodeId,
    /// Human-readable node name.
    pub name: String,
    /// Node kind.
    pub kind: PipelineNodeKind,
    /// Logical stage represented by the node.
    pub stage: PipelineStage,
    /// Optional plugin id or name bound to this node.
    pub plugin: Option<String>,
    /// Logical input type accepted by this node.
    pub input_type: Option<String>,
    /// Logical output type produced by this node.
    pub output_type: Option<String>,
}

impl PipelineNode {
    /// Creates a pipeline node with default stage derived from its kind.
    #[must_use]
    pub fn new(id: PipelineNodeId, name: impl Into<String>, kind: PipelineNodeKind) -> Self {
        let stage = kind.default_stage();
        Self {
            id,
            name: name.into(),
            kind,
            stage,
            plugin: None,
            input_type: None,
            output_type: None,
        }
    }

    /// Creates a pipeline node with explicit stage.
    #[must_use]
    pub fn with_stage(
        id: PipelineNodeId,
        name: impl Into<String>,
        kind: PipelineNodeKind,
        stage: PipelineStage,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind,
            stage,
            plugin: None,
            input_type: None,
            output_type: None,
        }
    }

    /// Binds a plugin id or name.
    #[must_use]
    pub fn with_plugin(mut self, plugin: impl Into<String>) -> Self {
        self.plugin = Some(plugin.into());
        self
    }

    /// Sets logical input and output types.
    #[must_use]
    pub fn with_io(
        mut self,
        input_type: Option<impl Into<String>>,
        output_type: Option<impl Into<String>>,
    ) -> Self {
        self.input_type = input_type.map(Into::into);
        self.output_type = output_type.map(Into::into);
        self
    }
}

/// Backward-compatible alias for node metadata.
pub type PipelineNodeMetadata = PipelineNode;

/// Processing contract for graph-executed pipeline nodes.
#[async_trait]
pub trait PipelineProcessor<I, O>: Send + Sync {
    /// Returns static metadata for this node.
    fn metadata(&self) -> &PipelineNode;

    /// Processes one input item into one output item.
    ///
    /// # Errors
    ///
    /// Returns an error when the node cannot transform the input.
    async fn process(&self, input: I) -> TunexResult<O>;
}
