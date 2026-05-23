use std::sync::Mutex;

use tunex_core::{prelude::*, scheduler::TaskPriority};

struct MockRuntime {
    state: Mutex<RuntimeState>,
}

impl RuntimeHandle for MockRuntime {
    fn state(&self) -> RuntimeState {
        *self.state.lock().expect("runtime mutex")
    }

    fn stats(&self) -> RuntimeStats {
        RuntimeStats {
            active_workers: 1,
            ..RuntimeStats::default()
        }
    }

    fn pause(&self) -> TunexResult<()> {
        *self.state.lock().expect("runtime mutex") = RuntimeState::Paused;
        Ok(())
    }

    fn resume(&self) -> TunexResult<()> {
        *self.state.lock().expect("runtime mutex") = RuntimeState::Running;
        Ok(())
    }

    fn drain(&self) -> TunexResult<()> {
        *self.state.lock().expect("runtime mutex") = RuntimeState::Draining;
        Ok(())
    }

    fn stop(&self, _mode: ShutdownMode) -> TunexResult<()> {
        *self.state.lock().expect("runtime mutex") = RuntimeState::Stopped;
        Ok(())
    }

    fn emit_event(&self, _event: TunexEvent) -> TunexResult<()> {
        Ok(())
    }

    fn submit_task(&self, _priority: TaskPriority) -> TunexResult<()> {
        Ok(())
    }
}

fn main() -> TunexResult<()> {
    let runtime = MockRuntime {
        state: Mutex::new(RuntimeState::Running),
    };

    runtime.submit_task(TaskPriority::High)?;
    runtime.emit_event(TunexEvent::new(
        EventId::new("runtime-event"),
        EventKind::StageStarted,
        EventSeverity::Info,
    ))?;
    runtime.pause()?;
    runtime.resume()?;
    runtime.drain()?;
    runtime.stop(ShutdownMode::Graceful)?;

    println!(
        "runtime stopped with {} active worker(s)",
        runtime.stats().active_workers
    );
    Ok(())
}
