//! Shutdown guard contract.

use std::time::{Duration, Instant};

use super::{CancellationReason, CancellationScope, CancellationToken, DrainPolicy};

/// Guard representing an in-progress shutdown.
#[derive(Clone, Debug)]
pub struct ShutdownGuard {
    scope: CancellationScope,
    started_at: Instant,
    deadline: Option<Instant>,
}

impl ShutdownGuard {
    /// Starts shutdown for a scope.
    #[must_use]
    pub fn new(scope: CancellationScope) -> Self {
        let started_at = Instant::now();
        let deadline = scope
            .drain_policy()
            .timeout_millis
            .map(|millis| started_at + Duration::from_millis(millis));
        Self {
            scope,
            started_at,
            deadline,
        }
    }

    /// Returns the shutdown scope.
    #[must_use]
    pub const fn scope(&self) -> &CancellationScope {
        &self.scope
    }

    /// Returns the drain policy.
    #[must_use]
    pub const fn drain_policy(&self) -> &DrainPolicy {
        self.scope.drain_policy()
    }

    /// Returns elapsed shutdown time.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Returns true when the shutdown timeout has elapsed.
    #[must_use]
    pub fn timed_out(&self) -> bool {
        self.deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
    }

    /// Cancels the scope attached to this guard.
    #[must_use]
    pub fn cancel(&self, reason: CancellationReason) -> CancellationToken {
        self.scope.cancel(reason)
    }
}
