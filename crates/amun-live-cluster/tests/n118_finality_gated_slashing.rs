// N118 — Finality-Gated Slashing Verification
// Verifies that slashing executes in the finality path and reduces stake.

use amun_consensus_network::{
    EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor, StakingAdapter,
    ValidatorIdentity,
};
use amun_kernel_types::PublicKey;
use amun_staking::validator::ValidatorRegistry;

#[test]
fn n118_slashing_executes_after_finality() {
    let mut staking = ValidatorRegistry::new();
    let pk = PublicKey([0x42u8; 48]);
    let validator_id = [0x42u8; 32];
    staking.register(pk, 100_000).unwrap();
    let initial_stake = staking.total_stake;

    let mut misbehavior = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
    misbehavior.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
    misbehavior.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
    misbehavior.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);

    let mut executor = RealStakingExecutor::new(staking);
    executor
        .identity_registry
        .register(ValidatorIdentity::new(validator_id, [0x42u8; 48], 1))
        .unwrap();
    let mut adapter = StakingAdapter::new(misbehavior, executor);

    let result = adapter.try_slash(&validator_id);
    assert!(
        result.is_some(),
        "N118: Slashing must execute in finality path"
    );
    assert!(
        adapter.executor.registry.total_stake < initial_stake,
        "N118: Stake must decrease after finality-gated slash"
    );

    eprintln!("N118 PASSED: Slashing executes in finality-gated path");
}
