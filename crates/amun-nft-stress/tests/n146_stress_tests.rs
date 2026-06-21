use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_nft_marketplace::MarketplaceEngine;
use amun_nft_stress::{run_stress_mint, run_stress_marketplace};
use sha2::{Sha256, Digest};

fn token_id(seed: u8, salt: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([seed]);
    hasher.update(salt.to_le_bytes());
    hasher.finalize().into()
}

#[test]
fn n146_stress_mint_1000_nfts() {
    let mut reg = ResourceRegistry::new(10000);
    let col_id = [1u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: ResourceId(col_id),
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(ResourceId(col_id)),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    }).unwrap();
    let result = run_stress_mint(&mut reg, col_id, 1000);
    assert_eq!(result.successful, 1000);
    assert_eq!(result.failed, 0);
    assert!(result.registry_size >= 1000);
}

#[test]
fn n146_stress_marketplace_rapid_trades() {
    let mut reg = ResourceRegistry::new(1000);
    let col_id = [1u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: ResourceId(col_id),
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(ResourceId(col_id)),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    }).unwrap();
    let tokens: Vec<[u8; 32]> = (0..10).map(|i| {
        let tid = token_id(i as u8, i as u64);
        let meta = ResourceMetadata {
            resource_id: ResourceId(tid),
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(
                ResourceId(tid), ResourceId(col_id),
                reg.resource_hash(&ResourceId(col_id)).unwrap(),
                reg.get(&ResourceId(col_id)).unwrap().lineage.version + 1,
            ),
            contract_id: [0u8; 32],
            owner: [1u8; 32],
        };
        reg.derive_from_collection(&ResourceId(col_id), meta).unwrap();
        tid
    }).collect();
    let mut mp = MarketplaceEngine::new();
    let result = run_stress_marketplace(&mut reg, &mut mp, &tokens, 100);
    assert!(result.successful > 0);
    assert!(result.marketplace_listings > 0);
}

#[test]
fn n146_stress_state_root_consistent_under_load() {
    let mut reg1 = ResourceRegistry::new(1000);
    let mut reg2 = ResourceRegistry::new(1000);
    let col_id = [1u8; 32];
    for reg in [&mut reg1, &mut reg2].iter_mut() {
        reg.register_genesis(ResourceMetadata {
            resource_id: ResourceId(col_id),
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(ResourceId(col_id)),
            contract_id: [0u8; 32],
            owner: [0u8; 32],
        }).unwrap();
    }
    let result1 = run_stress_mint(&mut reg1, col_id, 500);
    let result2 = run_stress_mint(&mut reg2, col_id, 500);
    assert_eq!(result1.successful, result2.successful);
    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
}

#[test]
fn n147_full_constitutional_flow() {
    use amun_resource_core::{
        ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
        ResourceLineage, ResourceRegistry,
    };
    use amun_nft_marketplace::MarketplaceEngine;
    use amun_nft_constitutional_registry::{NftConstitutionalRecord, ConstitutionalRegistry};
    use amun_nft_royalty::RoyaltyPolicy;
    use amun_nft_governance::{GovernanceRight, GovernanceLedger};
    use amun_nft_bridge::{BridgeLock, BridgeLedger};
    use amun_nft_royalty_accounting::RoyaltyLedger;
    use amun_nft_constitutional_enforcement::EnforcementEngine;

    let mut reg = ResourceRegistry::new(1000);
    let mut marketplace = MarketplaceEngine::new();
    let mut constitutional = ConstitutionalRegistry::new();
    let mut royalty_ledger = RoyaltyLedger::new();
    let mut gov_ledger = GovernanceLedger::new();
    let mut bridge_ledger = BridgeLedger::new();

    let creator = [1u8; 32];
    let minter = [2u8; 32];
    let buyer = [3u8; 32];
    let col_id = [10u8; 32];
    let token_id = [20u8; 32];

    // 1. Register collection
    reg.register_genesis(ResourceMetadata {
        resource_id: ResourceId(col_id),
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(ResourceId(col_id)),
        contract_id: [0u8; 32],
        owner: creator,
    }).unwrap();

    // 2. Mint NFT
    let parent_hash = reg.resource_hash(&ResourceId(col_id)).unwrap();
    reg.derive_from_collection(&ResourceId(col_id), ResourceMetadata {
        resource_id: ResourceId(token_id),
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(
            ResourceId(token_id), ResourceId(col_id),
            parent_hash,
            reg.get(&ResourceId(col_id)).unwrap().lineage.version + 1,
        ),
        contract_id: [0u8; 32],
        owner: minter,
    }).unwrap();

    // 3. Register constitutional record with royalty and governance
    constitutional.register(NftConstitutionalRecord {
        token_id, owner: minter, collection_id: Some(col_id), creator,
        mining_origin: Some("Validator".into()),
        royalty_policy: Some(RoyaltyPolicy { creator, royalty_bps: 500 }),
        governance_right: Some(GovernanceRight {
            token_id, owner: minter, can_propose: true, can_veto: false, voting_power: 100,
        }),
        bridge_lock: None,
    });
    gov_ledger.set_rights(GovernanceRight {
        token_id, owner: minter, can_propose: true, can_veto: false, voting_power: 100,
    });

    let root_before_sale = constitutional.compute_constitutional_root();

    // 4. Marketplace sale
    marketplace.list_nft(&reg, ResourceId(token_id), &minter, 1000, None).unwrap();
    let _new_id = marketplace.buy_nft(&mut reg, &ResourceId(token_id), &buyer, 1, 1000).unwrap();

    // 5. Royalty settlement
    let royalty_amount = EnforcementEngine::enforce_royalty(&constitutional, &token_id, 1000).unwrap();
    royalty_ledger.settle(&amun_nft_royalty::RoyaltyRecord {
        token_id, creator, payer: buyer, sale_price: 1000, royalty_amount, block_height: 1,
    });
    assert_eq!(royalty_amount, 50);
    assert_eq!(royalty_ledger.balance_of(&creator), 50);

    // 6. Governance transfer
    EnforcementEngine::transfer_governance(&mut constitutional, &token_id, &buyer);
    let updated = constitutional.get(&token_id).unwrap();
    assert_eq!(updated.owner, buyer);
    assert_eq!(updated.governance_right.as_ref().unwrap().owner, buyer);

    // 7. Bridge lock and unlock
    let lock = BridgeLock {
        source_chain: 1, token_id, owner: buyer,
        destination_chain: 2, destination_owner: [9u8; 32], lock_height: 100,
    };
    let lock_id = bridge_ledger.lock(lock);
    // Update registry with bridge lock
    let mut locked_record = updated.clone();
    locked_record.bridge_lock = Some(bridge_ledger.locks.get(&lock_id).unwrap().clone());
    constitutional.register(locked_record);
    assert!(!EnforcementEngine::can_be_sold(&constitutional, &bridge_ledger, &token_id));

    // Unlock
    bridge_ledger.unlock(amun_nft_bridge::BridgeUnlock {
        lock_id, destination_chain: 2, new_owner: buyer, unlock_height: 200,
    });
    let mut unlocked_record = constitutional.get(&token_id).unwrap().clone();
    unlocked_record.bridge_lock = None;
    constitutional.register(unlocked_record);
    assert!(EnforcementEngine::can_be_sold(&constitutional, &bridge_ledger, &token_id));

    // 8. Final constitutional root differs from start
    let root_after_all = constitutional.compute_constitutional_root();
    assert_ne!(root_before_sale, root_after_all);

    // 9. State root consistency
    let state_root = reg.compute_state_root();
    assert_ne!(state_root, [0u8; 32]);
}
