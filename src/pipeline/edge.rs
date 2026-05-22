//! Pipeline edge contracts.

use serde::{Deserialize, Serialize};

use super::PipelineNodeId;

/// Directed edge between two pipeline nodes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineEdge {
    /// Upstream node.
    pub from: PipelineNodeId,
    /// Downstream node.
    pub to: PipelineNodeId,
    /// Optional logical output type expected from the upstream node.
    pub output_type: Option<String>,
    /// Optional logical input type expected by the downstream node.
    pub input_type: Option<String>,
}

impl PipelineEdge {
    /// Creates a directed pipeline edge.
    #[must_use]
    pub const fn new(from: PipelineNodeId, to: PipelineNodeId) -> Self {
        Self {
            from,
            to,
            output_type: None,
            input_type: None,
        }
    }

    /// Sets logical type compatibility metadata for this edge.
    #[must_use]
    pub fn with_types(
        mut self,
        output_type: impl Into<String>,
        input_type: impl Into<String>,
    ) -> Self {
        self.output_type = Some(output_type.into());
        self.input_type = Some(input_type.into());
        self
    }
}
