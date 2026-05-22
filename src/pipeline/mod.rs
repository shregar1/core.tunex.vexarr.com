//! Pipeline DAG contracts.

pub mod edge;
pub mod graph;
pub mod node;
pub mod stage;
pub mod state;
pub mod validation;

pub use edge::PipelineEdge;
pub use graph::PipelineGraph;
pub use node::{
    PipelineNode, PipelineNodeId, PipelineNodeKind, PipelineNodeMetadata, PipelineProcessor,
};
pub use stage::PipelineStage;
pub use state::PipelineState;
pub use validation::{PipelineValidationError, PipelineValidationReport};

use crate::{types::PipelineEvent, TunexResult};

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
