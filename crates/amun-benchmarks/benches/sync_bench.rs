use amun_resource_core::ResourceRegistry;
use amun_state_sync::sync_protocol::SyncProtocol;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_test_registry(size: u64) -> ResourceRegistry {
    let mut reg = ResourceRegistry::new(size as usize * 2);
    use amun_resource_core::resource_lineage::ResourceLineage;
    use amun_resource_core::transformation_matrix::ResourceArchetype;
    use amun_resource_core::{ResourceId, ResourceMetadata, ResourceState};

    for i in 0..size {
        let id = {
            let mut a = [0u8; 32];
            a[0..8].copy_from_slice(&i.to_le_bytes());
            ResourceId(a)
        };
        reg.register_genesis(ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [0u8; 32],
            owner: [1u8; 32],
        })
        .unwrap();
    }
    reg
}

fn bench_snapshot_create_1k(c: &mut Criterion) {
    let reg = create_test_registry(1000);
    c.bench_function("snapshot_create_1k_resources", |b| {
        b.iter(|| {
            let snap = SyncProtocol::create_snapshot(&reg, 1, [0xAA; 32], [0xBB; 32]);
            black_box(snap.total_resources);
        })
    });
}

fn bench_snapshot_import_1k(c: &mut Criterion) {
    let reg = create_test_registry(1000);
    let snap = SyncProtocol::create_snapshot(&reg, 1, [0xAA; 32], [0xBB; 32]);

    c.bench_function("snapshot_import_1k_resources", |b| {
        b.iter(|| {
            let imported = SyncProtocol::import_snapshot(&snap, [0xBB; 32]).unwrap();
            black_box(imported.compute_state_root());
        })
    });
}

fn bench_snapshot_create_10k(c: &mut Criterion) {
    let reg = create_test_registry(10000);
    c.bench_function("snapshot_create_10k_resources", |b| {
        b.iter(|| {
            let snap = SyncProtocol::create_snapshot(&reg, 1, [0xAA; 32], [0xBB; 32]);
            black_box(snap.total_resources);
        })
    });
}

criterion_group!(
    benches,
    bench_snapshot_create_1k,
    bench_snapshot_import_1k,
    bench_snapshot_create_10k
);
criterion_main!(benches);
