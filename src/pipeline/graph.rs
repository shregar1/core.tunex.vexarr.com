//! Pipeline DAG graph contracts.

use serde::{Deserialize, Serialize};

use super::{
    PipelineEdge, PipelineNode, PipelineNodeId, PipelineState, PipelineValidationError,
    PipelineValidationReport,
};
use crate::{
    error::{ErrorKind, Recoverability},
    TunexError, TunexResult,
};

/// Static pipeline DAG definition.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineGraph {
    nodes: Vec<PipelineNode>,
    edges: Vec<PipelineEdge>,
    state: PipelineState,
}

impl PipelineGraph {
    /// Creates an empty pipeline graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            state: PipelineState::Created,
        }
    }

    /// Returns graph nodes.
    #[must_use]
    pub fn nodes(&self) -> &[PipelineNode] {
        &self.nodes
    }

    /// Returns graph edges.
    #[must_use]
    pub fn edges(&self) -> &[PipelineEdge] {
        &self.edges
    }

    /// Returns pipeline state.
    #[must_use]
    pub const fn state(&self) -> PipelineState {
        self.state
    }

    /// Transitions pipeline state.
    ///
    /// # Errors
    ///
    /// Returns an error when the requested transition is invalid.
    pub fn transition_to(&mut self, next: PipelineState) -> TunexResult<()> {
        if !self.state.can_transition_to(next) {
            return Err(pipeline_error(
                &PipelineValidationError::InvalidStateTransition {
                    from: self.state,
                    to: next,
                },
            ));
        }
        self.state = next;
        Ok(())
    }

    /// Adds a node to the graph.
    ///
    /// # Errors
    ///
    /// Returns an error when another node already uses the same id.
    pub fn add_node(&mut self, node: PipelineNode) -> TunexResult<()> {
        if self.contains_node(node.id) {
            return Err(pipeline_error(&PipelineValidationError::DuplicateNode {
                node_id: node.id,
            }));
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Adds an edge to the graph.
    ///
    /// # Errors
    ///
    /// Returns an error when validation fails.
    pub fn add_edge(&mut self, edge: PipelineEdge) -> TunexResult<()> {
        validate_edge_endpoints(self, &edge).map_err(|error| pipeline_error(&error))?;
        validate_edge_types(self, &edge).map_err(|error| pipeline_error(&error))?;
        validate_stage_order(self, &edge).map_err(|error| pipeline_error(&error))?;

        self.edges.push(edge);
        if self.has_cycle() {
            let _ = self.edges.pop();
            return Err(pipeline_error(&PipelineValidationError::CycleDetected));
        }

        Ok(())
    }

    /// Returns true when a node id is present.
    #[must_use]
    pub fn contains_node(&self, id: PipelineNodeId) -> bool {
        self.nodes.iter().any(|node| node.id == id)
    }

    /// Returns a node by id.
    #[must_use]
    pub fn node(&self, id: PipelineNodeId) -> Option<&PipelineNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    /// Validates the full graph.
    #[must_use]
    pub fn validate(&self) -> PipelineValidationReport {
        let mut report = PipelineValidationReport::default();

        for edge in &self.edges {
            if let Err(error) = validate_edge_endpoints(self, edge) {
                report.push(error);
                continue;
            }
            if let Err(error) = validate_edge_types(self, edge) {
                report.push(error);
            }
            if let Err(error) = validate_stage_order(self, edge) {
                report.push(error);
            }
        }
        if self.has_cycle() {
            report.push(PipelineValidationError::CycleDetected);
        }

        report
    }

    fn has_cycle(&self) -> bool {
        self.nodes
            .iter()
            .any(|node| self.reaches(node.id, node.id, &mut Vec::new()))
    }

    fn reaches(
        &self,
        start: PipelineNodeId,
        target: PipelineNodeId,
        visited: &mut Vec<PipelineNodeId>,
    ) -> bool {
        if visited.contains(&start) {
            return false;
        }
        visited.push(start);

        self.edges
            .iter()
            .filter(|edge| edge.from == start)
            .any(|edge| edge.to == target || self.reaches(edge.to, target, visited))
    }
}

fn validate_edge_endpoints(
    graph: &PipelineGraph,
    edge: &PipelineEdge,
) -> Result<(), PipelineValidationError> {
    if !graph.contains_node(edge.from) {
        return Err(PipelineValidationError::MissingEndpoint { node_id: edge.from });
    }
    if !graph.contains_node(edge.to) {
        return Err(PipelineValidationError::MissingEndpoint { node_id: edge.to });
    }
    Ok(())
}

fn validate_edge_types(
    graph: &PipelineGraph,
    edge: &PipelineEdge,
) -> Result<(), PipelineValidationError> {
    let from = graph.node(edge.from).expect("validated endpoint");
    let to = graph.node(edge.to).expect("validated endpoint");
    let output_type = edge.output_type.as_ref().or(from.output_type.as_ref());
    let input_type = edge.input_type.as_ref().or(to.input_type.as_ref());

    match (output_type, input_type) {
        (Some(output_type), Some(input_type)) if output_type != input_type => {
            Err(PipelineValidationError::IncompatibleTypes {
                from: edge.from,
                to: edge.to,
                output_type: output_type.clone(),
                input_type: input_type.clone(),
            })
        }
        _ => Ok(()),
    }
}

fn validate_stage_order(
    graph: &PipelineGraph,
    edge: &PipelineEdge,
) -> Result<(), PipelineValidationError> {
    let from = graph.node(edge.from).expect("validated endpoint");
    let to = graph.node(edge.to).expect("validated endpoint");

    if from.stage.can_precede(to.stage) {
        Ok(())
    } else {
        Err(PipelineValidationError::InvalidStageOrder {
            from: edge.from,
            to: edge.to,
        })
    }
}

fn pipeline_error(error: &PipelineValidationError) -> TunexError {
    TunexError::new(ErrorKind::Internal, format!("{error:?}"))
        .with_recoverability(Recoverability::Recoverable)
}
