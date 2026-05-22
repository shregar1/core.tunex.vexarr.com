//! Runtime handle contracts.

use crate::{cancellation::ShutdownMode, events::TunexEvent, scheduler::TaskPriority, TunexResult};

use super::{RuntimeState, RuntimeStats};

/// Handle to a running runtime.
pub trait RuntimeHandle: Send + Sync {
    /// Returns runtime state.
    fn state(&self) -> RuntimeState;

    /// Returns runtime stats.
    fn stats(&self) -> RuntimeStats;

    /// Pauses pipeline execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot pause.
    fn pause(&self) -> TunexResult<()>;

    /// Resumes pipeline execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot resume.
    fn resume(&self) -> TunexResult<()>;

    /// Drains in-flight work.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot drain.
    fn drain(&self) -> TunexResult<()>;

    /// Stops the runtime.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot stop.
    fn stop(&self, mode: ShutdownMode) -> TunexResult<()>;

    /// Emits one event through the runtime telemetry path.
    ///
    /// # Errors
    ///
    /// Returns an error when the event cannot be emitted.
    fn emit_event(&self, event: TunexEvent) -> TunexResult<()>;

    /// Submits a lightweight task marker.
    ///
    /// # Errors
    ///
    /// Returns an error when task submission fails.
    fn submit_task(&self, priority: TaskPriority) -> TunexResult<()>;
}
