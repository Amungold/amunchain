use amun_consensus::action::{ActionLog, ConsensusAction};
use amun_constitutional_block::{
    finalizer::{BlockFinalizer, FinalizationContext},
    Blockchain,
};

#[test]
fn test_evidence_root_matches_block() {
    let mut chain = Blockchain::new();
    let mut log = ActionLog::default();
    log.record(ConsensusAction::None, 1, 0, "Test", [1u8; 32]);

    let ctx = FinalizationContext {
        state_runtime: None,
        pre_state_root: [0u8; 32],
        governance_root: "g".into(),
        execution_root: "e".into(),
        timestamp: "t".into(),
        proposer: "p".into(),
    };
    BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
    chain.verify_block_evidence(0, &log).unwrap();
}

#[test]
fn test_tampered_log_fails_verification() {
    let mut chain = Blockchain::new();
    let mut log = ActionLog::default();
    log.record(ConsensusAction::None, 1, 0, "Test", [1u8; 32]);

    let ctx = FinalizationContext {
        state_runtime: None,
        pre_state_root: [0u8; 32],
        governance_root: "g".into(),
        execution_root: "e".into(),
        timestamp: "t".into(),
        proposer: "p".into(),
    };
    BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();

    let mut tampered = ActionLog::default();
    tampered.record(ConsensusAction::None, 1, 0, "Tampered", [99u8; 32]);
    assert!(chain.verify_block_evidence(0, &tampered).is_err());
}

#[test]
fn test_full_chain_replay_audit() {
    let mut chain = Blockchain::new();
    let mut logs = Vec::new();
    for i in 0..3u8 {
        let mut log = ActionLog::default();
        log.record(ConsensusAction::None, 1, 0, "Round", [i; 32]);
        let ctx = FinalizationContext {
            state_runtime: None,
            pre_state_root: [0u8; 32],
            governance_root: "g".into(),
            execution_root: "e".into(),
            timestamp: "t".into(),
            proposer: "p".into(),
        };
        BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
        logs.push(log);
    }
    chain.verify_chain_evidence(&logs).unwrap();
}
