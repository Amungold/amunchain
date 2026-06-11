use amun_consensus::action::{ActionLog, ConsensusAction};
use amun_constitutional_block::{
    finalizer::{BlockFinalizer, FinalizationContext},
    Blockchain,
};
use amun_constitutional_state::ConstitutionalStateRuntime;

#[test]
fn test_commit_creates_block_with_state_root() {
    let mut chain = Blockchain::new();
    let mut log = ActionLog::default();
    let mut state = ConstitutionalStateRuntime::new();

    log.record(ConsensusAction::None, 1, 0, "Test", [1u8; 32]);
    state.apply_transition(&[1u8; 32], &[0xAA; 32]);

    let ctx = FinalizationContext {
        state_runtime: Some(state),
        pre_state_root: [0u8; 32],
        governance_root: "gov".into(),
        execution_root: "exec".into(),
        timestamp: "t".into(),
        proposer: "p".into(),
    };

    let block = BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
    assert_eq!(chain.blocks.len(), 1);
    assert!(!block.state_root.is_empty());
    assert!(!block.replay_certificate_root.is_empty());
}

#[test]
fn test_state_root_changes_block_hash() {
    let mut chain1 = Blockchain::new();
    let mut chain2 = Blockchain::new();
    let log = ActionLog::default();

    let mut state1 = ConstitutionalStateRuntime::new();
    let mut state2 = ConstitutionalStateRuntime::new();
    state1.apply_transition(&[1u8; 32], &[0xAA; 32]);
    state2.apply_transition(&[1u8; 32], &[0xBB; 32]);

    let ctx1 = FinalizationContext {
        state_runtime: Some(state1),
        pre_state_root: [0u8; 32],
        governance_root: "g".into(),
        execution_root: "e".into(),
        timestamp: "t".into(),
        proposer: "p".into(),
    };
    let ctx2 = FinalizationContext {
        state_runtime: Some(state2),
        pre_state_root: [0u8; 32],
        governance_root: "g".into(),
        execution_root: "e".into(),
        timestamp: "t".into(),
        proposer: "p".into(),
    };

    let b1 = BlockFinalizer::finalize(&mut chain1, &log, ctx1).unwrap();
    let b2 = BlockFinalizer::finalize(&mut chain2, &log, ctx2).unwrap();

    assert_ne!(b1.state_root, b2.state_root);
    assert_ne!(b1.block_hash, b2.block_hash);
    assert_ne!(b1.replay_certificate_root, b2.replay_certificate_root);
}
