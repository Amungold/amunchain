use amun_bench::time_op;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use sha2::Digest;

#[test]
fn n161_bench_state_root_10k_nft() {
    let mut reg = ResourceRegistry::new(20000);
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

    for i in 0..10_000u64 {
        let token_id = ResourceId(sha2::Sha256::digest(i.to_le_bytes()).into());
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

    let result = time_op("state_root_10k_nft", || {
        let _root = reg.compute_state_root();
    });

    println!("Bench {}: {}ms", result.name, result.duration_ms);
    assert!(
        result.duration_ms < 500,
        "State root too slow: {}ms",
        result.duration_ms
    );
}
