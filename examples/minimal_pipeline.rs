use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use async_trait::async_trait;
use bytes::Bytes;
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

struct FakeDatasetSource;

#[async_trait]
impl DatasetSource for FakeDatasetSource {
    async fn next_sample(&self) -> TunexResult<Option<RawSample>> {
        Ok(Some(RawSample::with_id(
            "sample-1",
            Bytes::from_static(b"hello tunex"),
        )))
    }
}

struct FakeTokenizer;

#[async_trait]
impl Tokenizer for FakeTokenizer {
    async fn tokenize(&self, sample: &RawSample) -> TunexResult<TokenizedSample> {
        let mut tokenized =
            TokenizedSample::new(sample.payload.iter().map(|byte| u32::from(*byte)).collect());
        tokenized.id.clone_from(&sample.id);
        Ok(tokenized)
    }
}

struct FakePacker;

#[async_trait]
impl SequencePacker for FakePacker {
    async fn pack(&self, samples: &[TokenizedSample]) -> TunexResult<PackedBatch> {
        let mut token_ids = Vec::new();
        let mut offsets = Vec::with_capacity(samples.len());
        for sample in samples {
            offsets.push(u32::try_from(token_ids.len()).unwrap_or(u32::MAX));
            token_ids.extend(sample.token_ids.iter().copied());
        }
        let labels = token_ids
            .iter()
            .map(|token| i32::try_from(*token).unwrap_or(i32::MAX))
            .collect();
        Ok(PackedBatch::new(token_ids, offsets, labels))
    }
}

fn build_graph() -> TunexResult<PipelineGraph> {
    let mut graph = PipelineGraph::new();
    graph.add_node(PipelineNode::new(
        PipelineNodeId::new(1),
        "dataset",
        PipelineNodeKind::Dataset,
    ))?;
    graph.add_node(PipelineNode::new(
        PipelineNodeId::new(2),
        "tokenizer",
        PipelineNodeKind::Tokenizer,
    ))?;
    graph.add_node(PipelineNode::new(
        PipelineNodeId::new(3),
        "packer",
        PipelineNodeKind::Packer,
    ))?;
    graph.add_edge(PipelineEdge::new(
        PipelineNodeId::new(1),
        PipelineNodeId::new(2),
    ))?;
    graph.add_edge(PipelineEdge::new(
        PipelineNodeId::new(2),
        PipelineNodeId::new(3),
    ))?;
    Ok(graph)
}

fn main() -> TunexResult<()> {
    let graph = build_graph()?;
    assert!(graph.validate().is_valid());

    let dataset = FakeDatasetSource;
    let tokenizer = FakeTokenizer;
    let packer = FakePacker;

    let sample = block_on(dataset.next_sample())?.expect("fixture has one sample");
    let tokenized = block_on(tokenizer.tokenize(&sample))?;
    let batch = block_on(packer.pack(&[tokenized]))?;

    println!(
        "built graph with {} nodes and packed {} tokens",
        graph.nodes().len(),
        batch.token_count()
    );
    Ok(())
}
