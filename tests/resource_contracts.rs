use tunex_core::resources::{
    CpuResource, DiskResource, GpuResource, MemoryResource, NetworkResource,
    OversubscriptionPolicy, ResourceId, ResourceKind, ResourceLimit, ResourceReservation,
    ResourceSnapshot, ResourceUsage,
};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn resource_contract_types_are_send_sync() {
    assert_send_sync::<ResourceId>();
    assert_send_sync::<ResourceKind>();
    assert_send_sync::<CpuResource>();
    assert_send_sync::<GpuResource>();
    assert_send_sync::<MemoryResource>();
    assert_send_sync::<DiskResource>();
    assert_send_sync::<NetworkResource>();
    assert_send_sync::<ResourceLimit>();
    assert_send_sync::<ResourceUsage>();
    assert_send_sync::<ResourceSnapshot>();
    assert_send_sync::<ResourceReservation>();
    assert_send_sync::<OversubscriptionPolicy>();
}

#[test]
fn cpu_gpu_and_memory_resources_capture_affinity_and_capacity() {
    let cpu = CpuResource::new(ResourceId::new("cpu:0"), 16)
        .with_physical_cores(8)
        .with_affinity_cores([0, 1, 2, 3])
        .with_numa_node(0);
    let gpu = GpuResource::new(ResourceId::new("gpu:0"), 0, 24 * 1024 * 1024 * 1024)
        .with_name("NVIDIA A10")
        .with_available_memory(20 * 1024 * 1024 * 1024)
        .with_numa_node(0);
    let memory = MemoryResource::new(ResourceId::new("ram:0"), 128 * 1024 * 1024 * 1024)
        .with_available_bytes(96 * 1024 * 1024 * 1024)
        .with_numa_node(0);

    assert_eq!(cpu.logical_cores, 16);
    assert_eq!(cpu.physical_cores, Some(8));
    assert_eq!(cpu.affinity_cores.as_slice(), &[0, 1, 2, 3]);
    assert_eq!(gpu.device_index, 0);
    assert_eq!(gpu.memory_available_bytes, 20 * 1024 * 1024 * 1024);
    assert_eq!(memory.available_bytes, 96 * 1024 * 1024 * 1024);
}

#[test]
fn disk_and_network_resources_capture_capacity_and_bandwidth() {
    let disk = DiskResource::new(ResourceId::new("disk:train"), "/mnt/train", 2_000)
        .with_available_bytes(1_500)
        .with_bandwidth(900, 700);
    let network = NetworkResource::new(ResourceId::new("net:eth0"), "eth0")
        .with_bandwidth(10_000, 10_000)
        .distributed_reachable();

    assert_eq!(disk.mount_path, "/mnt/train");
    assert_eq!(disk.available_bytes, 1_500);
    assert_eq!(disk.read_bytes_per_sec, Some(900));
    assert_eq!(network.interface, "eth0");
    assert_eq!(network.tx_bytes_per_sec, Some(10_000));
    assert!(network.distributed_reachable);
}

#[test]
fn limits_and_reservations_model_oversubscription_policy() {
    let limit = ResourceLimit::new(ResourceKind::Gpu)
        .for_resource(ResourceId::new("gpu:0"))
        .with_max_bytes(20 * 1024 * 1024 * 1024)
        .with_oversubscription(OversubscriptionPolicy::Deny);
    let reservation =
        ResourceReservation::new("reservation-1", ResourceId::new("gpu:0"), ResourceKind::Gpu)
            .with_bytes(8 * 1024 * 1024 * 1024);

    assert_eq!(limit.resource_id.as_ref().unwrap().as_str(), "gpu:0");
    assert_eq!(limit.max_bytes, Some(20 * 1024 * 1024 * 1024));
    assert_eq!(limit.oversubscription, OversubscriptionPolicy::Deny);
    assert_eq!(reservation.reservation_id, "reservation-1");
    assert_eq!(reservation.bytes, Some(8 * 1024 * 1024 * 1024));
}

#[test]
fn usage_snapshot_aggregates_available_memory_and_utilization() {
    let snapshot = ResourceSnapshot::new()
        .with_memory(MemoryResource::new(ResourceId::new("ram:0"), 1_000).with_available_bytes(600))
        .with_memory(
            MemoryResource::new(ResourceId::new("ram:1"), 2_000).with_available_bytes(1_400),
        )
        .with_gpu(GpuResource::new(ResourceId::new("gpu:0"), 0, 1_000).with_available_memory(250))
        .with_usage(
            ResourceUsage::new(ResourceId::new("cpu:0"), ResourceKind::Cpu)
                .with_used_units(6)
                .with_utilization_basis_points(12_000),
        )
        .with_reservation(
            ResourceReservation::new(
                "reservation-1",
                ResourceId::new("ram:0"),
                ResourceKind::Memory,
            )
            .with_bytes(200),
        );

    assert_eq!(snapshot.available_ram_bytes(), 2_000);
    assert_eq!(snapshot.available_gpu_memory_bytes(), 250);
    assert_eq!(snapshot.usage[0].used_units, Some(6));
    assert_eq!(snapshot.usage[0].utilization_basis_points, Some(10_000));
    assert_eq!(snapshot.reservations.len(), 1);
}
