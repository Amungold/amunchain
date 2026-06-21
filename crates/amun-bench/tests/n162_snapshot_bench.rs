use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_bench::time_op;
use sha2::Digest;

#[test]
fn n162_bench_snapshot_build_and_restore() {
    let mut reg = ResourceRegistry::new(20000);
    let col_id = ResourceId([1u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    }).unwrap();

    for i in 0..5_000u64 {
        let token_id = ResourceId(sha2::Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(&col_id, ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
            contract_id: [0u8; 32],
            owner: [1u8; 32],
        }).unwrap();
    }

    let snapshot_result = time_op("snapshot_clone_active", || {
        let _cloned: Vec<ResourceMetadata> = reg.active_resources().iter().map(|&m| m.clone()).collect();
    });

    println!("Snapshot clone active resources: {}ms", snapshot_result.duration_ms);
    assert!(snapshot_result.duration_ms < 200, "Snapshot clone too slow");
}
