//! Backpressure policy contracts.

use serde::{Deserialize, Serialize};

/// Policy applied when a bounded channel is under pressure.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BackpressurePolicy {
    /// Block producer until capacity is available.
    #[default]
    Block,
    /// Drop oldest queued item.
    DropOldest,
    /// Drop newest incoming item.
    DropNewest,
    /// Drop lowest-priority queued item.
    DropLowPriority,
    /// Spill excess data to disk or external storage.
    SpillToDisk,
    /// Adapt behavior based on pressure and lag.
    Adaptive,
}
