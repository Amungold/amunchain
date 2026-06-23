use amun_core_optimization::OptimizedRegistry;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use sha2::Digest;
use std::time::Instant;

#[test]
fn n161_compare_cached_vs_uncached() {
    let mut reg = ResourceRegistry::new(20000);
    let mut opt_reg = OptimizedRegistry::new(20000);
    let col_id = ResourceId([1u8; 32]);

    for reg_ref in [&mut reg, &mut opt_reg.registry].iter_mut() {
        reg_ref
            .register_genesis(ResourceMetadata {
                resource_id: col_id,
                archetype: ResourceArchetype::NFTCollection,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(col_id),
                contract_id: [0u8; 32],
                owner: [0u8; 32],
            })
            .unwrap();
    }

    for i in 0..5_000u64 {
        let token_id = ResourceId(sha2::Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        let meta = ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
            contract_id: [0u8; 32],
            owner: [1u8; 32],
        };
        reg.derive_from_collection(&col_id, meta.clone()).unwrap();
        opt_reg
            .registry
            .derive_from_collection(&col_id, meta)
            .unwrap();
    }

    let start = Instant::now();
    let _root1 = reg.compute_state_root();
    let uncached_ms = start.elapsed().as_millis();

    let start = Instant::now();
    let _root2 = opt_reg.compute_state_root();
    let cached_ms = start.elapsed().as_millis();

    println!("Uncached: {}ms, Cached: {}ms", uncached_ms, cached_ms);
    assert!(cached_ms <= uncached_ms + 2);
}
