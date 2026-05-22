//! Queue pressure and backpressure state contracts.

use serde::{Deserialize, Serialize};

/// Queue pressure level.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueuePressure {
    /// Queue is mostly empty.
    #[default]
    Low,
    /// Queue has moderate utilization.
    Medium,
    /// Queue is close to capacity.
    High,
    /// Queue is full.
    Full,
}

/// Backpressure state snapshot.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackpressureState {
    /// Current pressure level.
    pub pressure: QueuePressure,
    /// Whether producers should throttle.
    pub producer_throttled: bool,
    /// Consumer lag in items.
    pub consumer_lag: usize,
    /// Optional suggested producer delay in milliseconds.
    pub suggested_delay_millis: Option<u64>,
}

impl BackpressureState {
    /// Creates a pressure state from queue length and capacity.
    #[must_use]
    pub fn from_depth(depth: usize, capacity: usize) -> Self {
        let pressure = if capacity == 0 || depth >= capacity {
            QueuePressure::Full
        } else if depth.saturating_mul(100) / capacity >= 90 {
            QueuePressure::High
        } else if depth.saturating_mul(100) / capacity >= 50 {
            QueuePressure::Medium
        } else {
            QueuePressure::Low
        };

        Self {
            pressure,
            producer_throttled: matches!(pressure, QueuePressure::High | QueuePressure::Full),
            consumer_lag: depth,
            suggested_delay_millis: matches!(pressure, QueuePressure::High | QueuePressure::Full)
                .then_some(1),
        }
    }
}
