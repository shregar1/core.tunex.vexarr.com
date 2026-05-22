//! Distributed heartbeat contracts.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    channel::{BackpressureState, QueuePressure},
    events::EventTimestamp,
};

use super::{NodeId, NodeRole, NodeState};

/// Heartbeat emitted by one distributed node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeHeartbeat {
    /// Node identity.
    pub node_id: NodeId,
    /// Node role.
    pub role: NodeRole,
    /// Node state.
    pub state: NodeState,
    /// Heartbeat timestamp.
    pub timestamp: EventTimestamp,
    /// Membership epoch known by this node.
    pub membership_epoch: u64,
    /// Active worker count on this node.
    pub active_workers: usize,
    /// Current aggregate queue pressure.
    pub queue_pressure: QueuePressure,
    /// Current aggregate distributed backpressure.
    pub backpressure: BackpressureState,
}

impl NodeHeartbeat {
    /// Creates a heartbeat for an active node.
    #[must_use]
    pub fn new(node_id: NodeId, role: NodeRole) -> Self {
        Self {
            node_id,
            role,
            state: NodeState::Active,
            timestamp: EventTimestamp::now(),
            membership_epoch: 0,
            active_workers: 0,
            queue_pressure: QueuePressure::Low,
            backpressure: BackpressureState::default(),
        }
    }

    /// Sets node state.
    #[must_use]
    pub const fn with_state(mut self, state: NodeState) -> Self {
        self.state = state;
        self
    }

    /// Sets membership epoch.
    #[must_use]
    pub const fn with_membership_epoch(mut self, epoch: u64) -> Self {
        self.membership_epoch = epoch;
        self
    }

    /// Sets active worker count.
    #[must_use]
    pub const fn with_active_workers(mut self, active_workers: usize) -> Self {
        self.active_workers = active_workers;
        self
    }

    /// Sets aggregate backpressure state.
    #[must_use]
    pub fn with_backpressure(mut self, backpressure: BackpressureState) -> Self {
        self.queue_pressure = backpressure.pressure;
        self.backpressure = backpressure;
        self
    }

    /// Returns whether this heartbeat is older than the timeout.
    #[must_use]
    pub fn is_stale_at(&self, now: EventTimestamp, timeout: Duration) -> bool {
        let elapsed = now
            .as_unix_millis()
            .saturating_sub(self.timestamp.as_unix_millis());
        elapsed > i64::try_from(timeout.as_millis()).unwrap_or(i64::MAX)
    }
}
