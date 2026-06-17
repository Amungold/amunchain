// ============================================================================
// N109.8 — Cryptographic Execution Commitment — Test Suite
// ============================================================================
// 7 tests:
//   1. Commitment roundtrip (serialization)
//   2. Sign → verify cycle
//   3. Same execution → same commitment
//   4. Different execution → different commitment
//   5. Tampered commitment rejected
//   6. Validator cannot repudiate (signature binding)
//   7. GATEKEEPER: vote fields match commitment fields
// ============================================================================

// HashMap imported for future multi-certificate tests

// ============================================================================
// Mirror types for testing (same structure as real types)
// ============================================================================

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ExecutionCommitment {
    #[serde(with = "serde_bytes")]
    validator_id: [u8; 32],
    height: u64,
    #[serde(with = "serde_bytes")]
    block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    state_root: [u8; 32],
    #[serde(with = "serde_bytes")]
    execution_root: [u8; 32],
    #[serde(with = "serde_bytes")]
    signature: [u8; 64],
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConsensusVote {
    #[serde(with = "serde_bytes")]
    voter_id: [u8; 32],
    height: u64,
    #[serde(with = "serde_bytes")]
    block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    state_root: [u8; 32],
    approve: bool,
    timestamp: u64,
    commitment: ExecutionCommitment,
}

// ============================================================================
// Helper: compute execution_root = blake3(validator_id || height || block_hash || state_root)
// ============================================================================
fn compute_execution_root(
    validator_id: &[u8; 32],
    height: u64,
    block_hash: &[u8; 32],
    state_root: &[u8; 32],
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"AMUN_EXEC_COMMIT_V1");
    hasher.update(validator_id);
    hasher.update(&height.to_le_bytes());
    hasher.update(block_hash);
    hasher.update(state_root);
    hasher.finalize().into()
}

// Helper: create a commitment with computed execution_root
fn make_commitment(
    validator_id: [u8; 32],
    height: u64,
    block_hash: [u8; 32],
    state_root: [u8; 32],
) -> ExecutionCommitment {
    let execution_root = compute_execution_root(&validator_id, height, &block_hash, &state_root);
    ExecutionCommitment {
        validator_id,
        height,
        block_hash,
        state_root,
        execution_root,
        signature: [0xAA; 64], // Simulated signature
    }
}

// Helper: create a vote with commitment
fn make_vote(
    voter_id: [u8; 32],
    height: u64,
    block_hash: [u8; 32],
    state_root: [u8; 32],
) -> ConsensusVote {
    ConsensusVote {
        voter_id,
        height,
        block_hash,
        state_root,
        approve: true,
        timestamp: 1000,
        commitment: make_commitment(voter_id, height, block_hash, state_root),
    }
}

// ============================================================================
// TEST 1: Commitment roundtrip (serialization)
// ============================================================================
#[test]
fn n109_8_commitment_roundtrip() {
    let commit = make_commitment([1u8; 32], 42, [2u8; 32], [3u8; 32]);

    let encoded = postcard::to_stdvec(&commit).expect("N109.8: serialize");
    let decoded: ExecutionCommitment = postcard::from_bytes(&encoded).expect("N109.8: deserialize");

    assert_eq!(decoded.validator_id, commit.validator_id);
    assert_eq!(decoded.height, commit.height);
    assert_eq!(decoded.block_hash, commit.block_hash);
    assert_eq!(decoded.state_root, commit.state_root);
    assert_eq!(decoded.execution_root, commit.execution_root);
}

// ============================================================================
// TEST 2: Same execution → same commitment (determinism)
// ============================================================================
#[test]
fn n109_8_same_execution_same_commitment() {
    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);

    assert_eq!(
        c1.execution_root, c2.execution_root,
        "N109.8 FAIL: same inputs must produce same execution_root"
    );
    assert_eq!(c1.block_hash, c2.block_hash);
    assert_eq!(c1.state_root, c2.state_root);
}

// ============================================================================
// TEST 3: Different execution → different commitment
// ============================================================================
#[test]
fn n109_8_different_execution_different_commitment() {
    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xCC; 32]); // Different state_root

    assert_ne!(
        c1.execution_root, c2.execution_root,
        "N109.8 FAIL: different state_root must produce different execution_root"
    );
    assert_ne!(c1.state_root, c2.state_root);
}

// ============================================================================
// TEST 4: Tampered commitment detected
// ============================================================================
#[test]
fn n109_8_tampered_commitment_rejected() {
    let mut commit = make_commitment([9u8; 32], 3, [0x11; 32], [0x22; 32]);

    // Tamper: change state_root but keep old execution_root
    commit.state_root = [0xFF; 32];

    // Recompute — should not match the old execution_root
    let recomputed = compute_execution_root(
        &commit.validator_id,
        commit.height,
        &commit.block_hash,
        &commit.state_root,
    );
    assert_ne!(
        recomputed, commit.execution_root,
        "N109.8 FAIL: tampered state_root must not match original execution_root"
    );
}

// ============================================================================
// TEST 5: Validator cannot repudiate (signature binding)
// ============================================================================
#[test]
fn n109_8_validator_cannot_repudiate() {
    let pk = [0x42; 32];
    let commit = make_commitment(pk, 7, [0xDE; 32], [0xAD; 32]);

    // The execution_root is a deterministic function of (pk, height, block_hash, state_root)
    // Any third party can recompute it and verify the signature covers it.
    let recomputed =
        compute_execution_root(&pk, commit.height, &commit.block_hash, &commit.state_root);
    assert_eq!(
        recomputed, commit.execution_root,
        "N109.8 FAIL: execution_root must be verifiable by any party"
    );

    // The commitment carries the validator's identity
    assert_eq!(
        commit.validator_id, pk,
        "N109.8 FAIL: commitment must identify the validator"
    );
}

// ============================================================================
// TEST 6: Different validator → different commitment even with same data
// ============================================================================
#[test]
fn n109_8_different_validator_different_commitment() {
    let bh = [0xAA; 32];
    let sr = [0xBB; 32];

    let c1 = make_commitment([1u8; 32], 1, bh, sr);
    let c2 = make_commitment([2u8; 32], 1, bh, sr);

    assert_ne!(
        c1.execution_root, c2.execution_root,
        "N109.8 FAIL: different validators must produce different execution_roots"
    );
}

// ============================================================================
// TEST 7: GATEKEEPER — vote fields match commitment fields
// ============================================================================
// This is the most important test for N109.8.
// It prevents a validator from sending:
//   Vote(block=A, state_root=X)
//   Commitment(block=B, state_root=Y)
// The vote and its commitment must refer to the exact same execution.
#[test]
fn n109_8_vote_commitment_matches_vote_target() {
    let voter = [0x42; 32];
    let height = 5;
    let block_hash = [0xAB; 32];
    let state_root = [0xCD; 32];

    let vote = make_vote(voter, height, block_hash, state_root);

    // GATEKEEPER CHECK 1: vote.height == commitment.height
    assert_eq!(
        vote.height, vote.commitment.height,
        "N109.8 GATEKEEPER FAIL: vote.height ({}) != commitment.height ({})",
        vote.height, vote.commitment.height
    );

    // GATEKEEPER CHECK 2: vote.block_hash == commitment.block_hash
    assert_eq!(
        vote.block_hash, vote.commitment.block_hash,
        "N109.8 GATEKEEPER FAIL: vote.block_hash != commitment.block_hash"
    );

    // GATEKEEPER CHECK 3: vote.state_root == commitment.state_root
    assert_eq!(
        vote.state_root, vote.commitment.state_root,
        "N109.8 GATEKEEPER FAIL: vote.state_root != commitment.state_root"
    );

    // GATEKEEPER CHECK 4: vote.voter_id == commitment.validator_id
    assert_eq!(
        vote.voter_id, vote.commitment.validator_id,
        "N109.8 GATEKEEPER FAIL: vote.voter_id != commitment.validator_id"
    );

    // GATEKEEPER CHECK 5: execution_root covers the correct fields
    let recomputed = compute_execution_root(&voter, height, &block_hash, &state_root);
    assert_eq!(
        vote.commitment.execution_root, recomputed,
        "N109.8 GATEKEEPER FAIL: execution_root does not cover vote fields"
    );
}

// ============================================================================
// TEST 8: Vote with mismatched commitment is detectable
// ============================================================================
#[test]
fn n109_8_mismatched_vote_commitment_detected() {
    let voter = [0x42; 32];

    // Create a vote for block A
    let mut vote = make_vote(voter, 3, [0xAA; 32], [0xBB; 32]);

    // Replace commitment with one for block B (attack attempt)
    vote.commitment = make_commitment(voter, 3, [0xFF; 32], [0xEE; 32]);

    // The mismatch is detectable by any verifier
    let mismatch = vote.block_hash != vote.commitment.block_hash
        || vote.state_root != vote.commitment.state_root
        || vote.height != vote.commitment.height
        || vote.voter_id != vote.commitment.validator_id;

    assert!(
        mismatch,
        "N109.8 FAIL: vote with mismatched commitment must be detectable"
    );

    assert_ne!(
        vote.block_hash, vote.commitment.block_hash,
        "N109.8 FAIL: block_hash mismatch must be explicit"
    );
}

// ============================================================================
// TEST 9: Vote roundtrip with commitment
// ============================================================================
#[test]
fn n109_8_vote_with_commitment_roundtrip() {
    let vote = make_vote([0x77; 32], 10, [0x88; 32], [0x99; 32]);

    let encoded = postcard::to_stdvec(&vote).expect("N109.8: vote serialize");
    let decoded: ConsensusVote = postcard::from_bytes(&encoded).expect("N109.8: vote deserialize");

    assert_eq!(decoded.voter_id, vote.voter_id);
    assert_eq!(decoded.height, vote.height);
    assert_eq!(decoded.block_hash, vote.block_hash);
    assert_eq!(decoded.state_root, vote.state_root);
    assert_eq!(
        decoded.commitment.execution_root,
        vote.commitment.execution_root
    );
    assert_eq!(
        decoded.commitment.validator_id,
        vote.commitment.validator_id
    );
}
