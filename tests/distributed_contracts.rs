use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use tunex_core::{
    channel::{BackpressureState, QueuePressure},
    distributed::{
        ClusterId, ClusterMembership, ClusterState, DistributedCoordinator, NodeHeartbeat, NodeId,
        NodeMembership, NodeRole, NodeState, PlacementStrategy, WorkerPlacement,
    },
    error::ErrorKind,
    events::EventTimestamp,
    TunexError, TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

struct TestCoordinator {
    membership: Mutex<ClusterMembership>,
}

#[async_trait]
impl DistributedCoordinator for TestCoordinator {
    async fn register_node(&self, node: NodeMembership) -> TunexResult<()> {
        self.membership.lock().unwrap().register_node(node);
        Ok(())
    }

    async fn remove_node(&self, node_id: &NodeId) -> TunexResult<()> {
        self.membership.lock().unwrap().remove_node(node_id);
        Ok(())
    }

    async fn heartbeat(&self, heartbeat: NodeHeartbeat) -> TunexResult<NodeState> {
        Ok(heartbeat.state)
    }

    async fn membership(&self) -> TunexResult<ClusterMembership> {
        Ok(self.membership.lock().unwrap().clone())
    }

    async fn discover_nodes(&self, role: NodeRole) -> TunexResult<Vec<NodeId>> {
        Ok(self
            .membership
            .lock()
            .unwrap()
            .active_nodes_by_role(role)
            .into_iter()
            .collect())
    }

    async fn leader(&self) -> TunexResult<Option<NodeId>> {
        Ok(self.membership.lock().unwrap().leader.clone())
    }

    async fn place_worker(
        &self,
        worker_id: &str,
        role: NodeRole,
        strategy: PlacementStrategy,
    ) -> TunexResult<WorkerPlacement> {
        let membership = self.membership.lock().unwrap();
        let node_id = membership
            .active_nodes_by_role(role)
            .into_iter()
            .next()
            .ok_or_else(|| TunexError::new(ErrorKind::Unsupported, "no active node for role"))?;
        Ok(WorkerPlacement::new(worker_id, node_id, role, strategy))
    }

    async fn sync_cluster_state(&self, state: ClusterState) -> TunexResult<()> {
        self.membership.lock().unwrap().state = state;
        Ok(())
    }

    async fn apply_backpressure(
        &self,
        _node_id: &NodeId,
        _state: BackpressureState,
    ) -> TunexResult<()> {
        Ok(())
    }
}

#[test]
fn distributed_contract_types_are_send_sync_and_object_safe() {
    assert_send_sync::<NodeId>();
    assert_send_sync::<NodeRole>();
    assert_send_sync::<NodeState>();
    assert_send_sync::<ClusterId>();
    assert_send_sync::<ClusterState>();
    assert_send_sync::<ClusterMembership>();
    assert_send_sync::<NodeMembership>();
    assert_send_sync::<NodeHeartbeat>();
    assert_send_sync::<PlacementStrategy>();
    assert_send_sync::<WorkerPlacement>();
    assert_object_safe::<dyn DistributedCoordinator>();

    let _coordinator = TestCoordinator {
        membership: Mutex::new(ClusterMembership::new(ClusterId::new("cluster-a"))),
    };
}

#[test]
fn cluster_membership_registers_discovers_and_removes_nodes() {
    let trainer = NodeId::new("trainer-0");
    let dataset = NodeId::new("dataset-0");
    let mut membership =
        ClusterMembership::new(ClusterId::new("cluster-a")).with_state(ClusterState::Active);

    membership.register_node(NodeMembership::new(trainer.clone(), NodeRole::Trainer, 1));
    membership.register_node(NodeMembership::new(dataset.clone(), NodeRole::Dataset, 1));
    membership.set_leader(trainer.clone());

    assert_eq!(membership.epoch, 3);
    assert!(membership.contains(&trainer));
    assert_eq!(membership.leader.as_ref(), Some(&trainer));
    assert_eq!(
        membership
            .active_nodes_by_role(NodeRole::Trainer)
            .as_slice(),
        std::slice::from_ref(&trainer)
    );

    assert!(membership.remove_node(&trainer));
    assert_eq!(membership.leader, None);
    assert!(!membership.contains(&trainer));
    assert!(membership.contains(&dataset));
}

#[test]
fn node_heartbeat_carries_failure_and_backpressure_signals() {
    let pressure = BackpressureState::from_depth(95, 100);
    let heartbeat = NodeHeartbeat::new(NodeId::new("node-1"), NodeRole::Worker)
        .with_membership_epoch(7)
        .with_active_workers(12)
        .with_backpressure(pressure);

    assert_eq!(heartbeat.state, NodeState::Active);
    assert_eq!(heartbeat.membership_epoch, 7);
    assert_eq!(heartbeat.active_workers, 12);
    assert_eq!(heartbeat.queue_pressure, QueuePressure::High);
    assert!(!heartbeat.is_stale_at(heartbeat.timestamp, Duration::from_secs(1)));

    let later = EventTimestamp::from_unix_millis(heartbeat.timestamp.as_unix_millis() + 2_000);
    assert!(heartbeat.is_stale_at(later, Duration::from_secs(1)));
}

#[test]
fn placement_strategy_models_future_multi_node_scheduling_modes() {
    let placement = WorkerPlacement::new(
        "trainer-worker-0",
        NodeId::new("gpu-node-0"),
        NodeRole::Trainer,
        PlacementStrategy::GpuLocal,
    )
    .with_queue_pressure(QueuePressure::Medium);

    assert_eq!(placement.worker_id, "trainer-worker-0");
    assert_eq!(placement.node_id.as_str(), "gpu-node-0");
    assert_eq!(placement.role, NodeRole::Trainer);
    assert_eq!(placement.strategy, PlacementStrategy::GpuLocal);
    assert_eq!(placement.queue_pressure, QueuePressure::Medium);

    assert!(NodeState::Active.is_schedulable());
    assert!(NodeState::Unreachable.is_unhealthy());
    assert!(ClusterState::Degraded.accepts_placements());
    assert!(!ClusterState::Stopped.accepts_placements());
}
