use std::{thread, time::Duration};

use tunex_core::{
    cancellation::{
        CancellationReason, CancellationScope, CancellationToken, DrainPolicy, ShutdownGuard,
        ShutdownMode,
    },
    runtime,
};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn token_cancellation_is_shared_between_clones_and_children() {
    let token = CancellationToken::new();
    let clone = token.clone();
    let child = token.child(CancellationReason::StageCancelled {
        stage: "tokenizer".to_owned(),
    });

    clone.cancel();

    assert!(token.is_cancelled());
    assert!(child.is_cancelled());
    assert!(matches!(
        child.reason(),
        Some(CancellationReason::StageCancelled { stage }) if stage == "tokenizer"
    ));
}

#[test]
fn cancellation_scope_cascades_to_child_scope() {
    let parent = CancellationScope::new("job", DrainPolicy::new(ShutdownMode::Graceful));
    let child = parent.child("worker", DrainPolicy::new(ShutdownMode::DrainCurrentBatch));

    let cancelled = parent.cancel(CancellationReason::JobCancelled {
        job_id: "job-1".to_owned(),
    });

    assert!(cancelled.is_cancelled());
    assert!(child.token().is_cancelled());
    assert!(matches!(
        child.token().reason(),
        Some(CancellationReason::Cascaded { parent_scope }) if parent_scope == "job"
    ));
}

#[test]
fn drain_policy_models_checkpoint_and_inflight_behavior() {
    let checkpoint = DrainPolicy::new(ShutdownMode::CheckpointThenStop).with_timeout_millis(1000);
    let immediate = DrainPolicy::new(ShutdownMode::Immediate);

    assert!(checkpoint.checkpoint_before_exit);
    assert!(checkpoint.drain_in_flight);
    assert_eq!(checkpoint.timeout_millis, Some(1000));
    assert!(!immediate.checkpoint_before_exit);
    assert!(!immediate.drain_in_flight);
}

#[test]
fn shutdown_guard_tracks_timeout_and_cancels_scope() {
    let scope = CancellationScope::new(
        "checkpoint",
        DrainPolicy::new(ShutdownMode::CheckpointThenStop).with_timeout_millis(1),
    );
    let guard = ShutdownGuard::new(scope);
    thread::sleep(Duration::from_millis(2));

    assert!(guard.timed_out());
    assert!(guard.cancel(CancellationReason::Timeout).is_cancelled());
}

#[test]
fn runtime_reexports_cancellation_contracts() {
    let token = runtime::CancellationToken::new();
    token.cancel();

    assert!(token.is_cancelled());
}

#[test]
fn cancellation_contracts_are_send_sync() {
    assert_send_sync::<CancellationToken>();
    assert_send_sync::<CancellationReason>();
    assert_send_sync::<CancellationScope>();
    assert_send_sync::<DrainPolicy>();
    assert_send_sync::<ShutdownGuard>();
    assert_send_sync::<ShutdownMode>();
}
