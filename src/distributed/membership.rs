//! Distributed membership records.

use serde::{Deserialize, Serialize};

use super::{NodeId, NodeRole, NodeState};

/// One node entry in cluster membership.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeMembership {
    /// Node identity.
    pub id: NodeId,
    /// Node role.
    pub role: NodeRole,
    /// Current node state.
    pub state: NodeState,
    /// Monotonic membership epoch observed for this node.
    pub epoch: u64,
}

impl NodeMembership {
    /// Creates an active membership entry.
    #[must_use]
    pub fn new(id: NodeId, role: NodeRole, epoch: u64) -> Self {
        Self {
            id,
            role,
            state: NodeState::Active,
            epoch,
        }
    }

    /// Updates the node state.
    #[must_use]
    pub fn with_state(mut self, state: NodeState) -> Self {
        self.state = state;
        self
    }
}
