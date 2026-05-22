//! Bounded channel contracts.

use std::{fmt, time::Duration};

use serde::{Deserialize, Serialize};

use super::{BackpressurePolicy, ChannelStats};
use crate::TunexResult;

/// Stable channel identifier.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelId(String);

impl ChannelId {
    /// Creates a channel id.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Logical channel kind.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelKind {
    /// Dataset to tokenizer transfer.
    DatasetToTokenizer,
    /// Tokenizer to packer transfer.
    TokenizerToPacker,
    /// Packer to trainer transfer.
    PackerToTrainer,
    /// Trainer to checkpoint transfer.
    TrainerToCheckpoint,
    /// Event channel.
    Events,
    /// Metrics channel.
    Metrics,
    /// Custom channel.
    Custom,
}

/// Outcome of a send operation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SendOutcome {
    /// Item was accepted.
    Sent,
    /// Sender should throttle before retrying.
    Throttled {
        /// Suggested delay in milliseconds.
        retry_after_millis: u64,
    },
    /// Item was dropped by policy.
    Dropped {
        /// Human-readable drop reason.
        reason: String,
    },
    /// Item was spilled by policy.
    Spilled {
        /// Spill target descriptor.
        target: String,
    },
    /// Operation timed out.
    TimedOut,
    /// Operation was cancelled.
    Cancelled,
}

/// Outcome of a receive operation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReceiveOutcome<T> {
    /// Item was received.
    Received(T),
    /// Channel was empty.
    Empty,
    /// Operation timed out.
    TimedOut,
    /// Operation was cancelled.
    Cancelled,
    /// Channel was closed.
    Closed,
}

/// Bounded queue abstraction for pipeline transfers.
pub trait BoundedChannel<T>: Send + Sync {
    /// Returns channel id.
    fn id(&self) -> &ChannelId;

    /// Returns channel kind.
    fn kind(&self) -> ChannelKind;

    /// Returns configured bounded capacity.
    fn capacity(&self) -> usize;

    /// Returns active backpressure policy.
    fn policy(&self) -> BackpressurePolicy;

    /// Returns queue stats.
    fn stats(&self) -> ChannelStats;

    /// Attempts to send immediately.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel cannot evaluate or apply send semantics.
    fn try_send(&self, item: T) -> TunexResult<SendOutcome>;

    /// Attempts to send with a timeout.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel cannot evaluate or apply send semantics.
    fn send_timeout(&self, item: T, timeout: Duration) -> TunexResult<SendOutcome>;

    /// Attempts to receive immediately.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel cannot evaluate or apply receive semantics.
    fn try_receive(&self) -> TunexResult<ReceiveOutcome<T>>;

    /// Attempts to receive with a timeout.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel cannot evaluate or apply receive semantics.
    fn receive_timeout(&self, timeout: Duration) -> TunexResult<ReceiveOutcome<T>>;
}
