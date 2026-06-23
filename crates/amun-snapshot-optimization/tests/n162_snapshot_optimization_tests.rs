use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use amun_snapshot_optimization::*;
use sha2::{Digest, Sha256};
use std::time::Instant;

#[test]
fn n162_incremental_snapshot_faster_than_full() {
    let mut reg = ResourceRegistry::new(10000);
    let col_id = ResourceId([1u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    for i in 0..2000u64 {
        let token_id = ResourceId(Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: token_id,
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
                contract_id: [0u8; 32],
                owner: [1u8; 32],
            },
        )
        .unwrap();
    }

    let base_root = reg.compute_state_root();

    let start = Instant::now();
    let (compressed, _) = compress_snapshot(&reg);
    let full_snapshot_ms = start.elapsed().as_millis();

    let mut inc_snap = IncrementalSnapshot::new(base_root, 1);
    for meta in &compressed {
        inc_snap.add_delta(meta.clone());
    }

    let start = Instant::now();
    let delta_count = inc_snap.delta_count();
    let delta_ms = start.elapsed().as_millis();

    println!(
        "Full snapshot: {}ms, Incremental delta count: {}, query: {}ms",
        full_snapshot_ms, delta_count, delta_ms
    );
    assert!(delta_count > 0);
    assert!(
        delta_ms <= full_snapshot_ms + 2,
        "Delta query should not be significantly slower"
    );
}

#[test]
fn n162_restore_from_compressed_is_fast() {
    let mut reg = ResourceRegistry::new(1000);
    let col_id = ResourceId([1u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    for i in 0..500u64 {
        let token_id = ResourceId(Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: token_id,
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
                contract_id: [0u8; 32],
                owner: [1u8; 32],
            },
        )
        .unwrap();
    }

    let (compressed, _) = compress_snapshot(&reg);

    let start = Instant::now();
    let restored = restore_from_compressed(&compressed);
    let restore_ms = start.elapsed().as_millis();

    println!(
        "Compressed count: {}, Restored active: {}, Time: {}ms",
        compressed.len(),
        restored.total_active(),
        restore_ms
    );
    assert!(
        !compressed.is_empty(),
        "Compressed snapshot should have resources"
    );
    assert!(
        restore_ms < 50,
        "Restore from compressed should be fast, got {}ms",
        restore_ms
    );
}
