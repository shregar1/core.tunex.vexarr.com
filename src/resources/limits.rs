//! Resource limit and reservation contracts.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::events::EventTimestamp;

use super::{ResourceId, ResourceKind};

/// Policy for allowing resource oversubscription.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OversubscriptionPolicy {
    /// Reject placements that exceed known capacity.
    #[default]
    Deny,
    /// Allow bounded oversubscription.
    Allow,
    /// Allow oversubscription only for retryable/background work.
    BackgroundOnly,
    /// Allow runtime implementations to adapt dynamically.
    Adaptive,
}

/// Limit applied to one resource kind or resource id.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLimit {
    /// Resource kind affected by this limit.
    pub kind: ResourceKind,
    /// Specific resource id, when the limit is scoped.
    pub resource_id: Option<ResourceId>,
    /// Maximum logical units, such as CPU cores.
    pub max_units: Option<u64>,
    /// Maximum bytes, such as RAM, VRAM, or disk space.
    pub max_bytes: Option<u64>,
    /// Maximum bandwidth in bytes per second.
    pub max_bandwidth_bytes_per_sec: Option<u64>,
    /// Preferred CPU cores for CPU-scoped limits.
    pub cpu_affinity: SmallVec<[usize; 16]>,
    /// NUMA node hint.
    pub numa_node: Option<usize>,
    /// Oversubscription policy.
    pub oversubscription: OversubscriptionPolicy,
}

impl ResourceLimit {
    /// Creates a limit for a resource kind.
    #[must_use]
    pub fn new(kind: ResourceKind) -> Self {
        Self {
            kind,
            resource_id: None,
            max_units: None,
            max_bytes: None,
            max_bandwidth_bytes_per_sec: None,
            cpu_affinity: SmallVec::new(),
            numa_node: None,
            oversubscription: OversubscriptionPolicy::Deny,
        }
    }

    /// Scopes the limit to a resource id.
    #[must_use]
    pub fn for_resource(mut self, resource_id: ResourceId) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Sets maximum logical units.
    #[must_use]
    pub fn with_max_units(mut self, max_units: u64) -> Self {
        self.max_units = Some(max_units);
        self
    }

    /// Sets maximum bytes.
    #[must_use]
    pub fn with_max_bytes(mut self, max_bytes: u64) -> Self {
        self.max_bytes = Some(max_bytes);
        self
    }

    /// Sets maximum bandwidth in bytes per second.
    #[must_use]
    pub fn with_max_bandwidth(mut self, max_bandwidth_bytes_per_sec: u64) -> Self {
        self.max_bandwidth_bytes_per_sec = Some(max_bandwidth_bytes_per_sec);
        self
    }

    /// Sets oversubscription policy.
    #[must_use]
    pub fn with_oversubscription(mut self, policy: OversubscriptionPolicy) -> Self {
        self.oversubscription = policy;
        self
    }
}

/// Reservation held for future work placement.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceReservation {
    /// Stable reservation identifier.
    pub reservation_id: String,
    /// Reserved resource id.
    pub resource_id: ResourceId,
    /// Reserved resource kind.
    pub kind: ResourceKind,
    /// Reserved logical units.
    pub units: Option<u64>,
    /// Reserved bytes.
    pub bytes: Option<u64>,
    /// Reservation expiration timestamp.
    pub expires_at: Option<EventTimestamp>,
    /// Oversubscription policy used for this reservation.
    pub oversubscription: OversubscriptionPolicy,
}

impl ResourceReservation {
    /// Creates a resource reservation.
    #[must_use]
    pub fn new(
        reservation_id: impl Into<String>,
        resource_id: ResourceId,
        kind: ResourceKind,
    ) -> Self {
        Self {
            reservation_id: reservation_id.into(),
            resource_id,
            kind,
            units: None,
            bytes: None,
            expires_at: None,
            oversubscription: OversubscriptionPolicy::Deny,
        }
    }

    /// Sets reserved units.
    #[must_use]
    pub fn with_units(mut self, units: u64) -> Self {
        self.units = Some(units);
        self
    }

    /// Sets reserved bytes.
    #[must_use]
    pub fn with_bytes(mut self, bytes: u64) -> Self {
        self.bytes = Some(bytes);
        self
    }

    /// Sets expiration.
    #[must_use]
    pub fn expires_at(mut self, expires_at: EventTimestamp) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}
