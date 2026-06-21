use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_nft_marketplace::MarketplaceEngine;

pub struct StressTestResult {
    pub total_operations: u64,
    pub successful: u64,
    pub failed: u64,
    pub registry_size: usize,
    pub marketplace_listings: usize,
}

pub fn run_stress_mint(
    registry: &mut ResourceRegistry,
    collection_id: [u8; 32],
    count: u64,
) -> StressTestResult {
    let col_id = ResourceId(collection_id);
    let mut success = 0u64;
    let mut failed = 0u64;
    for i in 0..count {
        let token_id = ResourceId(stress_token_id(i));
        let meta = ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(
                token_id,
                col_id,
                registry.resource_hash(&col_id).unwrap_or([0u8; 32]),
                registry.get(&col_id).map(|m| m.lineage.version + 1).unwrap_or(1),
            ),
            contract_id: [0u8; 32],
            owner: [0u8; 32],
        };
        match registry.derive_from_collection(&col_id, meta) {
            Ok(_) => success += 1,
            Err(_) => failed += 1,
        }
    }
    StressTestResult {
        total_operations: count,
        successful: success,
        failed,
        registry_size: registry.total_active(),
        marketplace_listings: 0,
    }
}

pub fn run_stress_marketplace(
    registry: &mut ResourceRegistry,
    marketplace: &mut MarketplaceEngine,
    token_ids: &[[u8; 32]],
    rounds: u64,
) -> StressTestResult {
    let mut success = 0u64;
    let mut failed = 0u64;
    for _ in 0..rounds {
        for &tid in token_ids {
            let token = ResourceId(tid);
            let seller = [1u8; 32];
            let buyer = [2u8; 32];
            if marketplace.list_nft(registry, token, &seller, 100, None).is_ok()
                && marketplace.buy_nft(registry, &token, &buyer, 1, 1000).is_ok()
            {
                success += 1;
                continue;
            }
            failed += 1;
        }
    }
    StressTestResult {
        total_operations: rounds * token_ids.len() as u64,
        successful: success,
        failed,
        registry_size: registry.total_active(),
        marketplace_listings: marketplace.event_log().len(),
    }
}

fn stress_token_id(index: u64) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(index.to_le_bytes());
    hasher.finalize().into()
}
