//! Pipeline graph validation contracts.

use serde::{Deserialize, Serialize};

use super::PipelineNodeId;

/// Pipeline validation error.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PipelineValidationError {
    /// Another node already uses the same id.
    DuplicateNode {
        /// Duplicate node id.
        node_id: PipelineNodeId,
    },
    /// Edge endpoint is missing.
    MissingEndpoint {
        /// Missing node id.
        node_id: PipelineNodeId,
    },
    /// Edge creates a cycle.
    CycleDetected,
    /// Edge logical input/output types are incompatible.
    IncompatibleTypes {
        /// Upstream node id.
        from: PipelineNodeId,
        /// Downstream node id.
        to: PipelineNodeId,
        /// Produced output type.
        output_type: String,
        /// Required input type.
        input_type: String,
    },
    /// Edge violates configured stage ordering.
    InvalidStageOrder {
        /// Upstream node id.
        from: PipelineNodeId,
        /// Downstream node id.
        to: PipelineNodeId,
    },
    /// Pipeline state transition is invalid.
    InvalidStateTransition {
        /// Previous state.
        from: super::PipelineState,
        /// Requested next state.
        to: super::PipelineState,
    },
}

/// Pipeline validation report.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineValidationReport {
    /// Validation errors.
    pub errors: Vec<PipelineValidationError>,
}

impl PipelineValidationReport {
    /// Returns true when no validation errors were found.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Adds one validation error.
    pub fn push(&mut self, error: PipelineValidationError) {
        self.errors.push(error);
    }
}
