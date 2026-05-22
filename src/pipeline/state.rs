//! Pipeline lifecycle state contracts.

use serde::{Deserialize, Serialize};

/// Pipeline lifecycle state.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineState {
    /// Graph has been created.
    #[default]
    Created,
    /// Graph has been validated.
    Validated,
    /// Graph is running.
    Running,
    /// Graph is draining in-flight work.
    Draining,
    /// Graph completed successfully.
    Completed,
    /// Graph failed.
    Failed,
    /// Graph was cancelled.
    Cancelled,
}

impl PipelineState {
    /// Returns true when transition from `self` to `next` is valid.
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Created, Self::Validated | Self::Cancelled)
                | (Self::Validated, Self::Running | Self::Cancelled)
                | (
                    Self::Running,
                    Self::Draining | Self::Completed | Self::Failed | Self::Cancelled
                )
                | (
                    Self::Draining,
                    Self::Completed | Self::Failed | Self::Cancelled
                )
        )
    }
}
