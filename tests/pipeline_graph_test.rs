use tunex_core::pipeline::{
    PipelineEdge, PipelineGraph, PipelineNode, PipelineNodeId, PipelineNodeKind, PipelineState,
};

#[test]
fn graph_supports_training_path_with_metrics_branch() {
    let mut graph = PipelineGraph::new();
    let dataset = PipelineNodeId::new(1);
    let tokenizer = PipelineNodeId::new(2);
    let packer = PipelineNodeId::new(3);
    let trainer = PipelineNodeId::new(4);
    let checkpoint = PipelineNodeId::new(5);
    let metrics = PipelineNodeId::new(6);

    for node in [
        PipelineNode::new(dataset, "dataset", PipelineNodeKind::Dataset)
            .with_plugin("tunex-dataset-mock"),
        PipelineNode::new(tokenizer, "tokenizer", PipelineNodeKind::Tokenizer)
            .with_plugin("tunex-tokenizer-mock"),
        PipelineNode::new(packer, "packer", PipelineNodeKind::Packer)
            .with_plugin("tunex-packer-mock"),
        PipelineNode::new(trainer, "trainer", PipelineNodeKind::Trainer)
            .with_plugin("tunex-trainer-mock"),
        PipelineNode::new(checkpoint, "checkpoint", PipelineNodeKind::Checkpoint)
            .with_plugin("tunex-checkpoint-mock"),
        PipelineNode::new(metrics, "metrics", PipelineNodeKind::Metrics)
            .with_plugin("tunex-metrics-mock"),
    ] {
        graph.add_node(node).expect("node should be accepted");
    }

    for edge in [
        PipelineEdge::new(dataset, tokenizer),
        PipelineEdge::new(tokenizer, packer),
        PipelineEdge::new(packer, trainer),
        PipelineEdge::new(trainer, checkpoint),
        PipelineEdge::new(packer, metrics),
    ] {
        graph.add_edge(edge).expect("edge should be acyclic");
    }

    assert!(graph.validate().is_valid());
    assert_eq!(graph.edges().len(), 5);
}

#[test]
fn graph_rejects_cycles_and_invalid_state_transitions() {
    let mut graph = PipelineGraph::new();
    let dataset = PipelineNodeId::new(1);
    let tokenizer = PipelineNodeId::new(2);

    graph
        .add_node(PipelineNode::new(
            dataset,
            "dataset",
            PipelineNodeKind::Dataset,
        ))
        .expect("dataset node should be accepted");
    graph
        .add_node(PipelineNode::new(
            tokenizer,
            "tokenizer",
            PipelineNodeKind::Tokenizer,
        ))
        .expect("tokenizer node should be accepted");
    graph
        .add_edge(PipelineEdge::new(dataset, tokenizer))
        .expect("forward edge should be accepted");

    assert!(graph
        .add_edge(PipelineEdge::new(tokenizer, dataset))
        .is_err());
    assert!(graph.transition_to(PipelineState::Running).is_err());
    graph
        .transition_to(PipelineState::Validated)
        .expect("created graph can become validated");
}
