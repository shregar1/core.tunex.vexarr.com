//! Event bus contract.

use async_trait::async_trait;

use super::{EventFilter, EventPublisher, EventSubscriber};
use crate::TunexResult;

/// Event bus queue and subscriber statistics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EventBusStats {
    /// Configured bounded queue capacity.
    pub capacity: usize,
    /// Current queued event count when observable.
    pub queued: usize,
    /// Current subscriber count.
    pub subscribers: usize,
}

/// Event bus abstraction connecting publishers and subscribers.
#[async_trait]
pub trait EventBus: EventPublisher {
    /// Registers a subscriber with a filter.
    ///
    /// # Errors
    ///
    /// Returns an error when the subscriber cannot be registered.
    async fn subscribe(
        &self,
        filter: EventFilter,
        subscriber: Box<dyn EventSubscriber>,
    ) -> TunexResult<()>;

    /// Returns bounded queue capacity.
    fn capacity(&self) -> usize;

    /// Returns current queued event count when observable.
    fn queued(&self) -> usize;

    /// Returns event bus stats.
    fn stats(&self) -> EventBusStats;
}
