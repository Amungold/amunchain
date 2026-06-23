use amun_constitutional_commitment::{
    compute_constitutional_root, EconomicError, EconomicSnapshot, EconomicTree, Hash32,
};

fn make_hash(byte: u8) -> Hash32 {
    [byte; 32]
}

fn sample_snapshot() -> EconomicSnapshot {
    EconomicSnapshot {
        total_supply: 1_000_000,
        treasury_balance: 100_000,
        validator_reward_pool: 50_000,
        ecosystem_pool: 30_000,
        burned_supply: 10_000,
        issued_supply: 200_000,
        staked_supply: 400_000,
        circulating_supply: 490_000, // 1M - 10k - 400k - 100k
    }
}

#[test]
fn identical_snapshots_produce_identical_root() {
    let snap = sample_snapshot();
    let root_a = EconomicTree::root(&snap).unwrap();
    let root_b = EconomicTree::root(&snap).unwrap();
    assert_eq!(root_a, root_b);
}

#[test]
fn treasury_balance_change_produces_different_root() {
    let mut snap = sample_snapshot();
    let root_a = EconomicTree::root(&snap).unwrap();
    snap.treasury_balance += 1;
    snap.circulating_supply -= 1; // keep consistency
    let root_b = EconomicTree::root(&snap).unwrap();
    assert_ne!(root_a, root_b);
}

#[test]
fn burned_supply_change_produces_different_root() {
    let mut snap = sample_snapshot();
    let root_a = EconomicTree::root(&snap).unwrap();
    snap.burned_supply += 1;
    snap.circulating_supply -= 1; // keep consistency
    let root_b = EconomicTree::root(&snap).unwrap();
    assert_ne!(root_a, root_b);
}

#[test]
fn invalid_circulating_supply_returns_error() {
    let mut snap = sample_snapshot();
    snap.circulating_supply += 1; // break invariant intentionally
    let result = EconomicTree::root(&snap);
    assert!(matches!(
        result,
        Err(EconomicError::InvalidCirculatingSupply { .. })
    ));
}

#[test]
fn economic_root_change_propagates_to_constitutional_root() {
    let snap = sample_snapshot();
    let econ_root_a = EconomicTree::root(&snap).unwrap();
    let const_root_a = compute_constitutional_root(
        make_hash(0x11),
        make_hash(0x22),
        make_hash(0x33),
        econ_root_a,
    );

    let mut snap2 = sample_snapshot();
    snap2.treasury_balance += 1;
    snap2.circulating_supply -= 1;
    let econ_root_b = EconomicTree::root(&snap2).unwrap();
    let const_root_b = compute_constitutional_root(
        make_hash(0x11),
        make_hash(0x22),
        make_hash(0x33),
        econ_root_b,
    );

    assert_ne!(econ_root_a, econ_root_b);
    assert_ne!(const_root_a, const_root_b);
}
