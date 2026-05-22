//! Runtime executor contract.

use async_trait::async_trait;

use crate::{
    cancellation::CancellationToken,
    scheduler::{SchedulingDecision, TaskId},
    TunexResult,
};

/// Executes scheduled runtime work.
#[async_trait]
pub trait RuntimeExecutor: Send + Sync {
    /// Executes one scheduled task.
    ///
    /// # Errors
    ///
    /// Returns an error when task execution fails or cancellation is observed.
    async fn execute(
        &self,
        task_id: TaskId,
        decision: SchedulingDecision,
        cancellation: CancellationToken,
    ) -> TunexResult<()>;
}
