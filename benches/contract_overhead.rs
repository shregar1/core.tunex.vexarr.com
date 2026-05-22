use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use tunex_core::{
    pipeline::{PipelineNodeId, PipelineNodeKind, PipelineNodeMetadata},
    topology::{PipelineEdge, PipelineGraph},
};

fn graph_contract_construction(c: &mut Criterion) {
    c.bench_function("pipeline_graph_contract_construction", |b| {
        b.iter(|| {
            let mut graph = PipelineGraph::new();
            let dataset = PipelineNodeMetadata::new(
                PipelineNodeId::new(1),
                "dataset",
                PipelineNodeKind::Dataset,
            );
            let tokenizer = PipelineNodeMetadata::new(
                PipelineNodeId::new(2),
                "tokenizer",
                PipelineNodeKind::Tokenizer,
            );

            graph.add_node(dataset).unwrap();
            graph.add_node(tokenizer).unwrap();
            graph
                .add_edge(PipelineEdge::new(
                    PipelineNodeId::new(1),
                    PipelineNodeId::new(2),
                ))
                .unwrap();
            black_box(graph);
        });
    });
}

criterion_group!(benches, graph_contract_construction);
criterion_main!(benches);
