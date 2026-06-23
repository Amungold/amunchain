use amun_nft_bridge::{BridgeLedger, BridgeLock};
use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
use amun_nft_governance::{GovernanceLedger, GovernanceRight};
use amun_nft_rights_enforcement::RightsEnforcementEngine;
use amun_nft_royalty::RoyaltyPolicy;

#[test]
fn n141_reject_unregistered_token() {
    let reg = ConstitutionalRegistry::new();
    let bridge = BridgeLedger::new();
    let gov = GovernanceLedger::new();
    let result = RightsEnforcementEngine::validate_transfer(
        &reg,
        &bridge,
        &gov,
        &[1u8; 32],
        &[10u8; 32],
        &[20u8; 32],
        100,
    );
    assert!(!result.allowed);
    assert_eq!(result.reason, Some("Token not registered".into()));
}

#[test]
fn n141_reject_non_owner_seller() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [2u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token,
        owner: [10u8; 32],
        collection_id: None,
        creator: [10u8; 32],
        mining_origin: None,
        royalty_policy: None,
        governance_right: None,
        bridge_lock: None,
    });
    let bridge = BridgeLedger::new();
    let gov = GovernanceLedger::new();
    let result = RightsEnforcementEngine::validate_transfer(
        &reg,
        &bridge,
        &gov,
        &token,
        &[99u8; 32],
        &[20u8; 32],
        100,
    );
    assert!(!result.allowed);
    assert_eq!(
        result.reason,
        Some("Seller is not constitutional owner".into())
    );
}

#[test]
fn n141_reject_bridge_locked_transfer() {
    let mut reg = ConstitutionalRegistry::new();
    let mut bridge = BridgeLedger::new();
    let gov = GovernanceLedger::new();
    let token = [3u8; 32];
    let lock = BridgeLock {
        source_chain: 1,
        token_id: token,
        owner: [10u8; 32],
        destination_chain: 2,
        destination_owner: [20u8; 32],
        lock_height: 42,
    };
    let lock_id = bridge.lock(lock);
    reg.register(NftConstitutionalRecord {
        token_id: token,
        owner: [10u8; 32],
        collection_id: None,
        creator: [10u8; 32],
        mining_origin: None,
        royalty_policy: None,
        governance_right: None,
        bridge_lock: Some(bridge.locks.get(&lock_id).unwrap().clone()),
    });
    let result = RightsEnforcementEngine::validate_transfer(
        &reg,
        &bridge,
        &gov,
        &token,
        &[10u8; 32],
        &[30u8; 32],
        200,
    );
    assert!(!result.allowed);
    assert_eq!(
        result.reason,
        Some("Token is locked in cross-chain bridge".into())
    );
}

#[test]
fn n141_allow_valid_transfer_with_royalty() {
    let mut reg = ConstitutionalRegistry::new();
    let mut gov = GovernanceLedger::new();
    let bridge = BridgeLedger::new();
    let token = [4u8; 32];
    let seller = [10u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: token,
        owner: seller,
        can_propose: true,
        can_veto: false,
        voting_power: 10,
    });
    reg.register(NftConstitutionalRecord {
        token_id: token,
        owner: seller,
        collection_id: None,
        creator: [30u8; 32],
        mining_origin: None,
        royalty_policy: Some(RoyaltyPolicy {
            creator: [30u8; 32],
            royalty_bps: 500,
        }),
        governance_right: Some(GovernanceRight {
            token_id: token,
            owner: seller,
            can_propose: true,
            can_veto: false,
            voting_power: 10,
        }),
        bridge_lock: None,
    });
    let result = RightsEnforcementEngine::validate_transfer(
        &reg,
        &bridge,
        &gov,
        &token,
        &seller,
        &[20u8; 32],
        1000,
    );
    assert!(result.allowed);
    assert_eq!(result.required_royalty, Some(50));
}

#[test]
fn n141_produce_enforcement_proof() {
    let mut reg = ConstitutionalRegistry::new();
    reg.register(NftConstitutionalRecord {
        token_id: [5u8; 32],
        owner: [50u8; 32],
        collection_id: None,
        creator: [60u8; 32],
        mining_origin: Some("Genesis".into()),
        royalty_policy: Some(RoyaltyPolicy {
            creator: [60u8; 32],
            royalty_bps: 250,
        }),
        governance_right: None,
        bridge_lock: None,
    });
    let proof = RightsEnforcementEngine::produce_enforcement_proof(
        &reg, [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [5u8; 32],
    );
    assert_ne!(proof, [0u8; 32]);
    let proof2 = RightsEnforcementEngine::produce_enforcement_proof(
        &reg, [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [5u8; 32],
    );
    assert_eq!(proof, proof2);
}
