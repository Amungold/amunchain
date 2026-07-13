use amun_constitutional_commitment::{AppHashPipeline, EconomicSnapshot, EndBlockPipeline, Hash32};

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

#[test]
fn endblock_pipeline_produces_commitment() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap);
    assert!(commitment.is_some());
    let c = commitment.unwrap();
    assert_eq!(c.version, 1);
    assert_eq!(c.identity_root, id);
    assert_eq!(c.evidence_root, ev);
    assert_eq!(c.governance_root, gv);
}

#[test]
fn treasury_change_changes_apphash() {
    let snap_a = sample_snapshot();
    let mut snap_b = sample_snapshot();
    snap_b.treasury_balance += 1;
    snap_b.circulating_supply -= 1;

    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let com_a = EndBlockPipeline::execute(id, ev, gv, &snap_a).unwrap();
    let com_b = EndBlockPipeline::execute(id, ev, gv, &snap_b).unwrap();

    let acc = make_hash(0xAA);
    let stk = make_hash(0xBB);
    let gov = make_hash(0xCC);

    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);

    let app_hash_a = AppHashPipeline::state_root_to_app_hash(state_root_a);
    let app_hash_b = AppHashPipeline::state_root_to_app_hash(state_root_b);

    assert_ne!(com_a.economic_root, com_b.economic_root);
    assert_ne!(com_a.constitutional_root, com_b.constitutional_root);
    assert_ne!(state_root_a, state_root_b);
    assert_ne!(app_hash_a, app_hash_b);
}

#[test]
fn evidence_change_changes_apphash() {
    let snap = sample_snapshot();

    let id = make_hash(0x11);
    let ev_a = make_hash(0x22);
    let ev_b = make_hash(0xFF);
    let gv = make_hash(0x33);

    let com_a = EndBlockPipeline::execute(id, ev_a, gv, &snap).unwrap();
    let com_b = EndBlockPipeline::execute(id, ev_b, gv, &snap).unwrap();

    let acc = make_hash(0xAA);
    let stk = make_hash(0xBB);
    let gov = make_hash(0xCC);

    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);

    let app_hash_a = AppHashPipeline::state_root_to_app_hash(state_root_a);
    let app_hash_b = AppHashPipeline::state_root_to_app_hash(state_root_b);

    assert_ne!(com_a.constitutional_root, com_b.constitutional_root);
    assert_ne!(state_root_a, state_root_b);
    assert_ne!(app_hash_a, app_hash_b);
}

#[test]
fn same_state_produces_same_apphash() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let com_a = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let com_b = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();

    let acc = make_hash(0xAA);
    let stk = make_hash(0xBB);
    let gov = make_hash(0xCC);

    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);

    let app_hash_a = AppHashPipeline::state_root_to_app_hash(state_root_a);
    let app_hash_b = AppHashPipeline::state_root_to_app_hash(state_root_b);

    assert_eq!(com_a.constitutional_root, com_b.constitutional_root);
    assert_eq!(state_root_a, state_root_b);
    assert_eq!(app_hash_a, app_hash_b);
}

#[test]
fn invalid_snapshot_returns_none_from_endblock() {
    let mut snap = sample_snapshot();
    snap.circulating_supply += 100;
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let result = EndBlockPipeline::execute(id, ev, gv, &snap);
    assert!(result.is_none());
}
