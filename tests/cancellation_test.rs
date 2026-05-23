use tunex_core::{
    cancellation::{CancellationReason, CancellationScope, CancellationToken, DrainPolicy},
    channel::{ChannelStats, QueuePressure},
    runtime::ShutdownMode,
};

#[test]
fn cancellation_token_cascades_to_pipeline_children() {
    let root = CancellationToken::new();
    let dataset = root.child(CancellationReason::StageCancelled {
        stage: "dataset".to_owned(),
    });
    let trainer = root.child(CancellationReason::StageCancelled {
        stage: "trainer".to_owned(),
    });

    root.cancel();

    assert!(dataset.is_cancelled());
    assert!(trainer.is_cancelled());
    assert!(matches!(
        trainer.reason(),
        Some(CancellationReason::StageCancelled { stage }) if stage == "trainer"
    ));
}

#[test]
fn cancellation_scope_preserves_checkpoint_before_stop_policy() {
    let scope = CancellationScope::new(
        "job-1",
        DrainPolicy::new(ShutdownMode::CheckpointThenStop).with_timeout_millis(5000),
    );
    let cancelled = scope.cancel(CancellationReason::JobCancelled {
        job_id: "job-1".to_owned(),
    });

    assert!(cancelled.is_cancelled());
    assert!(scope.drain_policy().checkpoint_before_exit);
    assert!(scope.drain_policy().drain_in_flight);
    assert_eq!(scope.drain_policy().timeout_millis, Some(5000));
}

#[test]
fn queue_pressure_contract_flags_backpressure_before_cancellation() {
    let stats = ChannelStats::from_depth(100, 100);

    assert_eq!(stats.backpressure.pressure, QueuePressure::Full);
    assert!(stats.backpressure.producer_throttled);
    assert_eq!(stats.backpressure.consumer_lag, 100);
}
