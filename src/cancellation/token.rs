//! Cancellation token contract.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::CancellationReason;

#[derive(Debug, Default)]
struct CancellationState {
    cancelled: AtomicBool,
}

/// Shared cancellation token for coordinated shutdown.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
    reason: Option<CancellationReason>,
}

impl CancellationToken {
    /// Creates a new cancellation token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new token with a pre-attached reason.
    #[must_use]
    pub fn with_reason(reason: CancellationReason) -> Self {
        Self {
            state: Arc::new(CancellationState::default()),
            reason: Some(reason),
        }
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.state.cancelled.store(true, Ordering::Release);
    }

    /// Requests cancellation and returns a token view carrying the reason.
    #[must_use]
    pub fn cancel_with_reason(&self, reason: CancellationReason) -> Self {
        self.cancel();
        Self {
            state: self.state.clone(),
            reason: Some(reason),
        }
    }

    /// Returns a child token that observes the same cancellation state.
    #[must_use]
    pub fn child(&self, reason: CancellationReason) -> Self {
        Self {
            state: self.state.clone(),
            reason: Some(reason),
        }
    }

    /// Returns true when cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    /// Returns the attached cancellation reason for this token view.
    #[must_use]
    pub const fn reason(&self) -> Option<&CancellationReason> {
        self.reason.as_ref()
    }
}
