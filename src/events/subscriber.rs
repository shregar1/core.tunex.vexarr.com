//! Event subscriber contract.

use async_trait::async_trait;

use super::{EventFilter, TunexEvent};
use crate::TunexResult;

/// Receives events from an event bus.
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    /// Returns the subscriber filter.
    fn filter(&self) -> EventFilter {
        EventFilter::all()
    }

    /// Handles one event.
    ///
    /// # Errors
    ///
    /// Returns an error when the subscriber cannot process the event.
    async fn on_event(&self, event: &TunexEvent) -> TunexResult<()>;
}
