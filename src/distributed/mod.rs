//! Distributed coordination contracts for multi-node `TuneX` runtimes.

pub mod cluster;
pub mod coordinator;
pub mod heartbeat;
pub mod membership;
pub mod node;
pub mod placement;
pub mod state;

pub use cluster::{ClusterId, ClusterMembership};
pub use coordinator::DistributedCoordinator;
pub use heartbeat::NodeHeartbeat;
pub use membership::NodeMembership;
pub use node::{NodeId, NodeRole};
pub use placement::{PlacementStrategy, WorkerPlacement};
pub use state::{ClusterState, NodeState};
