use amun_constitutional_commitment::{
    AppHashPipeline, ConstitutionalStatus, EconomicSnapshot, EndBlockPipeline, Hash32,
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

#[test]
fn rpc_status_contains_all_roots() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let acc = make_hash(0xAA);
    let stk = make_hash(0xBB);
    let gov_state = make_hash(0xCC);

    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
    let app_hash = AppHashPipeline::state_root_to_app_hash(state_root);

    let status = ConstitutionalStatus::new(7990, &commitment, app_hash);
    let json = status.to_json();

    assert_eq!(status.height, 7990);
    assert_eq!(status.version, 1);
    assert!(json.contains("identity_root"));
    assert!(json.contains("evidence_root"));
    assert!(json.contains("governance_root"));
    assert!(json.contains("economic_root"));
    assert!(json.contains("constitutional_root"));
    assert!(json.contains("app_hash"));
    assert!(json.contains("0x"));
}

#[test]
fn rpc_status_reflects_economic_change() {
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
    let gov_state = make_hash(0xCC);

    let sr_a = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_a);
    let sr_b = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_b);

    let ah_a = AppHashPipeline::state_root_to_app_hash(sr_a);
    let ah_b = AppHashPipeline::state_root_to_app_hash(sr_b);

    let status_a = ConstitutionalStatus::new(7990, &com_a, ah_a);
    let status_b = ConstitutionalStatus::new(7990, &com_b, ah_b);

    assert_ne!(status_a.economic_root, status_b.economic_root);
    assert_ne!(status_a.constitutional_root, status_b.constitutional_root);
    assert_ne!(status_a.app_hash, status_b.app_hash);
}

#[test]
fn rpc_json_is_valid_and_parseable() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let acc = make_hash(0xAA);
    let stk = make_hash(0xBB);
    let gov_state = make_hash(0xCC);

    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
    let app_hash = AppHashPipeline::state_root_to_app_hash(state_root);

    let status = ConstitutionalStatus::new(7990, &commitment, app_hash);
    let json = status.to_json();

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json);
    assert!(parsed.is_ok());

    let obj = parsed.unwrap();
    assert_eq!(obj["height"], 7990);
    assert_eq!(obj["version"], 1);
    assert!(obj["identity_root"].is_string());
    assert!(obj["evidence_root"].is_string());
    assert!(obj["governance_root"].is_string());
    assert!(obj["economic_root"].is_string());
    assert!(obj["constitutional_root"].is_string());
    assert!(obj["app_hash"].is_string());
}
