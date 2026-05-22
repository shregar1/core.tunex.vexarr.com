use std::sync::Mutex;

use async_trait::async_trait;
use tunex_core::{
    cancellation::{CancellationToken, ShutdownMode},
    events::{EventId, EventKind, EventSeverity, TunexEvent},
    pipeline::{PipelineGraph, PipelineNode, PipelineNodeId, PipelineNodeKind},
    runtime::{
        RuntimeContext, RuntimeEngine, RuntimeExecutor, RuntimeHandle, RuntimeMode, RuntimeOptions,
        RuntimeState, RuntimeStats,
    },
    scheduler::{SchedulingDecision, SchedulingPolicy, TaskId, TaskPriority},
    TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

struct TestHandle {
    state: Mutex<RuntimeState>,
    stats: RuntimeStats,
}

impl RuntimeHandle for TestHandle {
    fn state(&self) -> RuntimeState {
        *self.state.lock().unwrap()
    }

    fn stats(&self) -> RuntimeStats {
        self.stats.clone()
    }

    fn pause(&self) -> TunexResult<()> {
        *self.state.lock().unwrap() = RuntimeState::Paused;
        Ok(())
    }

    fn resume(&self) -> TunexResult<()> {
        *self.state.lock().unwrap() = RuntimeState::Running;
        Ok(())
    }

    fn drain(&self) -> TunexResult<()> {
        *self.state.lock().unwrap() = RuntimeState::Draining;
        Ok(())
    }

    fn stop(&self, _mode: ShutdownMode) -> TunexResult<()> {
        *self.state.lock().unwrap() = RuntimeState::Stopped;
        Ok(())
    }

    fn emit_event(&self, _event: TunexEvent) -> TunexResult<()> {
        Ok(())
    }

    fn submit_task(&self, _priority: TaskPriority) -> TunexResult<()> {
        Ok(())
    }
}

struct TestEngine {
    state: Mutex<RuntimeState>,
}

impl RuntimeEngine for TestEngine {
    fn start(&self, _ctx: RuntimeContext) -> TunexResult<Box<dyn RuntimeHandle>> {
        *self.state.lock().unwrap() = RuntimeState::Running;
        Ok(Box::new(TestHandle {
            state: Mutex::new(RuntimeState::Running),
            stats: RuntimeStats {
                active_workers: 1,
                ..RuntimeStats::default()
            },
        }))
    }

    fn stop(&self, _mode: ShutdownMode) -> TunexResult<()> {
        *self.state.lock().unwrap() = RuntimeState::Stopped;
        Ok(())
    }

    fn state(&self) -> RuntimeState {
        *self.state.lock().unwrap()
    }

    fn stats(&self) -> RuntimeStats {
        RuntimeStats::default()
    }

    fn apply_scheduling_decision(
        &self,
        _task_id: TaskId,
        _decision: SchedulingDecision,
    ) -> TunexResult<()> {
        Ok(())
    }
}

struct TestExecutor;

#[async_trait]
impl RuntimeExecutor for TestExecutor {
    async fn execute(
        &self,
        _task_id: TaskId,
        _decision: SchedulingDecision,
        cancellation: CancellationToken,
    ) -> TunexResult<()> {
        assert!(!cancellation.is_cancelled());
        Ok(())
    }
}

#[test]
fn runtime_context_carries_pipeline_options_and_cancellation() {
    let mut graph = PipelineGraph::new();
    graph
        .add_node(PipelineNode::new(
            PipelineNodeId::new(1),
            "dataset",
            PipelineNodeKind::Dataset,
        ))
        .unwrap();
    let options = RuntimeOptions {
        mode: RuntimeMode::Debug,
        scheduling_policy: SchedulingPolicy::LowLatency,
        max_workers: 8,
        events_enabled: true,
        metrics_enabled: false,
    };
    let cancellation = CancellationToken::new();
    let ctx = RuntimeContext::new(graph)
        .with_options(options.clone())
        .with_cancellation(cancellation.clone());

    assert_eq!(ctx.options, options);
    assert!(!ctx.cancellation.is_cancelled());
    assert_eq!(ctx.pipeline.nodes().len(), 1);
}

#[test]
fn runtime_state_transitions_are_explicit() {
    assert!(RuntimeState::Created.can_transition_to(RuntimeState::Initializing));
    assert!(RuntimeState::Running.can_transition_to(RuntimeState::Paused));
    assert!(RuntimeState::Running.can_transition_to(RuntimeState::Draining));
    assert!(!RuntimeState::Stopped.can_transition_to(RuntimeState::Running));
}

#[test]
fn runtime_engine_and_handle_contracts_are_object_safe() {
    assert_send_sync::<RuntimeContext>();
    assert_send_sync::<RuntimeOptions>();
    assert_send_sync::<RuntimeStats>();
    assert_send_sync::<RuntimeState>();
    assert_send_sync::<RuntimeMode>();
    assert_object_safe::<dyn RuntimeEngine>();
    assert_object_safe::<dyn RuntimeHandle>();
    assert_object_safe::<dyn RuntimeExecutor>();
    let _executor = TestExecutor;
}

#[test]
fn runtime_engine_starts_and_handle_controls_state() {
    let engine = TestEngine {
        state: Mutex::new(RuntimeState::Created),
    };
    let handle = engine
        .start(RuntimeContext::new(PipelineGraph::new()))
        .unwrap();

    assert_eq!(engine.state(), RuntimeState::Running);
    assert_eq!(handle.state(), RuntimeState::Running);
    handle.pause().unwrap();
    assert_eq!(handle.state(), RuntimeState::Paused);
    handle.resume().unwrap();
    handle
        .emit_event(TunexEvent::new(
            EventId::new("event"),
            EventKind::JobStarted,
            EventSeverity::Info,
        ))
        .unwrap();
    handle.submit_task(TaskPriority::High).unwrap();
    handle.stop(ShutdownMode::Graceful).unwrap();
    assert_eq!(handle.state(), RuntimeState::Stopped);
}
