//! Event bus and telemetry contracts.

pub mod bus;
pub mod event;
pub mod filter;
pub mod publisher;
pub mod subscriber;

pub use bus::{EventBus, EventBusStats};
pub use event::{
    EventId, EventKind, EventMetadata, EventMetadataEntry, EventSeverity, EventSource,
    EventTimestamp, TunexEvent,
};
pub use filter::EventFilter;
pub use publisher::EventPublisher;
pub use subscriber::EventSubscriber;

/// Backward-compatible alias for the core event bus contract.
pub type PipelineEventBus = dyn EventBus;
