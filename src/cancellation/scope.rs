//! Cancellation scope contract.

use serde::{Deserialize, Serialize};

use super::{CancellationReason, CancellationToken, DrainPolicy};

/// Scoped cancellation handle for jobs, stages, and workers.
#[derive(Clone, Debug)]
pub struct CancellationScope {
    id: String,
    token: CancellationToken,
    drain_policy: DrainPolicy,
}

impl CancellationScope {
    /// Creates a cancellation scope.
    #[must_use]
    pub fn new(id: impl Into<String>, drain_policy: DrainPolicy) -> Self {
        Self {
            id: id.into(),
            token: CancellationToken::new(),
            drain_policy,
        }
    }

    /// Returns the scope identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the scope token.
    #[must_use]
    pub const fn token(&self) -> &CancellationToken {
        &self.token
    }

    /// Returns the drain policy.
    #[must_use]
    pub const fn drain_policy(&self) -> &DrainPolicy {
        &self.drain_policy
    }

    /// Cancels this scope.
    #[must_use]
    pub fn cancel(&self, reason: CancellationReason) -> CancellationToken {
        self.token.cancel_with_reason(reason)
    }

    /// Creates a child scope that cascades from this scope.
    #[must_use]
    pub fn child(&self, id: impl Into<String>, drain_policy: DrainPolicy) -> Self {
        let id = id.into();
        Self {
            id,
            token: self.token.child(CancellationReason::Cascaded {
                parent_scope: self.id.clone(),
            }),
            drain_policy,
        }
    }
}

/// Serializable snapshot of a cancellation scope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CancellationScopeSnapshot {
    /// Scope identifier.
    pub id: String,
    /// Whether cancellation was requested.
    pub cancelled: bool,
    /// Drain policy.
    pub drain_policy: DrainPolicy,
}

impl From<&CancellationScope> for CancellationScopeSnapshot {
    fn from(scope: &CancellationScope) -> Self {
        Self {
            id: scope.id.clone(),
            cancelled: scope.token.is_cancelled(),
            drain_policy: scope.drain_policy.clone(),
        }
    }
}
