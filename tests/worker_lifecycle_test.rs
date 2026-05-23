use tunex_core::runtime::{RuntimeState, WorkerSignal};

#[derive(Debug)]
struct MockWorkerLifecycle {
    state: RuntimeState,
    signals: Vec<WorkerSignal>,
}

impl Default for MockWorkerLifecycle {
    fn default() -> Self {
        Self {
            state: RuntimeState::Created,
            signals: Vec::new(),
        }
    }
}

impl MockWorkerLifecycle {
    fn apply(&mut self, signal: WorkerSignal) {
        self.signals.push(signal.clone());
        self.state = match signal {
            WorkerSignal::Start => RuntimeState::Running,
            WorkerSignal::Pause => RuntimeState::Paused,
            WorkerSignal::Drain => RuntimeState::Draining,
            WorkerSignal::Stop => RuntimeState::Stopped,
            WorkerSignal::Restart => RuntimeState::Initializing,
        };
    }
}

#[test]
fn worker_lifecycle_signals_model_start_pause_drain_stop() {
    let mut worker = MockWorkerLifecycle::default();

    worker.apply(WorkerSignal::Start);
    worker.apply(WorkerSignal::Pause);
    worker.apply(WorkerSignal::Start);
    worker.apply(WorkerSignal::Drain);
    worker.apply(WorkerSignal::Stop);

    assert_eq!(worker.state, RuntimeState::Stopped);
    assert_eq!(worker.signals.len(), 5);
    assert_eq!(worker.signals[0], WorkerSignal::Start);
}

#[test]
fn worker_restart_signal_reenters_initializing_state() {
    let mut worker = MockWorkerLifecycle::default();

    worker.apply(WorkerSignal::Start);
    worker.apply(WorkerSignal::Restart);

    assert_eq!(worker.state, RuntimeState::Initializing);
}
