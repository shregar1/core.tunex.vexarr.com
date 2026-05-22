//! Runtime engine contract.

use crate::{
    cancellation::ShutdownMode,
    scheduler::{SchedulingDecision, TaskId},
    TunexResult,
};

use super::{RuntimeContext, RuntimeHandle, RuntimeState, RuntimeStats};

/// Runtime engine abstraction.
pub trait RuntimeEngine: Send + Sync {
    /// Starts pipeline execution and returns a runtime handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot start.
    fn start(&self, ctx: RuntimeContext) -> TunexResult<Box<dyn RuntimeHandle>>;

    /// Stops the runtime.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot stop.
    fn stop(&self, mode: ShutdownMode) -> TunexResult<()>;

    /// Returns current runtime state.
    fn state(&self) -> RuntimeState;

    /// Returns runtime statistics.
    fn stats(&self) -> RuntimeStats;

    /// Applies a scheduling decision.
    ///
    /// # Errors
    ///
    /// Returns an error when the decision cannot be applied.
    fn apply_scheduling_decision(
        &self,
        task_id: TaskId,
        decision: SchedulingDecision,
    ) -> TunexResult<()>;
}
