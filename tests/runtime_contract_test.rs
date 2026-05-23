use std::sync::Mutex;

use tunex_core::{
    cancellation::ShutdownMode,
    events::{EventId, EventKind, EventSeverity, TunexEvent},
    runtime::{RuntimeHandle, RuntimeState, RuntimeStats},
    scheduler::TaskPriority,
    TunexResult,
};

#[derive(Default)]
struct MockRuntimeHandle {
    state: Mutex<RuntimeState>,
    stats: Mutex<RuntimeStats>,
}

impl MockRuntimeHandle {
    fn transition_to(&self, next: RuntimeState) {
        *self.state.lock().expect("runtime state mutex") = next;
    }
}

impl RuntimeHandle for MockRuntimeHandle {
    fn state(&self) -> RuntimeState {
        *self.state.lock().expect("runtime state mutex")
    }

    fn stats(&self) -> RuntimeStats {
        self.stats.lock().expect("runtime stats mutex").clone()
    }

    fn pause(&self) -> TunexResult<()> {
        self.transition_to(RuntimeState::Paused);
        Ok(())
    }

    fn resume(&self) -> TunexResult<()> {
        self.transition_to(RuntimeState::Running);
        Ok(())
    }

    fn drain(&self) -> TunexResult<()> {
        self.transition_to(RuntimeState::Draining);
        Ok(())
    }

    fn stop(&self, _mode: ShutdownMode) -> TunexResult<()> {
        self.transition_to(RuntimeState::Stopped);
        Ok(())
    }

    fn emit_event(&self, _event: TunexEvent) -> TunexResult<()> {
        let mut stats = self.stats.lock().expect("runtime stats mutex");
        stats.emitted_events = stats.emitted_events.saturating_add(1);
        Ok(())
    }

    fn submit_task(&self, _priority: TaskPriority) -> TunexResult<()> {
        let mut stats = self.stats.lock().expect("runtime stats mutex");
        stats.submitted_tasks = stats.submitted_tasks.saturating_add(1);
        Ok(())
    }
}

#[test]
fn runtime_handle_tracks_state_tasks_and_events() {
    let runtime = MockRuntimeHandle::default();

    runtime.resume().expect("runtime should resume");
    runtime
        .submit_task(TaskPriority::High)
        .expect("task marker should submit");
    runtime
        .emit_event(TunexEvent::new(
            EventId::new("event-1"),
            EventKind::JobStarted,
            EventSeverity::Info,
        ))
        .expect("runtime should emit events");
    runtime.pause().expect("runtime should pause");
    runtime.drain().expect("runtime should drain");
    runtime
        .stop(ShutdownMode::Graceful)
        .expect("runtime should stop");

    let stats = runtime.stats();
    assert_eq!(runtime.state(), RuntimeState::Stopped);
    assert_eq!(stats.submitted_tasks, 1);
    assert_eq!(stats.emitted_events, 1);
}

#[test]
fn runtime_state_contract_allows_expected_transitions() {
    assert!(RuntimeState::Created.can_transition_to(RuntimeState::Initializing));
    assert!(RuntimeState::Initializing.can_transition_to(RuntimeState::Running));
    assert!(RuntimeState::Running.can_transition_to(RuntimeState::Paused));
    assert!(RuntimeState::Running.can_transition_to(RuntimeState::Draining));
    assert!(!RuntimeState::Stopped.can_transition_to(RuntimeState::Running));
}
