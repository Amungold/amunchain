// N121.5 — Snapshot Commitment
// N121.6 — Historical Audit Layer
// =================================
// Verifies that slashing state can be snapshotted, restored,
// and audited via query interfaces.

use amun_consensus_network::{
    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
};

fn make_cert(vid: [u8; 32], h: u64, amt: u64) -> SlashingCertificate {
    SlashingCertificate::from_slash_result(
        vid,
        30,
        vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
        vec![EvidenceCount {
            evidence_type: EvidenceType::DoubleVote,
            count: 3,
            weight: 30,
        }],
        1500,
        amt,
        100_000 - amt,
        3,
        ValidatorStatus::SlashEligible,
        h,
    )
}

/// N121.5: Snapshot root matches the state root
#[test]
fn n121_5_snapshot_root_matches_state() {
    let mut state = SlashingState::new();
    state
        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
        .unwrap();
    state
        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
        .unwrap();

    // Snapshot the root
    let snapshot_root = state.root;

    // Verify the state is consistent with its own snapshot
    assert!(
        state.verify_consistency().is_ok(),
        "N121.5 FAIL: state must be consistent with snapshot root"
    );
    assert_ne!(
        snapshot_root, [0u8; 32],
        "N121.5 FAIL: non-empty state snapshot must be non-zero"
    );
}

/// N121.5: Restored state from snapshot is identical
#[test]
fn n121_5_restored_state_from_snapshot() {
    // Simulate: build state, snapshot it, rebuild from scratch
    let certs = vec![
        make_cert([0x42; 32], 100, 15000),
        make_cert([0x99; 32], 200, 5000),
        make_cert([0xAA; 32], 300, 20000),
    ];

    // Original state
    let mut original = SlashingState::new();
    for cert in &certs {
        original.execute(cert, || Ok(())).unwrap();
    }
    let snapshot_root = original.root;

    // Rebuild from the same certificates (simulating restore from snapshot)
    let mut restored = SlashingState::new();
    for cert in &certs {
        restored.execute(cert, || Ok(())).unwrap();
    }

    assert_eq!(
        restored.root, snapshot_root,
        "N121.5 FAIL: restored state must match snapshot root"
    );
    assert!(restored.verify_consistency().is_ok());
}

/// N121.6: Audit trail returns correct history for a validator
#[test]
fn n121_6_audit_trail_by_validator() {
    let mut state = SlashingState::new();

    state
        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
        .unwrap();
    state
        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
        .unwrap();
    state
        .execute(&make_cert([0x42; 32], 300, 10000), || Ok(()))
        .unwrap();

    let v42_history: Vec<_> = state
        .history()
        .iter()
        .filter(|s| s.validator_id == [0x42; 32])
        .collect();

    assert_eq!(
        v42_history.len(),
        2,
        "N121.6 FAIL: validator 0x42 must have 2 slashes"
    );
    assert_eq!(v42_history[0].height, 100);
    assert_eq!(v42_history[1].height, 300);

    let v99_history: Vec<_> = state
        .history()
        .iter()
        .filter(|s| s.validator_id == [0x99; 32])
        .collect();

    assert_eq!(
        v99_history.len(),
        1,
        "N121.6 FAIL: validator 0x99 must have 1 slash"
    );
    assert_eq!(v99_history[0].height, 200);
}

/// N121.6: Total slash count is auditable
#[test]
fn n121_6_total_slash_count_auditable() {
    let mut state = SlashingState::new();
    assert_eq!(state.executed_count(), 0);

    state
        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
        .unwrap();
    assert_eq!(state.executed_count(), 1);

    state
        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
        .unwrap();
    assert_eq!(state.executed_count(), 2);
}

/// N121.6: Empty state audit returns empty history
#[test]
fn n121_6_empty_state_empty_history() {
    let state = SlashingState::new();
    assert!(state.history().is_empty());
    assert_eq!(state.executed_count(), 0);
    assert_eq!(state.root, [0u8; 32]);
}
