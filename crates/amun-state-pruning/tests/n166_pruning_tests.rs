use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use amun_state_pruning::PrunableRegistry;
use sha2::{Digest, Sha256};

#[test]
fn n166_prune_by_height_reduces_active_set() {
    let mut pr = PrunableRegistry::new(10000);
    let col_id = ResourceId([1u8; 32]);
    pr.registry
        .register_genesis(ResourceMetadata {
            resource_id: col_id,
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(col_id),
            contract_id: [0u8; 32],
            owner: [0u8; 32],
        })
        .unwrap();

    for i in 1..500u64 {
        let token_id = ResourceId(Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = pr.registry.resource_hash(&col_id).unwrap();
        let version = pr.registry.get(&col_id).unwrap().lineage.version + 1;
        pr.registry
            .derive_from_collection(
                &col_id,
                ResourceMetadata {
                    resource_id: token_id,
                    archetype: ResourceArchetype::NFTAsset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(
                        token_id,
                        col_id,
                        parent_hash,
                        version,
                    ),
                    contract_id: [0u8; 32],
                    owner: [1u8; 32],
                },
            )
            .unwrap();
    }

    let active_before = pr.total_active();
    let pruned = pr.archive_by_height(250);
    let active_after = pr.total_active();

    println!(
        "Active before: {}, pruned: {}, active after: {}",
        active_before, pruned, active_after
    );
    assert!(pruned > 0, "Should have pruned some resources");
}

#[test]
fn n166_restore_archived_brings_back_resources() {
    let mut pr = PrunableRegistry::new(1000);
    let col_id = ResourceId([1u8; 32]);
    pr.registry
        .register_genesis(ResourceMetadata {
            resource_id: col_id,
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(col_id),
            contract_id: [0u8; 32],
            owner: [0u8; 32],
        })
        .unwrap();

    for i in 1..100u64 {
        let token_id = ResourceId(Sha256::digest(i.to_le_bytes()).into());
        let parent_hash = pr.registry.resource_hash(&col_id).unwrap();
        let version = pr.registry.get(&col_id).unwrap().lineage.version + 1;
        pr.registry
            .derive_from_collection(
                &col_id,
                ResourceMetadata {
                    resource_id: token_id,
                    archetype: ResourceArchetype::NFTAsset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(
                        token_id,
                        col_id,
                        parent_hash,
                        version,
                    ),
                    contract_id: [0u8; 32],
                    owner: [1u8; 32],
                },
            )
            .unwrap();
    }

    pr.archive_by_height(50);
    let active_after_prune = pr.total_active();
    let restored = pr.restore_archived();
    let active_after_restore = pr.total_active();

    println!(
        "After prune: {}, restored: {}, after restore: {}",
        active_after_prune, restored, active_after_restore
    );
    assert!(restored > 0, "Should have restored some resources");
}

#[test]
fn n166_pruned_root_deterministic() {
    let mut pr1 = PrunableRegistry::new(1000);
    let mut pr2 = PrunableRegistry::new(1000);

    for pr in [&mut pr1, &mut pr2].iter_mut() {
        let col_id = ResourceId([1u8; 32]);
        pr.registry
            .register_genesis(ResourceMetadata {
                resource_id: col_id,
                archetype: ResourceArchetype::NFTCollection,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(col_id),
                contract_id: [0u8; 32],
                owner: [0u8; 32],
            })
            .unwrap();

        for i in 1..10u64 {
            let token_id = ResourceId(Sha256::digest(i.to_le_bytes()).into());
            let parent_hash = pr.registry.resource_hash(&col_id).unwrap();
            let version = pr.registry.get(&col_id).unwrap().lineage.version + 1;
            pr.registry
                .derive_from_collection(
                    &col_id,
                    ResourceMetadata {
                        resource_id: token_id,
                        archetype: ResourceArchetype::NFTAsset,
                        state: ResourceState::Active,
                        lineage: ResourceLineage::single_ancestor(
                            token_id,
                            col_id,
                            parent_hash,
                            version,
                        ),
                        contract_id: [0u8; 32],
                        owner: [1u8; 32],
                    },
                )
                .unwrap();
        }
        pr.archive_by_height(5);
    }

    assert_eq!(pr1.compute_pruned_root(), pr2.compute_pruned_root());
}
