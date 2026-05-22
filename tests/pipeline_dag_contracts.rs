use tunex_core::pipeline::{
    PipelineEdge, PipelineGraph, PipelineNode, PipelineNodeId, PipelineNodeKind, PipelineStage,
    PipelineState, PipelineValidationError,
};

#[test]
fn validates_standard_training_dag_with_metrics_branch() {
    let mut graph = PipelineGraph::new();
    let dataset = PipelineNode::new(PipelineNodeId::new(1), "dataset", PipelineNodeKind::Dataset)
        .with_plugin("tunex-dataset-jsonl")
        .with_io(None::<String>, Some("raw_sample"));
    let tokenizer = PipelineNode::new(
        PipelineNodeId::new(2),
        "tokenizer",
        PipelineNodeKind::Tokenizer,
    )
    .with_plugin("tunex-tokenizer-hf")
    .with_io(Some("raw_sample"), Some("tokenized_sample"));
    let packer = PipelineNode::new(PipelineNodeId::new(3), "packer", PipelineNodeKind::Packer)
        .with_io(Some("tokenized_sample"), Some("packed_batch"));
    let trainer = PipelineNode::new(PipelineNodeId::new(4), "trainer", PipelineNodeKind::Trainer)
        .with_io(Some("packed_batch"), Some("step_metrics"));
    let checkpoint = PipelineNode::new(
        PipelineNodeId::new(5),
        "checkpoint",
        PipelineNodeKind::Checkpoint,
    )
    .with_io(Some("step_metrics"), None::<String>);
    let metrics = PipelineNode::new(PipelineNodeId::new(6), "metrics", PipelineNodeKind::Metrics)
        .with_io(Some("packed_batch"), None::<String>);

    for node in [dataset, tokenizer, packer, trainer, checkpoint, metrics] {
        graph.add_node(node).unwrap();
    }

    graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(1),
            PipelineNodeId::new(2),
        ))
        .unwrap();
    graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(2),
            PipelineNodeId::new(3),
        ))
        .unwrap();
    graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(3),
            PipelineNodeId::new(4),
        ))
        .unwrap();
    graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(4),
            PipelineNodeId::new(5),
        ))
        .unwrap();
    graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(3),
            PipelineNodeId::new(6),
        ))
        .unwrap();

    assert!(graph.validate().is_valid());
    assert_eq!(
        graph
            .node(PipelineNodeId::new(1))
            .unwrap()
            .plugin
            .as_deref(),
        Some("tunex-dataset-jsonl")
    );
}

#[test]
fn rejects_incompatible_edge_types() {
    let mut graph = PipelineGraph::new();
    graph
        .add_node(
            PipelineNode::new(PipelineNodeId::new(1), "dataset", PipelineNodeKind::Dataset)
                .with_io(None::<String>, Some("raw_sample")),
        )
        .unwrap();
    graph
        .add_node(
            PipelineNode::new(PipelineNodeId::new(2), "packer", PipelineNodeKind::Packer)
                .with_io(Some("tokenized_sample"), Some("packed_batch")),
        )
        .unwrap();

    let error = graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(1),
            PipelineNodeId::new(2),
        ))
        .unwrap_err();

    assert_eq!(error.category(), "internal");
}

#[test]
fn validation_report_detects_stage_order_and_cycles() {
    let mut graph = PipelineGraph::new();
    graph
        .add_node(PipelineNode::with_stage(
            PipelineNodeId::new(1),
            "trainer",
            PipelineNodeKind::Trainer,
            PipelineStage::Trainer,
        ))
        .unwrap();
    graph
        .add_node(PipelineNode::with_stage(
            PipelineNodeId::new(2),
            "dataset",
            PipelineNodeKind::Dataset,
            PipelineStage::Dataset,
        ))
        .unwrap();

    let error = graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(1),
            PipelineNodeId::new(2),
        ))
        .unwrap_err();

    assert_eq!(error.category(), "internal");
}

#[test]
fn pipeline_state_transitions_are_explicit() {
    let mut graph = PipelineGraph::new();

    graph.transition_to(PipelineState::Validated).unwrap();
    graph.transition_to(PipelineState::Running).unwrap();
    assert!(graph.transition_to(PipelineState::Created).is_err());
    graph.transition_to(PipelineState::Draining).unwrap();
    graph.transition_to(PipelineState::Completed).unwrap();
    assert_eq!(graph.state(), PipelineState::Completed);
}

#[test]
fn validation_report_can_hold_errors() {
    let mut report = tunex_core::pipeline::PipelineValidationReport::default();
    report.push(PipelineValidationError::CycleDetected);

    assert!(!report.is_valid());
}
