use std::{
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use async_trait::async_trait;
use tunex_core::prelude::*;

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

#[derive(Default)]
struct RecordingPublisher {
    events: Mutex<Vec<TunexEvent>>,
}

#[async_trait]
impl EventPublisher for RecordingPublisher {
    async fn publish(&self, event: TunexEvent) -> TunexResult<()> {
        self.try_publish(event)
    }

    fn try_publish(&self, event: TunexEvent) -> TunexResult<()> {
        self.events.lock().expect("event mutex").push(event);
        Ok(())
    }
}

fn main() -> TunexResult<()> {
    let publisher = RecordingPublisher::default();
    let event = TunexEvent::new(
        EventId::new("event-1"),
        EventKind::JobStarted,
        EventSeverity::Info,
    )
    .with_source(EventSource::new().with_module("example").with_job("job-1"))
    .with_message("example job started")
    .with_metadata("example", "event_bus");

    block_on(publisher.publish(event))?;
    let count = publisher.events.lock().expect("event mutex").len();
    println!("published {count} event(s)");
    Ok(())
}
