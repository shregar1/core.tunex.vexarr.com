//! Bounded channel and backpressure contracts.

pub mod bounded;
pub mod error;
pub mod policy;
pub mod pressure;
pub mod stats;

pub use bounded::{BoundedChannel, ChannelId, ChannelKind, ReceiveOutcome, SendOutcome};
pub use error::ChannelError;
pub use policy::BackpressurePolicy;
pub use pressure::{BackpressureState, QueuePressure};
pub use stats::ChannelStats;
