use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use bytes::Bytes;
use tunex_core::{
    checkpoint::{CheckpointFormat, CheckpointId, CheckpointMetadata, CheckpointSnapshot},
    pipeline::{PipelineEdge, PipelineGraph, PipelineNode, PipelineNodeId, PipelineNodeKind},
    testing::{
        MockCheckpointWriter, MockDatasetSource, MockSequencePacker, MockTokenizer,
        MockTrainerBackend,
    },
    traits::{CheckpointWriter, DatasetSource, SequencePacker, Tokenizer, TrainerBackend},
    types::RawSample,
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

#[test]
fn mock_pipeline_contracts_move_sample_to_checkpoint() {
    let dataset =
        MockDatasetSource::new([RawSample::with_id("sample-1", Bytes::from_static(b"hello"))]);
    let tokenizer = MockTokenizer;
    let packer = MockSequencePacker;
    let trainer = MockTrainerBackend::new();
    let checkpoint = MockCheckpointWriter::default();

    let sample = block_on(dataset.next_sample())
        .expect("dataset source should produce result")
        .expect("mock dataset should contain one sample");
    let tokenized = block_on(tokenizer.tokenize(&sample)).expect("tokenizer should produce tokens");
    let batch = block_on(packer.pack(&[tokenized])).expect("packer should build batch");
    let metrics = block_on(trainer.train_step(&batch)).expect("trainer should emit metrics");

    let metadata = CheckpointMetadata::new(CheckpointId::new("ckpt-1"), metrics.step, "mock")
        .with_format(CheckpointFormat::RuntimeState)
        .full_state();
    let snapshot = CheckpointSnapshot::new(metadata).with_chunk(Bytes::from_static(b"state"));
    let written = block_on(checkpoint.write_snapshot(snapshot))
        .expect("checkpoint writer should record snapshot");

    assert_eq!(dataset.remaining(), 0);
    assert_eq!(batch.sequence_count(), 1);
    assert_eq!(metrics.tokens, u64::try_from(batch.token_count()).unwrap());
    assert_eq!(written.id.as_str(), "ckpt-1");
    assert_eq!(checkpoint.snapshots().len(), 1);
}

#[test]
fn standard_training_dag_contract_validates() {
    let mut graph = PipelineGraph::new();
    let dataset = PipelineNodeId::new(1);
    let tokenizer = PipelineNodeId::new(2);
    let packer = PipelineNodeId::new(3);
    let trainer = PipelineNodeId::new(4);
    let checkpoint = PipelineNodeId::new(5);

    for node in [
        PipelineNode::new(dataset, "dataset", PipelineNodeKind::Dataset),
        PipelineNode::new(tokenizer, "tokenizer", PipelineNodeKind::Tokenizer),
        PipelineNode::new(packer, "packer", PipelineNodeKind::Packer),
        PipelineNode::new(trainer, "trainer", PipelineNodeKind::Trainer),
        PipelineNode::new(checkpoint, "checkpoint", PipelineNodeKind::Checkpoint),
    ] {
        graph.add_node(node).expect("node ids should be unique");
    }

    for edge in [
        PipelineEdge::new(dataset, tokenizer),
        PipelineEdge::new(tokenizer, packer),
        PipelineEdge::new(packer, trainer),
        PipelineEdge::new(trainer, checkpoint),
    ] {
        graph.add_edge(edge).expect("stage order should be valid");
    }

    assert!(graph.validate().is_valid());
    assert_eq!(graph.nodes().len(), 5);
    assert_eq!(graph.edges().len(), 4);
}
