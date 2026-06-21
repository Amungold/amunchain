use amun_nft_royalty::RoyaltyPolicy;
use amun_nft_governance::GovernanceRight;
use amun_nft_bridge::{BridgeLock, BridgeLedger};
use amun_nft_constitutional_registry::{NftConstitutionalRecord, ConstitutionalRegistry};
use amun_nft_constitutional_enforcement::EnforcementEngine;

#[test]
fn n140_1_cannot_sell_bridge_locked_nft() {
    let mut reg = ConstitutionalRegistry::new();
    let mut bridge = BridgeLedger::new();
    let token = [1u8; 32];
    let lock = BridgeLock {
        source_chain: 1, token_id: token, owner: [10u8; 32],
        destination_chain: 2, destination_owner: [20u8; 32], lock_height: 42,
    };
    let lock_id = bridge.lock(lock);
    let record = NftConstitutionalRecord {
        token_id: token, owner: [10u8; 32], collection_id: None, creator: [10u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None,
        bridge_lock: Some(bridge.locks.get(&lock_id).unwrap().clone()),
    };
    reg.register(record);
    assert!(!EnforcementEngine::can_be_sold(&reg, &bridge, &token));
}

#[test]
fn n140_1_can_sell_unlocked_nft() {
    let mut reg = ConstitutionalRegistry::new();
    let bridge = BridgeLedger::new();
    let token = [2u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token, owner: [10u8; 32], collection_id: None, creator: [10u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None, bridge_lock: None,
    });
    assert!(EnforcementEngine::can_be_sold(&reg, &bridge, &token));
}

#[test]
fn n140_1_governance_transfers_with_ownership() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [3u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token, owner: [10u8; 32], collection_id: None, creator: [10u8; 32],
        mining_origin: None, royalty_policy: None,
        governance_right: Some(GovernanceRight {
            token_id: token, owner: [10u8; 32], can_propose: true, can_veto: false, voting_power: 50,
        }),
        bridge_lock: None,
    });
    EnforcementEngine::transfer_governance(&mut reg, &token, &[20u8; 32]);
    let updated = reg.get(&token).unwrap();
    assert_eq!(updated.owner, [20u8; 32]);
    assert_eq!(updated.governance_right.as_ref().unwrap().owner, [20u8; 32]);
}

#[test]
fn n140_1_royalty_enforced_on_sale() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [4u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token, owner: [10u8; 32], collection_id: None, creator: [30u8; 32],
        mining_origin: None,
        royalty_policy: Some(RoyaltyPolicy { creator: [30u8; 32], royalty_bps: 500 }),
        governance_right: None, bridge_lock: None,
    });
    let royalty = EnforcementEngine::enforce_royalty(&reg, &token, 1000);
    assert_eq!(royalty, Some(50));
}

#[test]
fn n140_1_unified_constitutional_proof() {
    let mut reg = ConstitutionalRegistry::new();
    reg.register(NftConstitutionalRecord {
        token_id: [5u8; 32], owner: [50u8; 32], collection_id: None, creator: [60u8; 32],
        mining_origin: Some("Genesis".into()),
        royalty_policy: Some(RoyaltyPolicy { creator: [60u8; 32], royalty_bps: 250 }),
        governance_right: Some(GovernanceRight {
            token_id: [5u8; 32], owner: [50u8; 32], can_propose: true, can_veto: true, voting_power: 100,
        }),
        bridge_lock: None,
    });
    let proof = EnforcementEngine::produce_constitutional_proof(
        &reg,
        [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32],
    );
    assert_ne!(proof.unified_root, [0u8; 32]);
    let proof2 = EnforcementEngine::produce_constitutional_proof(
        &reg,
        [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32],
    );
    assert_eq!(proof.unified_root, proof2.unified_root);
}

#[test]
fn n140_1_full_integration_flow() {
    let mut reg = ConstitutionalRegistry::new();
    let bridge = BridgeLedger::new();
    let token = [9u8; 32];
    let creator = [100u8; 32];
    let owner = [200u8; 32];
    let buyer = [255u8; 32];

    // Mint: register constitutional record
    reg.register(NftConstitutionalRecord {
        token_id: token, owner, collection_id: Some([1u8; 32]), creator,
        mining_origin: Some("ValidatorOperation".into()),
        royalty_policy: Some(RoyaltyPolicy { creator, royalty_bps: 1000 }),
        governance_right: Some(GovernanceRight {
            token_id: token, owner, can_propose: true, can_veto: false, voting_power: 10,
        }),
        bridge_lock: None,
    });

    // Pre-sale checks
    assert!(EnforcementEngine::can_be_sold(&reg, &bridge, &token));
    let royalty = EnforcementEngine::enforce_royalty(&reg, &token, 5000);
    assert_eq!(royalty, Some(500));

    // Simulate sale: transfer governance to buyer
    EnforcementEngine::transfer_governance(&mut reg, &token, &buyer);
    let updated = reg.get(&token).unwrap();
    assert_eq!(updated.owner, buyer);
    assert_eq!(updated.governance_right.as_ref().unwrap().owner, buyer);

    // Produce unified proof
    let proof = EnforcementEngine::produce_constitutional_proof(
        &reg, [0xAu8; 32], [0xBu8; 32], [0xCu8; 32], [0xDu8; 32],
    );
    assert_ne!(proof.unified_root, [0u8; 32]);
}
