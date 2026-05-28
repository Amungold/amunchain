use amun_execution_receipt::{ExecutionReceipt, ExecutionStatus, ExecutionTranscript};
use amun_state_root::snapshot::ReplayEquivalenceProof;
use amun_state_root::verifier::SealCommitment;
use amun_state_root::{ConstitutionalSnapshot, ContinuityChain, SnapshotSeal};

fn make_snapshot(epoch: u64, parent_hash: [u8; 32]) -> ConstitutionalSnapshot {
    ConstitutionalSnapshot {
        epoch,
        height: epoch * 100,
        state_root: [0xAA; 32],
        validator_root: [0xBB; 32],
        execution_root: [0xCC; 32],
        previous_snapshot_hash: parent_hash,
        execution_transcript: ExecutionTranscript {
            receipts: vec![ExecutionReceipt {
                tx_hash: [0x01; 32],
                pre_state_root: [0x00; 32],
                post_state_root: [0x11; 32],
                execution_result_hash: [0xDD; 32],
                status: ExecutionStatus::Success,
                state_changed: true,
                previous_receipt_hash: [0x00; 32],
            }],
        },
        replay_equivalence_proof: ReplayEquivalenceProof {
            identical: true,
            live_root: [0xEE; 32],
            replayed_root: [0xEE; 32],
        },
        timestamp_slot: epoch * 1000,
    }
}

#[test]
fn test_snapshot_continuity_chain() {
    let genesis = make_snapshot(0, [0x00; 32]);
    let gen_hash = genesis.seal_hash();

    let snap1 = make_snapshot(1, gen_hash);
    assert!(ContinuityChain::verify_link(&gen_hash, &snap1));

    let snap1_hash = snap1.seal_hash();
    let snap2 = make_snapshot(2, snap1_hash);
    assert!(ContinuityChain::verify_link(&snap1_hash, &snap2));
}

#[test]
fn test_snapshot_continuity_rejects_broken_chain() {
    let genesis = make_snapshot(0, [0x00; 32]);
    let gen_hash = genesis.seal_hash();

    // Tamper: point to wrong parent
    let snap1 = make_snapshot(1, [0xFF; 32]); // wrong parent
    assert!(!ContinuityChain::verify_link(&gen_hash, &snap1));
}

#[test]
fn test_snapshot_seal_hash_deterministic() {
    let snap1 = make_snapshot(1, [0x00; 32]);
    let snap2 = make_snapshot(1, [0x00; 32]);
    assert_eq!(
        snap1.seal_hash(),
        snap2.seal_hash(),
        "Seal hash must be deterministic"
    );
}

#[test]
fn test_snapshot_seal_hash_differs_with_different_data() {
    let snap1 = make_snapshot(1, [0x00; 32]);
    let snap2 = make_snapshot(2, [0x00; 32]);
    assert_ne!(
        snap1.seal_hash(),
        snap2.seal_hash(),
        "Different snapshots must have different seals"
    );
}

#[test]
fn test_snapshot_with_transcript_produces_valid_seal() {
    let snapshot = make_snapshot(0, [0x00; 32]);
    let seal_hash = snapshot.seal_hash();
    // Seal hash must be non-zero
    assert_ne!(seal_hash, [0u8; 32]);
    // Quorum seal can be created
    let _seal = SnapshotSeal {
        snapshot_hash: seal_hash,
        quorum_commitment: SealCommitment {
            commitment: seal_hash,
        },
    };
}

#[test]
fn test_replay_proof_verify_only_roots_matter() {
    // identical=false but roots equal → still valid
    let proof = ReplayEquivalenceProof {
        identical: false,
        live_root: [0xAA; 32],
        replayed_root: [0xAA; 32],
    };
    assert!(
        proof.verify(),
        "Roots equal → proof valid regardless of flag"
    );

    // identical=true but roots differ → invalid
    let proof = ReplayEquivalenceProof {
        identical: true,
        live_root: [0xAA; 32],
        replayed_root: [0xBB; 32],
    };
    assert!(
        !proof.verify(),
        "Roots differ → proof invalid regardless of flag"
    );
}
