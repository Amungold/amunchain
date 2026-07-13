use amun_tokenomics::EpochEconomics;
use amun_tokenomics_ledger::EconomicLedger;

fn sample_economics() -> EpochEconomics {
    let mut e = EpochEconomics::new();
    e.distribute_epoch(1_000_000);
    e
}

#[test]
fn test_ledger_epoch_advances() {
    let mut ledger = EconomicLedger::new();
    let economics = sample_economics();

    for _ in 0..216_000u64 {
        ledger.on_block_finalized(&economics);
    }

    assert_eq!(
        ledger.current_epoch, 1,
        "Epoch should advance after BLOCKS_PER_EPOCH"
    );

    assert!(
        ledger.last_epoch_reward > 0,
        "Should have recorded a reward"
    );

    assert!(
        ledger.treasury_balance > 0,
        "Treasury should have accumulated"
    );

    assert!(
        ledger.validator_reward_pool > 0,
        "Validator pool should have accumulated"
    );

    assert!(
        ledger.ecosystem_reward_pool > 0,
        "Ecosystem pool should have accumulated"
    );
}

#[test]
fn test_ledger_root_deterministic() {
    let mut l1 = EconomicLedger::new();
    let mut l2 = EconomicLedger::new();

    let economics = sample_economics();

    for _ in 0..500_000u64 {
        l1.on_block_finalized(&economics);
        l2.on_block_finalized(&economics);
    }

    assert_eq!(l1.compute_ledger_root(), l2.compute_ledger_root());
}

#[test]
fn test_multiple_epochs() {
    let mut ledger = EconomicLedger::new();
    let economics = sample_economics();

    for _ in 0..(216_000 * 3) {
        ledger.on_block_finalized(&economics);
    }

    assert_eq!(ledger.current_epoch, 3, "Should have completed 3 epochs");

    assert!(ledger.total_issued_ntr > 0, "Should have issued NTR");
}
