use amun_constitutional_commitment::{
    roots::commitment_root, AppHashPipeline, EconomicSnapshot, EndBlockPipeline, Hash32, Verifier,
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
fn verifier_confirms_valid_roots() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let stored_commitment = commitment_root(&commitment);

    let result = Verifier::verify(
        id,
        ev,
        gv,
        commitment.economic_root,
        commitment.constitutional_root,
        stored_commitment,
    );

    assert!(result.constitutional_root_match);
    assert!(result.commitment_root_match);
    assert!(Verifier::verified(&result));
}

#[test]
fn verifier_rejects_tampered_constitutional_root() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let stored_commitment = commitment_root(&commitment);

    let mut tampered = commitment.constitutional_root;
    tampered[0] ^= 1;

    let result = Verifier::verify(
        id,
        ev,
        gv,
        commitment.economic_root,
        tampered,
        stored_commitment,
    );

    assert!(!result.constitutional_root_match);
    assert!(!Verifier::verified(&result));
}

#[test]
fn verifier_rejects_tampered_economic_root() {
    let snap = sample_snapshot();
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);

    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
    let stored_commitment = commitment_root(&commitment);

    let mut tampered_economic = commitment.economic_root;
    tampered_economic[0] ^= 1;

    let result = Verifier::verify(
        id,
        ev,
        gv,
        tampered_economic,
        commitment.constitutional_root,
        stored_commitment,
    );

    assert!(!result.constitutional_root_match);
    assert!(!Verifier::verified(&result));
}

#[test]
fn explorer_verification_flow_end_to_end() {
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
    let stored_commitment = commitment_root(&commitment);

    let result = Verifier::verify(
        commitment.identity_root,
        commitment.evidence_root,
        commitment.governance_root,
        commitment.economic_root,
        commitment.constitutional_root,
        stored_commitment,
    );

    assert!(Verifier::verified(&result));
    assert_eq!(
        result.recomputed_constitutional_root,
        commitment.constitutional_root
    );
    assert_eq!(result.recomputed_commitment_root, stored_commitment);

    let status =
        amun_constitutional_commitment::ConstitutionalStatus::new(7990, &commitment, app_hash);
    let json = status.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed["identity_root"],
        "0x1111111111111111111111111111111111111111111111111111111111111111"
    );
    assert_eq!(
        parsed["evidence_root"],
        "0x2222222222222222222222222222222222222222222222222222222222222222"
    );
    assert!(parsed["app_hash"].is_string());
}
