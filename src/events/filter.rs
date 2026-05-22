//! Event filtering contracts.

use serde::{Deserialize, Serialize};

use super::{EventKind, EventSeverity, TunexEvent};

/// Event subscription filter.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventFilter {
    /// Accepted event kinds. Empty accepts all.
    pub kinds: Vec<EventKind>,
    /// Accepted severities. Empty accepts all.
    pub severities: Vec<EventSeverity>,
    /// Optional module name.
    pub module: Option<String>,
    /// Optional stage name.
    pub stage: Option<String>,
    /// Optional job id.
    pub job_id: Option<String>,
    /// Optional worker id.
    pub worker_id: Option<String>,
}

impl EventFilter {
    /// Creates an event filter that accepts all events.
    #[must_use]
    pub fn all() -> Self {
        Self::default()
    }

    /// Creates an event filter for one kind.
    #[must_use]
    pub fn kind(kind: EventKind) -> Self {
        Self {
            kinds: vec![kind],
            ..Self::default()
        }
    }

    /// Returns true when an event matches this filter.
    #[must_use]
    pub fn matches(&self, event: &TunexEvent) -> bool {
        (self.kinds.is_empty() || self.kinds.contains(&event.kind))
            && (self.severities.is_empty() || self.severities.contains(&event.severity))
            && self
                .module
                .as_ref()
                .map_or(true, |module| event.source.module.as_ref() == Some(module))
            && self
                .stage
                .as_ref()
                .map_or(true, |stage| event.source.stage.as_ref() == Some(stage))
            && self
                .job_id
                .as_ref()
                .map_or(true, |job_id| event.source.job_id.as_ref() == Some(job_id))
            && self.worker_id.as_ref().map_or(true, |worker_id| {
                event.source.worker_id.as_ref() == Some(worker_id)
            })
    }
}
