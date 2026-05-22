//! Event publisher contract.

use async_trait::async_trait;

use super::TunexEvent;
use crate::TunexResult;

/// Publishes events into an event bus.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publishes one event asynchronously.
    ///
    /// # Errors
    ///
    /// Returns an error when the bounded event queue cannot accept the event.
    async fn publish(&self, event: TunexEvent) -> TunexResult<()>;

    /// Attempts to publish one event without waiting.
    ///
    /// # Errors
    ///
    /// Returns an error when the bounded event queue is full or closed.
    fn try_publish(&self, event: TunexEvent) -> TunexResult<()>;
}
