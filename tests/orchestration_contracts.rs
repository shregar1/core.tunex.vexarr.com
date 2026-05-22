use bytes::Bytes;
use tunex_core::{
    events::{EventPublisher, EventSubscriber, PipelineEventBus},
    pipeline::{BatchChannel, PipelineNodeId, PipelineNodeMetadata, PipelineStage},
    plugin::{PluginCapability, PluginLoader, PluginMetadata, PluginVersion},
    runtime::{CancellationToken, RuntimeLayer, ShutdownReason, WorkerSignal},
    scheduler::{BatchPriority, SchedulingPolicy, TaskPriority, WorkerAffinity},
    topology::{PipelineEdge, PipelineGraph},
    types::{BatchView, BorrowedBatch, PipelineEvent, ZeroCopyBuffer},
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

#[test]
fn pipeline_graph_rejects_duplicate_nodes_and_cycles() {
    let dataset =
        PipelineNodeMetadata::new(PipelineNodeId::new(1), "dataset", PipelineStage::Dataset);
    let tokenizer = PipelineNodeMetadata::new(
        PipelineNodeId::new(2),
        "tokenizer",
        PipelineStage::Tokenizer,
    );
    let mut graph = PipelineGraph::new();

    graph.add_node(dataset.clone()).unwrap();
    graph.add_node(tokenizer).unwrap();
    graph
        .add_edge(PipelineEdge::new(dataset.id, PipelineNodeId::new(2)))
        .unwrap();

    assert!(graph.add_node(dataset).is_err());
    assert!(graph
        .add_edge(PipelineEdge::new(
            PipelineNodeId::new(2),
            PipelineNodeId::new(1)
        ))
        .is_err());
}

#[test]
fn cancellation_token_is_shared_between_clones() {
    let token = CancellationToken::new();
    let cloned = token.clone();

    assert!(!token.is_cancelled());
    cloned.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn memory_views_do_not_copy_payload() {
    let buffer = ZeroCopyBuffer::new(Bytes::from_static(b"batch"));
    let cloned = buffer.clone();
    let items = [1_u32, 2, 3];
    let borrowed = BorrowedBatch::new(&items);
    let view = BatchView::Borrowed(borrowed.as_slice());

    assert_eq!(buffer.bytes().as_ptr(), cloned.bytes().as_ptr());
    assert_eq!(view.len(), 3);
    assert_eq!(view.as_slice(), &[1, 2, 3]);
}

#[test]
fn orchestration_contract_types_are_send_sync() {
    assert_send_sync::<PipelineNodeId>();
    assert_send_sync::<PipelineNodeMetadata>();
    assert_send_sync::<PipelineEdge>();
    assert_send_sync::<PipelineGraph>();
    assert_send_sync::<CancellationToken>();
    assert_send_sync::<WorkerSignal>();
    assert_send_sync::<ShutdownReason>();
    assert_send_sync::<RuntimeLayer>();
    assert_send_sync::<TaskPriority>();
    assert_send_sync::<BatchPriority>();
    assert_send_sync::<WorkerAffinity>();
    assert_send_sync::<SchedulingPolicy>();
    assert_send_sync::<PluginMetadata>();
    assert_send_sync::<PluginCapability>();
    assert_send_sync::<PluginVersion>();
    assert_send_sync::<PipelineEvent>();
}

#[test]
fn async_contract_traits_are_object_safe() {
    assert_object_safe::<dyn EventPublisher>();
    assert_object_safe::<dyn EventSubscriber>();
    assert_object_safe::<dyn PipelineEventBus>();
    assert_object_safe::<dyn PluginLoader>();
    assert_object_safe::<dyn BatchChannel<PipelineEvent>>();
}

#[test]
fn plugin_metadata_captures_capabilities() {
    let mut metadata = PluginMetadata::new(
        tunex_core::plugin::PluginId::new("local.cleaner"),
        tunex_core::plugin::PluginName::new("Local Cleaner"),
        PluginVersion::new(0, 1, 0),
        tunex_core::plugin::PluginKind::Dataset,
        Vec::new(),
    );
    metadata
        .capabilities
        .push(PluginCapability::new("preprocess"));

    assert_eq!(metadata.id.as_str(), "local.cleaner");
    assert_eq!(metadata.version.to_string(), "0.1.0");
    assert_eq!(metadata.capabilities[0].as_str(), "preprocess");
}
