//! Channel error contracts.

use thiserror::Error;

use super::ChannelId;

/// Channel-level error.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChannelError {
    /// Channel is closed.
    #[error("channel closed: {0}")]
    Closed(ChannelId),
    /// Channel operation timed out.
    #[error("channel timed out: {0}")]
    Timeout(ChannelId),
    /// Channel operation was cancelled.
    #[error("channel cancelled: {0}")]
    Cancelled(ChannelId),
    /// Channel capacity is invalid.
    #[error("invalid channel capacity for {channel_id}: {capacity}")]
    InvalidCapacity {
        /// Channel id.
        channel_id: ChannelId,
        /// Invalid capacity.
        capacity: usize,
    },
    /// Channel exceeded memory limit.
    #[error("channel memory limit exceeded: {0}")]
    MemoryLimitExceeded(ChannelId),
}
