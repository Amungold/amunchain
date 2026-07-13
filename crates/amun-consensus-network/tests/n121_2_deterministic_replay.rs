// N121.2 — Deterministic Replay of Slashing State
// ================================================
// Verifies that replaying the same sequence of slashing certificates
// produces identical slashing roots across multiple validators.

use amun_consensus_network::{
    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
};

fn make_cert(validator_id: [u8; 32], height: u64, amount: u64) -> SlashingCertificate {
    SlashingCertificate::from_slash_result(
        validator_id,
        30,
        vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
        vec![EvidenceCount {
            evidence_type: EvidenceType::DoubleVote,
            count: 3,
            weight: 30,
        }],
        1500,
        amount,
        100_000 - amount,
        3,
        ValidatorStatus::SlashEligible,
        height,
    )
}

/// N121.2 Gatekeeper: Three nodes replay the same events → same root
#[test]
fn n121_2_three_nodes_same_replay_same_root() {
    let certs = vec![
        make_cert([0x42; 32], 100, 15000),
        make_cert([0x99; 32], 200, 5000),
        make_cert([0xAA; 32], 300, 20000),
    ];

    // Node A
    let mut state_a = SlashingState::new();
    for cert in &certs {
        state_a.execute(cert, || Ok(())).unwrap();
    }

    // Node B (same sequence)
    let mut state_b = SlashingState::new();
    for cert in &certs {
        state_b.execute(cert, || Ok(())).unwrap();
    }

    // Node C (same sequence)
    let mut state_c = SlashingState::new();
    for cert in &certs {
        state_c.execute(cert, || Ok(())).unwrap();
    }

    assert_eq!(
        state_a.root, state_b.root,
        "N121.2 FAIL: Node A and Node B must produce the same slashing root"
    );
    assert_eq!(
        state_b.root, state_c.root,
        "N121.2 FAIL: Node B and Node C must produce the same slashing root"
    );
    assert_eq!(state_a.executed_count(), 3);
    assert_eq!(state_b.executed_count(), 3);
    assert_eq!(state_c.executed_count(), 3);
}

/// N121.2: Different order → different root (order sensitivity preserved)
#[test]
fn n121_2_different_order_different_root() {
    let cert1 = make_cert([0x42; 32], 100, 15000);
    let cert2 = make_cert([0x99; 32], 200, 5000);

    // Node A: cert1 then cert2
    let mut state_a = SlashingState::new();
    state_a.execute(&cert1, || Ok(())).unwrap();
    state_a.execute(&cert2, || Ok(())).unwrap();

    // Node B: cert2 then cert1 (different order)
    let mut state_b = SlashingState::new();
    state_b.execute(&cert2, || Ok(())).unwrap();
    state_b.execute(&cert1, || Ok(())).unwrap();

    // Roots differ because Merkle tree is order-sensitive
    assert_ne!(
        state_a.root, state_b.root,
        "N121.2 FAIL: different order must produce different roots"
    );
}

/// N121.2: Replay from empty always produces zero root
#[test]
fn n121_2_replay_empty_produces_zero_root() {
    let state = SlashingState::new();
    assert_eq!(
        state.root, [0u8; 32],
        "N121.2 FAIL: empty state must have zero root"
    );
    assert!(state.verify_consistency().is_ok());
}

/// N121.2: Single event replay is deterministic
#[test]
fn n121_2_single_event_replay_deterministic() {
    let cert = make_cert([0x42; 32], 100, 15000);

    let mut state1 = SlashingState::new();
    state1.execute(&cert, || Ok(())).unwrap();

    let mut state2 = SlashingState::new();
    state2.execute(&cert, || Ok(())).unwrap();

    assert_eq!(
        state1.root, state2.root,
        "N121.2 FAIL: single event replay must be deterministic"
    );
}

/// N121.2: Consistency verification catches corruption
#[test]
fn n121_2_consistency_verification_catches_corruption() {
    let cert = make_cert([0x42; 32], 100, 15000);
    let mut state = SlashingState::new();
    state.execute(&cert, || Ok(())).unwrap();

    // Corrupt the root
    state.root = [0xFF; 32];

    assert!(
        state.verify_consistency().is_err(),
        "N121.2 FAIL: corrupted root must be detected"
    );
}
