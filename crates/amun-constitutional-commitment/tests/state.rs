use amun_constitutional_commitment::{
    commitment_root, compute_constitutional_root, ConstitutionalCommitment, ConstitutionalState,
    EconomicSnapshot, EconomicTree, Hash32,
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
        circulating_supply: 490_000,
    }
}

fn make_commitment(id: Hash32, ev: Hash32, gv: Hash32, ec: Hash32) -> ConstitutionalCommitment {
    let cr = compute_constitutional_root(id, ev, gv, ec);
    ConstitutionalCommitment {
        version: 1,
        identity_root: id,
        evidence_root: ev,
        governance_root: gv,
        economic_root: ec,
        constitutional_root: cr,
    }
}

#[test]
fn save_and_load_roundtrip() {
    let snap = sample_snapshot();
    let ec = EconomicTree::root(&snap).unwrap();
    let commitment = make_commitment(make_hash(0x11), make_hash(0x22), make_hash(0x33), ec);

    let bytes = ConstitutionalState::save(&commitment);
    let loaded = ConstitutionalState::load(&bytes).unwrap();

    assert_eq!(commitment.version, loaded.version);
    assert_eq!(commitment.identity_root, loaded.identity_root);
    assert_eq!(commitment.evidence_root, loaded.evidence_root);
    assert_eq!(commitment.governance_root, loaded.governance_root);
    assert_eq!(commitment.economic_root, loaded.economic_root);
    assert_eq!(commitment.constitutional_root, loaded.constitutional_root);
}

#[test]
fn commitment_root_from_stored_bytes() {
    let snap = sample_snapshot();
    let ec = EconomicTree::root(&snap).unwrap();
    let commitment = make_commitment(make_hash(0x11), make_hash(0x22), make_hash(0x33), ec);

    let expected_root = commitment_root(&commitment);

    let bytes = ConstitutionalState::save(&commitment);
    let loaded = ConstitutionalState::load(&bytes).unwrap();
    let stored_root = commitment_root(&loaded);

    assert_eq!(expected_root, stored_root);
}

#[test]
fn state_root_changes_when_treasury_changes() {
    let snap = sample_snapshot();
    let ec_a = EconomicTree::root(&snap).unwrap();

    let mut snap_b = sample_snapshot();
    snap_b.treasury_balance += 1;
    snap_b.circulating_supply -= 1;
    let ec_b = EconomicTree::root(&snap_b).unwrap();

    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let com_a = make_commitment(id, ev, gv, ec_a);
    let com_b = make_commitment(id, ev, gv, ec_b);

    let root_a = commitment_root(&com_a);
    let root_b = commitment_root(&com_b);

    assert_ne!(ec_a, ec_b);
    assert_ne!(com_a.constitutional_root, com_b.constitutional_root);
    assert_ne!(root_a, root_b);
}

#[test]
fn state_root_changes_when_evidence_changes() {
    let snap = sample_snapshot();
    let ec = EconomicTree::root(&snap).unwrap();

    let id = make_hash(0x11);
    let ev_a = make_hash(0x22);
    let ev_b = make_hash(0xFF);
    let gv = make_hash(0x33);

    let com_a = make_commitment(id, ev_a, gv, ec);
    let com_b = make_commitment(id, ev_b, gv, ec);

    let root_a = commitment_root(&com_a);
    let root_b = commitment_root(&com_b);

    assert_ne!(com_a.constitutional_root, com_b.constitutional_root);
    assert_ne!(root_a, root_b);
}

#[test]
fn same_state_produces_same_commitment_root() {
    let snap = sample_snapshot();
    let ec = EconomicTree::root(&snap).unwrap();

    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let com_a = make_commitment(id, ev, gv, ec);
    let com_b = make_commitment(id, ev, gv, ec);

    let root_a = commitment_root(&com_a);
    let root_b = commitment_root(&com_b);

    assert_eq!(com_a.constitutional_root, com_b.constitutional_root);
    assert_eq!(root_a, root_b);
}

#[test]
fn load_invalid_data_returns_none() {
    assert!(ConstitutionalState::load(&[0u8; 10]).is_none());
    assert!(ConstitutionalState::load(&[0u8; 200]).is_none());
    assert!(ConstitutionalState::load(&[]).is_none());
}
