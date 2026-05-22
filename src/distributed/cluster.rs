//! Distributed cluster membership contracts.

use std::fmt;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::{ClusterState, NodeId, NodeMembership, NodeRole, NodeState};

/// Stable cluster identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterId(String);

impl ClusterId {
    /// Creates a cluster id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the cluster id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClusterId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Cluster membership snapshot.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClusterMembership {
    /// Cluster identity.
    pub id: ClusterId,
    /// Cluster lifecycle state.
    pub state: ClusterState,
    /// Current leader node, when known.
    pub leader: Option<NodeId>,
    /// Monotonic membership epoch.
    pub epoch: u64,
    /// Known node membership entries.
    pub nodes: SmallVec<[NodeMembership; 8]>,
}

impl ClusterMembership {
    /// Creates an empty cluster membership snapshot.
    #[must_use]
    pub fn new(id: ClusterId) -> Self {
        Self {
            id,
            state: ClusterState::Created,
            leader: None,
            epoch: 0,
            nodes: SmallVec::new(),
        }
    }

    /// Sets cluster state.
    #[must_use]
    pub fn with_state(mut self, state: ClusterState) -> Self {
        self.state = state;
        self
    }

    /// Registers or updates one node membership entry.
    pub fn register_node(&mut self, node: NodeMembership) {
        if let Some(existing) = self.nodes.iter_mut().find(|entry| entry.id == node.id) {
            *existing = node;
        } else {
            self.nodes.push(node);
        }
        self.epoch = self.epoch.saturating_add(1);
    }

    /// Removes one node from membership and returns whether it existed.
    pub fn remove_node(&mut self, node_id: &NodeId) -> bool {
        let before = self.nodes.len();
        self.nodes.retain(|node| &node.id != node_id);
        let removed = self.nodes.len() != before;
        if removed {
            self.epoch = self.epoch.saturating_add(1);
            if self.leader.as_ref() == Some(node_id) {
                self.leader = None;
            }
        }
        removed
    }

    /// Sets the current leader.
    pub fn set_leader(&mut self, leader: NodeId) {
        self.leader = Some(leader);
        self.epoch = self.epoch.saturating_add(1);
    }

    /// Returns whether the node is a known member.
    #[must_use]
    pub fn contains(&self, node_id: &NodeId) -> bool {
        self.nodes.iter().any(|node| &node.id == node_id)
    }

    /// Finds active nodes for a role.
    #[must_use]
    pub fn active_nodes_by_role(&self, role: NodeRole) -> SmallVec<[NodeId; 4]> {
        self.nodes
            .iter()
            .filter(|node| node.role == role && node.state == NodeState::Active)
            .map(|node| node.id.clone())
            .collect()
    }
}
