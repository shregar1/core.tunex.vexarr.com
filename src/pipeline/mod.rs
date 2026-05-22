//! Pipeline orchestration contracts.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{types::PipelineEvent, TunexResult};

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

/// Logical pipeline stage.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PipelineStage {
    /// Dataset ingestion stage.
    Dataset,
    /// Cleaning or preprocessing stage.
    Cleaner,
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

/// Static metadata for a pipeline node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineNodeMetadata {
    /// Stable node identifier.
    pub id: PipelineNodeId,
    /// Human-readable node name.
    pub name: String,
    /// Logical stage represented by the node.
    pub stage: PipelineStage,
}

impl PipelineNodeMetadata {
    /// Creates node metadata.
    #[must_use]
    pub fn new(id: PipelineNodeId, name: impl Into<String>, stage: PipelineStage) -> Self {
        Self {
            id,
            name: name.into(),
            stage,
        }
    }
}

/// Processing contract for graph-executed pipeline nodes.
#[async_trait]
pub trait PipelineNode<I, O>: Send + Sync {
    /// Returns static metadata for this node.
    fn metadata(&self) -> &PipelineNodeMetadata;

    /// Processes one input item into one output item.
    ///
    /// # Errors
    ///
    /// Returns an error when the node cannot transform the input.
    async fn process(&self, input: I) -> TunexResult<O>;
}

/// Bounded transfer channel for batches or events.
pub trait BatchChannel<T>: Send + Sync {
    /// Returns configured bounded capacity.
    fn capacity(&self) -> usize;

    /// Returns current queue length when observable.
    fn len(&self) -> usize;

    /// Returns true when no items are queued.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Attempts to send without unbounded waiting.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is closed, full, or cancelled.
    fn try_send(&self, item: T) -> TunexResult<()>;

    /// Attempts to receive without unbounded waiting.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is closed or cannot be inspected.
    fn try_recv(&self) -> TunexResult<Option<T>>;
}

/// Publishes pipeline events from node execution.
pub trait PipelineEventPublisher: Send + Sync {
    /// Publishes one pipeline event.
    ///
    /// # Errors
    ///
    /// Returns an error when the event cannot be accepted.
    fn publish(&self, event: PipelineEvent) -> TunexResult<()>;
}
