use amun_defi_governance::GovernanceEngine;

#[test]
fn n157_propose_vote_and_execute() {
    let mut engine = GovernanceEngine::new();
    let proposer = [10u8; 32];
    let voter = [20u8; 32];
    let prop_id = engine.propose(proposer, "amm_fee_bps".into(), 50);
    engine.vote(&prop_id, voter, true, 100);
    assert!(engine.execute(&prop_id));
    assert_eq!(engine.parameters.amm_fee_bps, 50);
}

#[test]
fn n157_governance_root_deterministic() {
    let mut engine1 = GovernanceEngine::new();
    let mut engine2 = GovernanceEngine::new();
    let id1 = engine1.propose([10u8; 32], "collateral_ratio_min".into(), 200);
    let id2 = engine2.propose([10u8; 32], "collateral_ratio_min".into(), 200);
    engine1.vote(&id1, [20u8; 32], true, 100);
    engine2.vote(&id2, [20u8; 32], true, 100);
    engine1.execute(&id1);
    engine2.execute(&id2);
    assert_eq!(
        engine1.compute_governance_root(),
        engine2.compute_governance_root()
    );
}

#[test]
fn n157_parameter_change_detected() {
    let mut engine = GovernanceEngine::new();
    let root_before = engine.compute_governance_root();
    let prop_id = engine.propose([10u8; 32], "liquidation_threshold".into(), 7500);
    engine.vote(&prop_id, [20u8; 32], true, 100);
    engine.execute(&prop_id);
    let root_after = engine.compute_governance_root();
    assert_ne!(root_before, root_after);
}
