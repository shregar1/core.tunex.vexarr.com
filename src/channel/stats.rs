//! Channel statistics contracts.

use serde::{Deserialize, Serialize};

use super::BackpressureState;

/// Channel queue and throughput statistics.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChannelStats {
    /// Configured queue capacity.
    pub capacity: usize,
    /// Current queue depth.
    pub depth: usize,
    /// Optional memory limit in bytes.
    pub memory_limit_bytes: Option<usize>,
    /// Current estimated memory usage in bytes.
    pub memory_used_bytes: Option<usize>,
    /// Total sent items.
    pub sent: u64,
    /// Total received items.
    pub received: u64,
    /// Total dropped items.
    pub dropped: u64,
    /// Total timed-out operations.
    pub timeouts: u64,
    /// Backpressure state.
    pub backpressure: BackpressureState,
}

impl ChannelStats {
    /// Creates stats from capacity and depth.
    #[must_use]
    pub fn from_depth(capacity: usize, depth: usize) -> Self {
        Self {
            capacity,
            depth,
            backpressure: BackpressureState::from_depth(depth, capacity),
            ..Self::default()
        }
    }
}
