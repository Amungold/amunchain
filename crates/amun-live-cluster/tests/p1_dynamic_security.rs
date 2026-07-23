//! P1.2: Dynamic Security Tests
//!
//! Tests system behavior under adversarial inputs.
//! All cases must be rejected safely — no panics, no state corruption.

use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;

// ============================================================================
// Malformed Vote Tests
// ============================================================================

#[test]
fn p1_reject_zero_height_vote() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);

    let vote = ConsensusVote {
        voter_id: [1u8; 32],
        height: 0, // Invalid: zero height
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64],
        timestamp: 1000,
        commitment: None,
    };
    let result = engine.process_vote(vote);
    assert!(result.is_err(), "Zero-height vote must be rejected");
}

#[test]
fn p1_reject_future_vote_beyond_window() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);

    let vote = ConsensusVote {
        voter_id: [1u8; 32],
        height: 1000, // Far future
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64],
        timestamp: 1000,
        commitment: None,
    };
    let result = engine.process_vote(vote);
    assert!(result.is_err(), "Far-future vote must be rejected");
    assert!(engine.needs_catchup.load(std::sync::atomic::Ordering::SeqCst),
        "Far-future vote must trigger catchup flag");
}

#[test]
fn p1_reject_duplicate_voter() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);

    let vote = ConsensusVote {
        voter_id: [1u8; 32],
        height: 1,
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64],
        timestamp: 1000,
        commitment: None,
    };
    assert!(engine.process_vote(vote.clone()).is_ok());
    assert!(engine.process_vote(vote).is_err(), "Duplicate voter must be rejected");
}

#[test]
fn p1_reject_equivocation() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);

    let vote_a = ConsensusVote {
        voter_id: [1u8; 32],
        height: 1,
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64],
        timestamp: 1000,
        commitment: None,
    };
    let vote_b = ConsensusVote {
        voter_id: [1u8; 32],
        height: 1,
        block_hash: [0xFF; 32], // Different block
        state_root: [0xBB; 32],
        approve: true,
        signature: [1u8; 64],
        timestamp: 1000,
        commitment: None,
    };
    assert!(engine.process_vote(vote_a).is_ok());
    assert!(engine.process_vote(vote_b).is_err(), "Equivocation must be rejected");
}

// ============================================================================
// Invalid Signature Tests
// ============================================================================

#[test]
fn p1_reject_all_zero_signature() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    // Register a validator so signature checking is active
    engine.register_validator([1u8; 32], [1u8; 32]);
    engine.start_round(1, [1u8; 32]);

    let vote = ConsensusVote {
        voter_id: [1u8; 32],
        height: 1,
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64], // All zeros
        timestamp: 1000,
        commitment: None,
    };
    let result = engine.process_vote(vote);
    assert!(result.is_err(), "Zero signature must be rejected");
}

// ============================================================================
// Oversized Message Tests
// ============================================================================

#[test]
fn p1_canonical_writer_rejects_oversized_allocation() {
    use amun_canonical_codec::CanonicalWriter;
    let mut w = CanonicalWriter::new();
    // Write a small payload first
    w.write_u32(42);
    // Attempting to write bytes exceeding MAX_CANONICAL_ALLOCATION should panic
    let huge = vec![0u8; 65 * 1024 * 1024]; // 65MB > 64MB limit
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        w.write_bytes(&huge);
    }));
    assert!(result.is_err(), "Oversized allocation must be rejected");
}

// ============================================================================
// Quorum Edge Cases
// ============================================================================

#[test]
fn p1_no_qc_without_quorum() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);
    engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

    // Only 1 vote out of 4 (< 2/3)
    engine.process_vote(ConsensusVote {
        voter_id: [1u8; 32], height: 1,
        block_hash: [0xAA; 32], state_root: [0xBB; 32],
        approve: true, signature: [0u8; 64], timestamp: 1000,
        commitment: None,
    }).unwrap();

    let active = engine.active_validator_count();
    let qc = engine.round_mut(1).unwrap().try_form_qc(
        active,
        &std::collections::HashMap::new(),
        0,
    );
    assert!(qc.is_none(), "QC must not form without 2/3 quorum");
}

#[test]
fn p1_qc_forms_with_supermajority() {
    let mut engine = ConsensusEngine::new([0u8; 32], 4);
    engine.start_round(1, [1u8; 32]);
    engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

    for id in 1..=3u8 {
        engine.process_vote(ConsensusVote {
            voter_id: [id; 32], height: 1,
            block_hash: [0xAA; 32], state_root: [0xBB; 32],
            approve: true, signature: [0u8; 64], timestamp: 1000,
            commitment: None,
        }).unwrap();
    }

    let active = engine.active_validator_count();
    let qc = engine.round_mut(1).unwrap().try_form_qc(
        active,
        &std::collections::HashMap::new(),
        0,
    );
    assert!(qc.is_some(), "QC must form with 3/4 > 2/3 quorum");
}
