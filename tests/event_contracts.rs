use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tunex_core::{
    events::{
        EventBus, EventBusStats, EventFilter, EventId, EventKind, EventPublisher, EventSeverity,
        EventSource, EventSubscriber, EventTimestamp, TunexEvent,
    },
    TunexResult,
};
use uuid::Uuid;

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

#[test]
fn event_creation_is_structured_and_low_allocation_for_metadata() {
    let event = TunexEvent::new(
        EventId::from_uuid(Uuid::nil()),
        EventKind::BatchProcessed,
        EventSeverity::Info,
    )
    .with_timestamp(EventTimestamp::from_unix_millis(42))
    .with_source(
        EventSource::new()
            .with_module("packer")
            .with_stage("packing")
            .with_trace("trace-1")
            .with_job("job-1")
            .with_worker("worker-1")
            .with_batch("batch-1"),
    )
    .with_message("batch processed")
    .with_metadata("tokens", "4096");

    assert_eq!(event.id.as_str(), "00000000-0000-0000-0000-000000000000");
    assert_eq!(event.timestamp.as_unix_millis(), 42);
    assert_eq!(event.source.module.as_deref(), Some("packer"));
    assert_eq!(event.source.batch_id.as_deref(), Some("batch-1"));
    assert_eq!(event.metadata[0], ("tokens".to_owned(), "4096".to_owned()));
}

#[test]
fn event_filter_matches_kind_severity_and_source_fields() {
    let event = TunexEvent::new(
        EventId::new("event-1"),
        EventKind::QueueStalled,
        EventSeverity::Warning,
    )
    .with_source(
        EventSource::new()
            .with_module("runtime")
            .with_stage("tokenizer")
            .with_job("job-1")
            .with_worker("worker-2"),
    );
    let filter = EventFilter {
        kinds: vec![EventKind::QueueStalled],
        severities: vec![EventSeverity::Warning],
        module: Some("runtime".to_owned()),
        stage: Some("tokenizer".to_owned()),
        job_id: Some("job-1".to_owned()),
        worker_id: Some("worker-2".to_owned()),
    };

    assert!(filter.matches(&event));
    assert!(!EventFilter::kind(EventKind::JobCompleted).matches(&event));
}

#[test]
fn event_contracts_are_send_sync_and_object_safe() {
    assert_send_sync::<TunexEvent>();
    assert_send_sync::<EventFilter>();
    assert_send_sync::<EventBusStats>();
    assert_object_safe::<dyn EventPublisher>();
    assert_object_safe::<dyn EventSubscriber>();
    assert_object_safe::<dyn EventBus>();
}

struct RecordingSubscriber {
    seen: Arc<Mutex<Vec<EventKind>>>,
}

#[async_trait]
impl EventSubscriber for RecordingSubscriber {
    fn filter(&self) -> EventFilter {
        EventFilter::kind(EventKind::PluginRegistered)
    }

    async fn on_event(&self, event: &TunexEvent) -> TunexResult<()> {
        self.seen.lock().unwrap().push(event.kind);
        Ok(())
    }
}

#[test]
fn subscriber_filter_is_part_of_contract() {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let subscriber = RecordingSubscriber { seen: seen.clone() };
    let matching = TunexEvent::new(
        EventId::new("plugin"),
        EventKind::PluginRegistered,
        EventSeverity::Info,
    );
    let skipped = TunexEvent::new(
        EventId::new("job"),
        EventKind::JobStarted,
        EventSeverity::Info,
    );

    assert!(subscriber.filter().matches(&matching));
    assert!(!subscriber.filter().matches(&skipped));
    assert!(seen.lock().unwrap().is_empty());
}
