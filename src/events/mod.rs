//! Event bus contracts for observability and orchestration.

use async_trait::async_trait;

use crate::{types::PipelineEvent, TunexResult};

/// Publishes events into the event bus.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publishes one event.
    ///
    /// # Errors
    ///
    /// Returns an error when the event bus cannot accept the event.
    async fn publish(&self, event: PipelineEvent) -> TunexResult<()>;
}

/// Receives events from the event bus.
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    /// Handles one event.
    ///
    /// # Errors
    ///
    /// Returns an error when the subscriber cannot process the event.
    async fn on_event(&self, event: &PipelineEvent) -> TunexResult<()>;
}

/// Event bus abstraction connecting publishers and subscribers.
#[async_trait]
pub trait PipelineEventBus: EventPublisher {
    /// Registers a subscriber with the bus.
    ///
    /// # Errors
    ///
    /// Returns an error when the subscriber cannot be registered.
    async fn subscribe(&self, subscriber: Box<dyn EventSubscriber>) -> TunexResult<()>;
}
