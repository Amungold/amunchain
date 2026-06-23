use amun_nft_marketplace::MarketplaceEngine;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use sha2::{Digest, Sha256};
use std::time::Instant;

fn unique_id(seed: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_le_bytes());
    hasher.finalize().into()
}

#[test]
fn n152_benchmark_mint_10k_nfts() {
    let mut reg = ResourceRegistry::new(20000);
    let col_id = ResourceId(unique_id(0));
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    let start = Instant::now();
    for i in 0..10000u64 {
        let token = unique_id(i + 1);
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: ResourceId(token),
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(
                    ResourceId(token),
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
    let elapsed = start.elapsed();
    println!("Mint 10k NFTs: {:?}", elapsed);
    assert!(
        elapsed.as_secs() < 10,
        "Mint 10k NFTs took too long: {:?}",
        elapsed
    );
    assert_eq!(reg.total_active(), 10001);
}

#[test]
fn n152_benchmark_rapid_trades() {
    let mut reg = ResourceRegistry::new(2000);
    let col_id = ResourceId(unique_id(0));
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    let mut tokens = Vec::new();
    for i in 0..100u64 {
        let token = unique_id(i + 1);
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: ResourceId(token),
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(
                    ResourceId(token),
                    col_id,
                    parent_hash,
                    version,
                ),
                contract_id: [0u8; 32],
                owner: [1u8; 32],
            },
        )
        .unwrap();
        tokens.push(token);
    }

    let mut mp = MarketplaceEngine::new();
    let start = Instant::now();
    let seller = [1u8; 32];
    let buyer = [2u8; 32];
    for _ in 0..500 {
        for &tid in &tokens {
            let token = ResourceId(tid);
            mp.list_nft(&reg, token, &seller, 100, None).ok();
            mp.buy_nft(&mut reg, &token, &buyer, 1, 1000).ok();
        }
    }
    let elapsed = start.elapsed();
    println!("500x100 trades: {:?}", elapsed);
    assert!(
        elapsed.as_secs() < 30,
        "Rapid trades took too long: {:?}",
        elapsed
    );
}

#[test]
fn n152_benchmark_state_root_10k() {
    let mut reg = ResourceRegistry::new(20000);
    let col_id = ResourceId(unique_id(0));
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    for i in 0..10000u64 {
        let token = unique_id(i + 1);
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: ResourceId(token),
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(
                    ResourceId(token),
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

    let start = Instant::now();
    let root = reg.compute_state_root();
    let elapsed = start.elapsed();
    println!("State root 10k NFTs: {:?}", elapsed);
    assert_ne!(root, [0u8; 32]);
    assert!(
        elapsed.as_millis() < 500,
        "State root computation took too long: {:?}",
        elapsed
    );
}
