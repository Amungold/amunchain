use amun_nft_constitutional_registry::{NftConstitutionalRecord, ConstitutionalRegistry};
use amun_nft_royalty::RoyaltyPolicy;
use amun_nft_governance::GovernanceRight;
use amun_nft_bridge::BridgeLock;

#[test]
fn n140_register_and_query_record() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [1u8; 32];
    let record = NftConstitutionalRecord {
        token_id: token,
        owner: [10u8; 32],
        collection_id: Some([20u8; 32]),
        creator: [30u8; 32],
        mining_origin: Some("ValidatorOperation".into()),
        royalty_policy: Some(RoyaltyPolicy { creator: [30u8; 32], royalty_bps: 500 }),
        governance_right: Some(GovernanceRight { token_id: token, owner: [10u8; 32], can_propose: true, can_veto: false, voting_power: 10 }),
        bridge_lock: None,
    };
    reg.register(record);
    let fetched = reg.get(&token).unwrap();
    assert_eq!(fetched.owner, [10u8; 32]);
    assert_eq!(fetched.royalty_policy.as_ref().unwrap().royalty_bps, 500);
    assert!(fetched.governance_right.as_ref().unwrap().can_propose);
}

#[test]
fn n140_multiple_tokens_independent() {
    let mut reg = ConstitutionalRegistry::new();
    let t1 = [1u8; 32];
    let t2 = [2u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: t1, owner: [10u8; 32], collection_id: None, creator: [30u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None, bridge_lock: None,
    });
    reg.register(NftConstitutionalRecord {
        token_id: t2, owner: [20u8; 32], collection_id: None, creator: [40u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None, bridge_lock: None,
    });
    assert_eq!(reg.get(&t1).unwrap().owner, [10u8; 32]);
    assert_eq!(reg.get(&t2).unwrap().owner, [20u8; 32]);
}

#[test]
fn n140_deterministic_constitutional_root() {
    let mut r1 = ConstitutionalRegistry::new();
    let mut r2 = ConstitutionalRegistry::new();
    let record = NftConstitutionalRecord {
        token_id: [5u8; 32], owner: [50u8; 32], collection_id: None, creator: [60u8; 32],
        mining_origin: Some("BugDiscovery".into()),
        royalty_policy: Some(RoyaltyPolicy { creator: [60u8; 32], royalty_bps: 1000 }),
        governance_right: Some(GovernanceRight { token_id: [5u8; 32], owner: [50u8; 32], can_propose: false, can_veto: true, voting_power: 100 }),
        bridge_lock: None,
    };
    r1.register(record.clone());
    r2.register(record);
    assert_eq!(r1.compute_constitutional_root(), r2.compute_constitutional_root());
}

#[test]
fn n140_root_changes_with_different_bridge_lock() {
    let mut r1 = ConstitutionalRegistry::new();
    let mut r2 = ConstitutionalRegistry::new();
    let base = NftConstitutionalRecord {
        token_id: [7u8; 32], owner: [70u8; 32], collection_id: None, creator: [80u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None, bridge_lock: None,
    };
    r1.register(base.clone());
    r2.register(base);
    // Add bridge lock to r1
    let mut r1_record = r1.get(&[7u8; 32]).unwrap().clone();
    r1_record.bridge_lock = Some(BridgeLock {
        source_chain: 1, token_id: [7u8; 32], owner: [70u8; 32],
        destination_chain: 2, destination_owner: [90u8; 32], lock_height: 100,
    });
    r1.register(r1_record);
    assert_ne!(r1.compute_constitutional_root(), r2.compute_constitutional_root());
}

#[test]
fn n140_root_changes_with_different_governance() {
    let mut r1 = ConstitutionalRegistry::new();
    let mut r2 = ConstitutionalRegistry::new();
    let base = NftConstitutionalRecord {
        token_id: [9u8; 32], owner: [90u8; 32], collection_id: None, creator: [100u8; 32],
        mining_origin: None, royalty_policy: None, governance_right: None, bridge_lock: None,
    };
    r1.register(base.clone());
    r2.register(base);
    let mut r2_record = r2.get(&[9u8; 32]).unwrap().clone();
    r2_record.governance_right = Some(GovernanceRight {
        token_id: [9u8; 32], owner: [90u8; 32], can_propose: true, can_veto: false, voting_power: 1,
    });
    r2.register(r2_record);
    assert_ne!(r1.compute_constitutional_root(), r2.compute_constitutional_root());
}
