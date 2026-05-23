use std::{
    future::Future,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use async_trait::async_trait;
use bytes::Bytes;
use tunex_core::{
    checkpoint::{CheckpointFormat, CheckpointId, CheckpointSnapshot},
    prelude::*,
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
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

#[derive(Default)]
struct InMemoryCheckpointWriter {
    snapshots: Mutex<Vec<CheckpointSnapshot>>,
}

#[async_trait]
impl CheckpointWriter for InMemoryCheckpointWriter {
    async fn write_snapshot(
        &self,
        snapshot: CheckpointSnapshot,
    ) -> TunexResult<CheckpointMetadata> {
        let metadata = snapshot.metadata.clone();
        self.snapshots
            .lock()
            .expect("checkpoint mutex")
            .push(snapshot);
        Ok(metadata)
    }

    async fn write_metadata(&self, _metadata: CheckpointMetadata) -> TunexResult<()> {
        Ok(())
    }
}

fn main() -> TunexResult<()> {
    let writer = InMemoryCheckpointWriter::default();
    let metadata = CheckpointMetadata::new(CheckpointId::new("ckpt-1"), 10, "example")
        .with_format(CheckpointFormat::RuntimeState)
        .full_state();
    let snapshot =
        CheckpointSnapshot::new(metadata).with_chunk(Bytes::from_static(b"runtime-state"));

    let written = block_on(writer.write_snapshot(snapshot))?;
    println!("wrote checkpoint {} at step {}", written.id, written.step);
    Ok(())
}
