//! Distributed coordination trait contracts.

use async_trait::async_trait;

use crate::{channel::BackpressureState, TunexResult};

use super::{
    ClusterMembership, ClusterState, NodeHeartbeat, NodeId, NodeMembership, NodeRole,
    PlacementStrategy, WorkerPlacement,
};

/// Coordinates membership, heartbeat, and placement across distributed nodes.
#[async_trait]
pub trait DistributedCoordinator: Send + Sync {
    /// Registers or updates a node in cluster membership.
    ///
    /// # Errors
    ///
    /// Returns an error when the node cannot be registered.
    async fn register_node(&self, node: NodeMembership) -> TunexResult<()>;

    /// Removes a node from cluster membership.
    ///
    /// # Errors
    ///
    /// Returns an error when the node cannot be removed.
    async fn remove_node(&self, node_id: &NodeId) -> TunexResult<()>;

    /// Records a node heartbeat and returns the accepted node state.
    ///
    /// # Errors
    ///
    /// Returns an error when the heartbeat cannot be recorded.
    async fn heartbeat(&self, heartbeat: NodeHeartbeat) -> TunexResult<super::NodeState>;

    /// Returns a membership snapshot for worker discovery and state sync.
    ///
    /// # Errors
    ///
    /// Returns an error when membership cannot be read.
    async fn membership(&self) -> TunexResult<ClusterMembership>;

    /// Returns active nodes for a role.
    ///
    /// # Errors
    ///
    /// Returns an error when discovery cannot be completed.
    async fn discover_nodes(&self, role: NodeRole) -> TunexResult<Vec<NodeId>>;

    /// Returns the current leader, if known.
    ///
    /// # Errors
    ///
    /// Returns an error when leader state cannot be read.
    async fn leader(&self) -> TunexResult<Option<NodeId>>;

    /// Places a logical worker on a node.
    ///
    /// # Errors
    ///
    /// Returns an error when no valid placement can be selected.
    async fn place_worker(
        &self,
        worker_id: &str,
        role: NodeRole,
        strategy: PlacementStrategy,
    ) -> TunexResult<WorkerPlacement>;

    /// Synchronizes cluster state.
    ///
    /// # Errors
    ///
    /// Returns an error when state cannot be synchronized.
    async fn sync_cluster_state(&self, state: ClusterState) -> TunexResult<()>;

    /// Propagates distributed backpressure from one node.
    ///
    /// # Errors
    ///
    /// Returns an error when backpressure cannot be propagated.
    async fn apply_backpressure(
        &self,
        node_id: &NodeId,
        state: BackpressureState,
    ) -> TunexResult<()>;
}
