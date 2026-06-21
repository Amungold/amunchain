use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use rand::Rng;

pub struct FuzzResult {
    pub test_name: String,
    pub iterations: u64,
    pub crashes: u64,
    pub state_invariants_broken: u64,
}

impl FuzzResult {
    pub fn new(test_name: &str) -> Self {
        Self { test_name: test_name.to_string(), iterations: 0, crashes: 0, state_invariants_broken: 0 }
    }

    pub fn passed(&self) -> bool {
        self.crashes == 0 && self.state_invariants_broken == 0
    }
}

/// Fuzz mint operations with random inputs
pub fn fuzz_mint(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new("fuzz_mint");
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(1000);
        let col_id = ResourceId(rng.gen::<[u8; 32]>());

        // Register collection
        if reg.register_genesis(ResourceMetadata {
            resource_id: col_id,
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(col_id),
            contract_id: [0u8; 32],
            owner: rng.gen::<[u8; 32]>(),
        }).is_err() {
            result.iterations += 1;
            continue;
        }

        // Mint random number of NFTs
        let count = rng.gen_range(1..50);
        let initial_active = reg.total_active();

        for _ in 0..count {
            let token_id = ResourceId(rng.gen::<[u8; 32]>());
            let parent_hash = reg.resource_hash(&col_id).unwrap_or([0u8; 32]);
            let version = reg.get(&col_id).map(|m| m.lineage.version + 1).unwrap_or(1);

            if reg.derive_from_collection(&col_id, ResourceMetadata {
                resource_id: token_id,
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
                contract_id: [0u8; 32],
                owner: rng.gen::<[u8; 32]>(),
            }).is_err() {
                result.crashes += 1;
            }
        }

        // Invariant: collection must remain active
        if let Some(col) = reg.get(&col_id) {
            if !matches!(col.state, ResourceState::Active) {
                result.state_invariants_broken += 1;
            }
        }

        // Invariant: active count >= minted NFTs
        if reg.total_active() < initial_active {
            result.state_invariants_broken += 1;
        }

        result.iterations += 1;
    }
    result
}

/// Fuzz marketplace with random operations
pub fn fuzz_marketplace(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new("fuzz_marketplace");
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(1000);
        let mut mp = amun_nft_marketplace::MarketplaceEngine::new();

        // Create a token
        let token_id = ResourceId(rng.gen::<[u8; 32]>());
        let seller = rng.gen::<[u8; 32]>();
        reg.register_genesis(ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(token_id),
            contract_id: [0u8; 32],
            owner: seller,
        }).ok();

        // Random operations
        let ops = rng.gen_range(1..20);
        for _ in 0..ops {
            match rng.gen_range(0..4) {
                0 => { mp.list_nft(&reg, token_id, &seller, rng.gen::<u64>(), None).ok(); }
                1 => {
                    let buyer = rng.gen::<[u8; 32]>();
                    if buyer != seller {
                        mp.buy_nft(&mut reg, &token_id, &buyer, 1, rng.gen::<u64>()).ok();
                    }
                }
                2 => { mp.cancel_listing(&token_id).ok(); }
                3 => {
                    mp.start_auction(&reg, token_id, &seller, rng.gen::<u64>()).ok();
                    let bidder = rng.gen::<[u8; 32]>();
                    if bidder != seller {
                        mp.place_bid(&token_id, &bidder, rng.gen::<u64>(), rng.gen::<u64>()).ok();
                    }
                }
                _ => {}
            }
        }

        // Invariant: marketplace should not crash the registry
        if reg.total() == 0 {
            result.state_invariants_broken += 1;
        }

        result.iterations += 1;
    }
    result
}

/// Fuzz royalty computation with extreme values
pub fn fuzz_royalty(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new("fuzz_royalty");
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let sale_price = rng.gen::<u64>();
        let bps = rng.gen_range(0..10001u16);

        // Should never panic
        let amount = amun_nft_royalty::RoyaltyEngine::compute_royalty(sale_price, bps);

        // Invariant: royalty cannot exceed sale price
        if amount > sale_price && bps < 10000 {
            result.state_invariants_broken += 1;
        }

        result.iterations += 1;
    }
    result
}

/// Fuzz governance with random rights
pub fn fuzz_governance(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new("fuzz_governance");
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut ledger = amun_nft_governance::GovernanceLedger::new();
        let token = rng.gen::<[u8; 32]>();
        let owner = rng.gen::<[u8; 32]>();

        ledger.set_rights(amun_nft_governance::GovernanceRight {
            token_id: token,
            owner,
            can_propose: rng.gen::<bool>(),
            can_veto: rng.gen::<bool>(),
            voting_power: rng.gen::<u64>(),
        });

        // Invariant: after revoke, cannot propose or veto
        ledger.revoke_rights(&token);
        if ledger.can_propose(&token, &owner) || ledger.can_veto(&token, &owner) {
            result.state_invariants_broken += 1;
        }

        result.iterations += 1;
    }
    result
}

/// Fuzz bridge with random locks/unlocks
pub fn fuzz_bridge(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new("fuzz_bridge");
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut ledger = amun_nft_bridge::BridgeLedger::new();
        let lock = amun_nft_bridge::BridgeLock {
            source_chain: rng.gen::<u32>(),
            token_id: rng.gen::<[u8; 32]>(),
            owner: rng.gen::<[u8; 32]>(),
            destination_chain: rng.gen::<u32>(),
            destination_owner: rng.gen::<[u8; 32]>(),
            lock_height: rng.gen::<u64>(),
        };
        let lock_id = ledger.lock(lock);

        // Random unlock with potentially wrong id
        let unlock_id = if rng.gen::<bool>() { lock_id } else { rng.gen::<[u8; 32]>() };
        ledger.unlock(amun_nft_bridge::BridgeUnlock {
            lock_id: unlock_id,
            destination_chain: rng.gen::<u32>(),
            new_owner: rng.gen::<[u8; 32]>(),
            unlock_height: rng.gen::<u64>(),
        });

        // Invariant: bridge root should be deterministic
        let root1 = ledger.compute_bridge_root();
        let root2 = ledger.compute_bridge_root();
        if root1 != root2 {
            result.state_invariants_broken += 1;
        }

        result.iterations += 1;
    }
    result
}
