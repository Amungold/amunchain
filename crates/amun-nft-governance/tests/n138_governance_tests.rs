use amun_nft_governance::{GovernanceLedger, GovernanceRight};

#[test]
fn n138_grant_and_check_rights() {
    let mut ledger = GovernanceLedger::new();
    let token = [1u8; 32];
    let owner = [10u8; 32];
    ledger.set_rights(GovernanceRight {
        token_id: token,
        owner,
        can_propose: true,
        can_veto: false,
        voting_power: 100,
    });
    assert!(ledger.can_propose(&token, &owner));
    assert!(!ledger.can_veto(&token, &owner));
    assert_eq!(ledger.voting_power(&token, &owner), 100);
    // Wrong owner cannot propose
    assert!(!ledger.can_propose(&token, &[99u8; 32]));
}

#[test]
fn n138_revoke_rights() {
    let mut ledger = GovernanceLedger::new();
    let token = [2u8; 32];
    let owner = [20u8; 32];
    ledger.set_rights(GovernanceRight {
        token_id: token,
        owner,
        can_propose: true,
        can_veto: true,
        voting_power: 50,
    });
    ledger.revoke_rights(&token);
    assert!(!ledger.can_propose(&token, &owner));
    assert!(!ledger.can_veto(&token, &owner));
    assert_eq!(ledger.voting_power(&token, &owner), 0);
}

#[test]
fn n138_multiple_tokens_independent_rights() {
    let mut ledger = GovernanceLedger::new();
    let t1 = [1u8; 32];
    let t2 = [2u8; 32];
    let o1 = [10u8; 32];
    let o2 = [20u8; 32];
    ledger.set_rights(GovernanceRight {
        token_id: t1,
        owner: o1,
        can_propose: true,
        can_veto: false,
        voting_power: 10,
    });
    ledger.set_rights(GovernanceRight {
        token_id: t2,
        owner: o2,
        can_propose: false,
        can_veto: true,
        voting_power: 20,
    });
    assert_eq!(ledger.voting_power(&t1, &o1), 10);
    assert_eq!(ledger.voting_power(&t2, &o2), 20);
    assert!(!ledger.can_propose(&t1, &o2));
}

#[test]
fn n138_deterministic_governance_root() {
    let mut l1 = GovernanceLedger::new();
    let mut l2 = GovernanceLedger::new();
    let right = GovernanceRight {
        token_id: [5u8; 32],
        owner: [50u8; 32],
        can_propose: true,
        can_veto: true,
        voting_power: 1000,
    };
    l1.set_rights(right.clone());
    l2.set_rights(right);
    assert_eq!(l1.compute_governance_root(), l2.compute_governance_root());
}

#[test]
fn n138_revoked_changes_root() {
    let mut l1 = GovernanceLedger::new();
    let mut l2 = GovernanceLedger::new();
    let token = [7u8; 32];
    let right = GovernanceRight {
        token_id: token,
        owner: [70u8; 32],
        can_propose: true,
        can_veto: false,
        voting_power: 5,
    };
    l1.set_rights(right.clone());
    l2.set_rights(right);
    l2.revoke_rights(&token);
    assert_ne!(l1.compute_governance_root(), l2.compute_governance_root());
}
