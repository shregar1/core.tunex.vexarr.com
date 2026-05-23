use std::{
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use async_trait::async_trait;
use tunex_core::{
    events::{
        EventBus, EventBusStats, EventFilter, EventId, EventKind, EventPublisher, EventSeverity,
        EventSource, EventSubscriber, TunexEvent,
    },
    TunexResult,
};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[derive(Default)]
struct RecordingSubscriber {
    events: Mutex<Vec<TunexEvent>>,
}

#[async_trait]
impl EventSubscriber for RecordingSubscriber {
    async fn on_event(&self, event: &TunexEvent) -> TunexResult<()> {
        self.events
            .lock()
            .expect("subscriber events mutex")
            .push(event.clone());
        Ok(())
    }
}

#[derive(Default)]
struct RecordingEventBus {
    capacity: usize,
    events: Mutex<Vec<TunexEvent>>,
    subscribers: Mutex<Vec<(EventFilter, Box<dyn EventSubscriber>)>>,
}

impl RecordingEventBus {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            ..Self::default()
        }
    }
}

#[async_trait]
impl EventPublisher for RecordingEventBus {
    async fn publish(&self, event: TunexEvent) -> TunexResult<()> {
        self.try_publish(event)
    }

    fn try_publish(&self, event: TunexEvent) -> TunexResult<()> {
        let mut events = self.events.lock().expect("event bus events mutex");
        if events.len() < self.capacity {
            events.push(event);
        }
        Ok(())
    }
}

#[async_trait]
impl EventBus for RecordingEventBus {
    async fn subscribe(
        &self,
        filter: EventFilter,
        subscriber: Box<dyn EventSubscriber>,
    ) -> TunexResult<()> {
        self.subscribers
            .lock()
            .expect("event bus subscribers mutex")
            .push((filter, subscriber));
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn queued(&self) -> usize {
        self.events.lock().expect("event bus events mutex").len()
    }

    fn stats(&self) -> EventBusStats {
        EventBusStats {
            capacity: self.capacity,
            queued: self.queued(),
            subscribers: self
                .subscribers
                .lock()
                .expect("event bus subscribers mutex")
                .len(),
        }
    }
}

#[test]
fn event_bus_records_structured_events_and_filters() {
    let bus = RecordingEventBus::with_capacity(4);
    block_on(bus.subscribe(
        EventFilter::kind(EventKind::BatchProcessed),
        Box::new(RecordingSubscriber::default()),
    ))
    .expect("subscriber should register");

    let event = TunexEvent::new(
        EventId::new("event-1"),
        EventKind::BatchProcessed,
        EventSeverity::Info,
    )
    .with_source(
        EventSource::new()
            .with_module("core")
            .with_job("job-1")
            .with_worker("worker-1")
            .with_batch("batch-1"),
    )
    .with_metadata("tokens", "5");

    assert!(EventFilter::kind(EventKind::BatchProcessed).matches(&event));
    assert!(!EventFilter::kind(EventKind::QueueStalled).matches(&event));

    block_on(bus.publish(event)).expect("event should publish");

    let stats = bus.stats();
    assert_eq!(stats.capacity, 4);
    assert_eq!(stats.queued, 1);
    assert_eq!(stats.subscribers, 1);
}
