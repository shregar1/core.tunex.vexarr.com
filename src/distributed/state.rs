//! Distributed cluster and node state contracts.

use serde::{Deserialize, Serialize};

/// Lifecycle state of a distributed node.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeState {
    /// Node record was created but has not joined.
    #[default]
    Created,
    /// Node is joining the cluster.
    Joining,
    /// Node is active and eligible for work.
    Active,
    /// Node is draining assigned work.
    Draining,
    /// Node missed heartbeat deadlines.
    Unreachable,
    /// Node failed health checks.
    Failed,
    /// Node intentionally left the cluster.
    Left,
}

impl NodeState {
    /// Returns whether the node can receive new work.
    #[must_use]
    pub const fn is_schedulable(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether failure detection should treat the state as unhealthy.
    #[must_use]
    pub const fn is_unhealthy(self) -> bool {
        matches!(self, Self::Unreachable | Self::Failed)
    }
}

/// Lifecycle state of a distributed cluster.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterState {
    /// Cluster was created but is not accepting work.
    #[default]
    Created,
    /// Cluster is discovering members.
    Forming,
    /// Cluster is active.
    Active,
    /// Cluster is active with degraded capacity.
    Degraded,
    /// Cluster is rebalancing placements.
    Rebalancing,
    /// Cluster is draining work.
    Draining,
    /// Cluster stopped cleanly.
    Stopped,
    /// Cluster failed.
    Failed,
}

impl ClusterState {
    /// Returns whether the cluster can accept new work placements.
    #[must_use]
    pub const fn accepts_placements(self) -> bool {
        matches!(self, Self::Active | Self::Degraded | Self::Rebalancing)
    }
}
