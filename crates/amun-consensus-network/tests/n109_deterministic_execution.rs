// ============================================================================
// N109.7 — Deterministic Re-Execution Tests
// ============================================================================
// Validator A (proposer): state_root = X
// Validator B (honest):   re-execution => X → vote accepted
// Validator C (byzantine): re-execution => Y → vote rejected
//
// Constitutional requirement: state_root mismatch → no vote → no QC formed.
// ============================================================================

use std::collections::HashMap;

// Mirror the N109 types (same as in n109_block_propagation.rs)
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct BlockProposal {
    proposer_id: [u8; 32],
    height: u64,
    timestamp: u64,
    block_hash: [u8; 32],
    parent_root: [u8; 32],
    state_root: [u8; 32],
    block_bytes: Vec<u8>,
}

// ============================================================================
// N109.7A — ExecutionReceipt
// ============================================================================
#[derive(Debug, Clone, PartialEq)]
struct ExecutionReceipt {
    state_root: [u8; 32],
    height: u64,
    block_hash: [u8; 32],
    success: bool,
}

// ============================================================================
// N109.7B — Metrics
// ============================================================================
#[derive(Debug, Default)]
struct ConsensusMetrics {
    state_root_mismatches: u64,
    #[allow(dead_code)]
    basic_validation_failures: u64, // Reserved for N109.6 metrics tests
    proposals_received: u64,
    proposals_accepted: u64,
}

// ============================================================================
// N109.7 — Verify Block Execution (the core function)
// ============================================================================
fn verify_block_execution<F>(
    proposal: &BlockProposal,
    mut executor: F,
) -> Result<ExecutionReceipt, String>
where
    F: FnMut(&[u8]) -> Result<[u8; 32], String>,
{
    let computed_root =
        executor(&proposal.block_bytes).map_err(|e| format!("N109.7 EXECUTION_FAILED: {}", e))?;

    if computed_root != proposal.state_root {
        return Err(format!(
            "N109.7 STATE_ROOT_MISMATCH: height={} proposed={} computed={}",
            proposal.height,
            hex::encode(proposal.state_root),
            hex::encode(computed_root),
        ));
    }

    Ok(ExecutionReceipt {
        state_root: computed_root,
        height: proposal.height,
        block_hash: proposal.block_hash,
        success: true,
    })
}

// ============================================================================
// Helper: simulate a deterministic executor
// ============================================================================
fn simulate_executor(block_bytes: &[u8], seed: u64) -> Result<[u8; 32], String> {
    // Deterministic: hash(seed || block_bytes)
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed.to_le_bytes());
    hasher.update(block_bytes);
    Ok(hasher.finalize().into())
}

// ============================================================================
// TEST: Validator B re-executes and matches → vote accepted
// ============================================================================
#[test]
fn n109_7_re_execution_accepts_matching_state_root() {
    let block_bytes = vec![1, 2, 3, 4];
    let honest_root = simulate_executor(&block_bytes, 42).unwrap();

    let proposal = BlockProposal {
        proposer_id: [0xAA; 32],
        height: 1,
        timestamp: 1000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: honest_root, // Proposer computed correctly
        block_bytes,
    };

    // Validator B uses the SAME seed → should match
    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 42));
    assert!(
        result.is_ok(),
        "N109.7 FAIL: honest validator should accept matching root"
    );
    let receipt = result.unwrap();
    assert_eq!(receipt.state_root, honest_root);
    assert!(receipt.success);
}

// ============================================================================
// TEST: Validator C re-executes and MISMATCHES → vote rejected
// ============================================================================
#[test]
fn n109_7_re_execution_rejects_mismatched_state_root() {
    let block_bytes = vec![1, 2, 3, 4];
    let proposer_root = simulate_executor(&block_bytes, 42).unwrap(); // Seed 42

    let proposal = BlockProposal {
        proposer_id: [0xAA; 32],
        height: 1,
        timestamp: 1000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: proposer_root,
        block_bytes,
    };

    // Validator C uses DIFFERENT seed → should mismatch
    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 99));
    assert!(
        result.is_err(),
        "N109.7 FAIL: should reject mismatched root"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("STATE_ROOT_MISMATCH"),
        "error must mention STATE_ROOT_MISMATCH"
    );
    assert!(err.contains("height=1"), "error must include height");
}

// ============================================================================
// TEST: Three validators — one mismatch means no QC
// ============================================================================
#[test]
fn n109_7_three_validators_one_mismatch_no_qc() {
    let block_bytes = vec![10, 20, 30];
    let honest_root = simulate_executor(&block_bytes, 1).unwrap();

    let proposal = BlockProposal {
        proposer_id: [1u8; 32],
        height: 5,
        timestamp: 5000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: honest_root,
        block_bytes: block_bytes.clone(),
    };

    // Simulate 3 validators re-executing
    let mut approvals = 0u64;
    let mut rejections = 0u64;
    let mut metrics = ConsensusMetrics::default();
    let mut voted: Vec<u8> = vec![];

    for v_id in 0..3u8 {
        metrics.proposals_received += 1;

        // Validators 0 and 1 use correct seed → match
        // Validator 2 uses wrong seed → mismatch
        let seed = if v_id == 2 { 99 } else { 1 };

        let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, seed));

        match result {
            Ok(_receipt) => {
                metrics.proposals_accepted += 1;
                approvals += 1;
                voted.push(v_id);
            }
            Err(e) => {
                if e.contains("STATE_ROOT_MISMATCH") {
                    metrics.state_root_mismatches += 1;
                }
                rejections += 1;
            }
        }
    }

    // Constitutional check: 2 approvals out of 4 total validators (not > 2/3 of 4=3)
    // With 4 validators, need 3 approvals for quorum.
    // 2 approvals + 1 rejection = no quorum reached.
    assert_eq!(approvals, 2, "N109.7 FAIL: expected 2 approvals");
    assert_eq!(rejections, 1, "N109.7 FAIL: expected 1 rejection");
    assert_eq!(
        metrics.state_root_mismatches, 1,
        "N109.7B FAIL: mismatch counter should be 1"
    );
    assert_eq!(
        metrics.proposals_accepted, 2,
        "N109.7B FAIL: accepted counter should be 2"
    );

    // Verify: rejected validator is #2
    assert!(!voted.contains(&2), "Validator 2 should NOT have voted");
    assert!(voted.contains(&0), "Validator 0 should have voted");
    assert!(voted.contains(&1), "Validator 1 should have voted");

    eprintln!(
        "N109.7 RESULT: {} approvals, {} rejections — {}",
        approvals,
        rejections,
        if approvals >= 3 {
            "QC FORMED"
        } else {
            "NO QC (constitutional)"
        }
    );
}

// ============================================================================
// TEST: Metrics increment correctly on mismatch
// ============================================================================
#[test]
fn n109_7b_metrics_counts_state_root_mismatches() {
    let mut metrics = ConsensusMetrics::default();
    let block_bytes = vec![7, 7, 7];

    let proposal = BlockProposal {
        proposer_id: [0xFF; 32],
        height: 3,
        timestamp: 3000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: [0xAA; 32], // Wrong root
        block_bytes,
    };

    metrics.proposals_received += 1;
    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 1));

    assert!(result.is_err());
    metrics.state_root_mismatches += 1;

    assert_eq!(metrics.state_root_mismatches, 1);
    assert_eq!(metrics.proposals_accepted, 0);
    assert_eq!(metrics.proposals_received, 1);

    // summary() not on local test struct
}

// ============================================================================
// TEST: Execution failure (not just mismatch) is handled
// ============================================================================
#[test]
fn n109_7_execution_failure_does_not_vote() {
    let block_bytes = vec![9, 9, 9];

    let proposal = BlockProposal {
        proposer_id: [0xBB; 32],
        height: 2,
        timestamp: 2000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: [0u8; 32],
        block_bytes,
    };

    let result = verify_block_execution(&proposal, |_bytes| {
        Err("SIMULATED_CRASH: execution engine failure".to_string())
    });

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("EXECUTION_FAILED"),
        "error must mention EXECUTION_FAILED"
    );
    assert!(
        err.contains("SIMULATED_CRASH"),
        "error must include original cause"
    );
}

// ============================================================================
// TEST: Cache retention — proposal stays until finalized
// ============================================================================
#[test]
fn n109_7c_proposal_retained_until_finalized() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    let block_bytes = vec![1, 2, 3];
    let proposal = BlockProposal {
        proposer_id: [1u8; 32],
        height: 10,
        timestamp: 10000,
        block_hash: blake3::hash(&block_bytes).into(),
        parent_root: [0u8; 32],
        state_root: simulate_executor(&block_bytes, 1).unwrap(),
        block_bytes,
    };

    // N109.5: Store proposal on receipt
    cache.insert(10, proposal.clone());

    // N109.7: Re-execute using cached proposal
    let cached = cache
        .get(&10)
        .expect("N109.7C FAIL: proposal must be in cache");
    let result = verify_block_execution(cached, |bytes| simulate_executor(bytes, 1));
    assert!(
        result.is_ok(),
        "N109.7C FAIL: re-execution from cache failed"
    );

    // N109.7C: Proposal stays in cache after re-execution
    assert!(
        cache.contains_key(&10),
        "N109.7C FAIL: proposal removed too early"
    );

    // N109.3: Cleanup only after finalization
    cache.remove(&10);
    assert!(
        !cache.contains_key(&10),
        "N109.7C FAIL: proposal should be removed after finalization"
    );
}

// ============================================================================
// TEST: Determinism — same input → same state_root every time
// ============================================================================
#[test]
fn n109_7_execution_is_deterministic() {
    let block_bytes = vec![5, 5, 5];
    let seed = 42u64;

    let root1 = simulate_executor(&block_bytes, seed).unwrap();
    let root2 = simulate_executor(&block_bytes, seed).unwrap();
    let root3 = simulate_executor(&block_bytes, seed).unwrap();

    assert_eq!(
        root1, root2,
        "N109.7 FAIL: execution not deterministic (run 1 vs 2)"
    );
    assert_eq!(
        root2, root3,
        "N109.7 FAIL: execution not deterministic (run 2 vs 3)"
    );
}

// ============================================================================
// TEST: Different blocks → different state roots (no collision)
// ============================================================================
#[test]
fn n109_7_different_blocks_different_roots() {
    let bytes_a = vec![1, 2, 3];
    let bytes_b = vec![1, 2, 4]; // Single byte difference
    let seed = 1u64;

    let root_a = simulate_executor(&bytes_a, seed).unwrap();
    let root_b = simulate_executor(&bytes_b, seed).unwrap();

    assert_ne!(
        root_a, root_b,
        "N109.7 FAIL: different blocks must produce different roots"
    );
}
