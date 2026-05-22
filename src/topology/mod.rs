//! Directed acyclic graph contracts for pipeline topology.

use serde::{Deserialize, Serialize};

use crate::{
    error::{ErrorKind, Recoverability},
    pipeline::{PipelineNodeId, PipelineNodeMetadata},
    TunexError, TunexResult,
};

/// Directed edge between two pipeline nodes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PipelineEdge {
    /// Upstream node.
    pub from: PipelineNodeId,
    /// Downstream node.
    pub to: PipelineNodeId,
}

impl PipelineEdge {
    /// Creates a directed pipeline edge.
    #[must_use]
    pub const fn new(from: PipelineNodeId, to: PipelineNodeId) -> Self {
        Self { from, to }
    }
}

/// Static pipeline graph definition.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PipelineGraph {
    nodes: Vec<PipelineNodeMetadata>,
    edges: Vec<PipelineEdge>,
}

impl PipelineGraph {
    /// Creates an empty pipeline graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Returns graph nodes.
    #[must_use]
    pub fn nodes(&self) -> &[PipelineNodeMetadata] {
        &self.nodes
    }

    /// Returns graph edges.
    #[must_use]
    pub fn edges(&self) -> &[PipelineEdge] {
        &self.edges
    }

    /// Adds a node to the graph.
    ///
    /// # Errors
    ///
    /// Returns an error when another node already uses the same id.
    pub fn add_node(&mut self, node: PipelineNodeMetadata) -> TunexResult<()> {
        if self.nodes.iter().any(|existing| existing.id == node.id) {
            return Err(TunexError::new(
                ErrorKind::Internal,
                format!("duplicate pipeline node id {}", node.id.get()),
            )
            .with_recoverability(Recoverability::Recoverable));
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Adds an edge to the graph.
    ///
    /// # Errors
    ///
    /// Returns an error when either endpoint is missing or the edge would create a cycle.
    pub fn add_edge(&mut self, edge: PipelineEdge) -> TunexResult<()> {
        if !self.contains_node(edge.from) || !self.contains_node(edge.to) {
            return Err(
                TunexError::new(ErrorKind::Internal, "pipeline edge endpoint is missing")
                    .with_recoverability(Recoverability::Recoverable),
            );
        }

        self.edges.push(edge);
        if self.has_cycle() {
            let _ = self.edges.pop();
            return Err(
                TunexError::new(ErrorKind::Internal, "pipeline edge creates a cycle")
                    .with_recoverability(Recoverability::Recoverable),
            );
        }

        Ok(())
    }

    /// Returns true when a node id is present.
    #[must_use]
    pub fn contains_node(&self, id: PipelineNodeId) -> bool {
        self.nodes.iter().any(|node| node.id == id)
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
