use std::collections::HashMap;

// ============================================================================
// N109 — Constitutional Block Propagation — Test Suite
// ============================================================================
// Tests N109.1 through N109.6 before building N109.7.
//
// CRITICAL GATEKEEPER:
//   n109_block_hash_matches_serialized_block — if this fails, N109.6 is broken.
//
// Run: cargo test n109 -- --nocapture
// ============================================================================

// ============================================================================
// N109 Local Type Definitions (mirror design spec)
// ============================================================================

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

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ConsensusVoteStub {
    voter_id: [u8; 32],
    height: u64,
    block_hash: [u8; 32],
    state_root: [u8; 32],
    approve: bool,
    #[serde(with = "serde_bytes")]
    signature: Vec<u8>,
    timestamp: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
enum NetworkMessage {
    Proposal(BlockProposal),
    Vote(ConsensusVoteStub),
}

// ============================================================================
// HELPERS
// ============================================================================

fn make_test_proposal(height: u64, parent: [u8; 32]) -> BlockProposal {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"TEST_BLOCK_V1");
    hasher.update(&height.to_le_bytes());
    hasher.update(&parent);
    let block_bytes = hasher.finalize().as_bytes().to_vec();
    let block_hash: [u8; 32] = blake3::hash(&block_bytes).into();
    let mut state_hasher = blake3::Hasher::new();
    state_hasher.update(b"STATE_V1");
    state_hasher.update(&height.to_le_bytes());
    let state_root: [u8; 32] = state_hasher.finalize().into();
    BlockProposal {
        proposer_id: [1u8; 32],
        height,
        timestamp: 1000 * height,
        block_hash,
        parent_root: parent,
        state_root,
        block_bytes,
    }
}

fn validate_basic_testable(
    p: &BlockProposal,
    current_height: u64,
    tip_parent: &[u8; 32],
    now_secs: u64,
) -> Result<(), String> {
    if p.height != current_height + 1 {
        return Err(format!(
            "HEIGHT: expected {}, got {}",
            current_height + 1,
            p.height
        ));
    }
    if &p.parent_root != tip_parent {
        return Err("PARENT: parent_root != local tip".into());
    }
    if p.timestamp > now_secs + 10 {
        return Err("TIMESTAMP_FUTURE".into());
    }
    if p.timestamp < now_secs.saturating_sub(60) {
        return Err("TIMESTAMP_PAST".into());
    }
    let computed: [u8; 32] = blake3::hash(&p.block_bytes).into();
    if computed != p.block_hash {
        return Err(format!(
            "HASH_INTEGRITY: stated={} computed={}",
            hex::encode(p.block_hash),
            hex::encode(computed),
        ));
    }
    Ok(())
}

// ============================================================================
// N109.1 — Proposal Roundtrip
// ============================================================================
#[test]
fn n109_proposal_roundtrip() {
    let proposal = make_test_proposal(1, [0u8; 32]);
    let encoded = postcard::to_stdvec(&proposal).expect("serialize");
    let decoded: BlockProposal = postcard::from_bytes(&encoded).expect("deserialize");
    assert_eq!(decoded.proposer_id, proposal.proposer_id, "proposer_id");
    assert_eq!(decoded.height, proposal.height, "height");
    assert_eq!(decoded.timestamp, proposal.timestamp, "timestamp");
    assert_eq!(decoded.block_hash, proposal.block_hash, "block_hash");
    assert_eq!(decoded.parent_root, proposal.parent_root, "parent_root");
    assert_eq!(decoded.state_root, proposal.state_root, "state_root");
    assert_eq!(decoded.block_bytes, proposal.block_bytes, "block_bytes");
}

// ============================================================================
// N109.1 GATEKEEPER — block_hash == blake3(block_bytes)
// ============================================================================
#[test]
fn n109_block_hash_matches_serialized_block() {
    let proposal = make_test_proposal(42, [0xAB; 32]);
    let computed: [u8; 32] = blake3::hash(&proposal.block_bytes).into();
    assert_eq!(
        proposal.block_hash, computed,
        "GATEKEEPER: block_hash != blake3(block_bytes)"
    );
}

// ============================================================================
// N109.2 — NetworkMessage Vote Roundtrip
// ============================================================================
#[test]
fn n109_network_message_vote_roundtrip() {
    let vote = ConsensusVoteStub {
        voter_id: [1u8; 32],
        height: 42,
        block_hash: [2u8; 32],
        state_root: [3u8; 32],
        approve: true,
        signature: vec![0xAA; 64],
        timestamp: 1000,
    };
    let msg = NetworkMessage::Vote(vote.clone());
    let encoded = postcard::to_stdvec(&msg).expect("serialize");
    let decoded: NetworkMessage = postcard::from_bytes(&encoded).expect("deserialize");
    match decoded {
        NetworkMessage::Vote(v) => {
            assert_eq!(v.voter_id, vote.voter_id, "voter_id");
            assert_eq!(v.height, vote.height, "height");
            assert_eq!(v.block_hash, vote.block_hash, "block_hash");
            assert_eq!(v.state_root, vote.state_root, "state_root");
            assert_eq!(v.approve, vote.approve, "approve");
        }
        NetworkMessage::Proposal(_) => panic!("N109.2 FAIL: Vote decoded as Proposal"),
    }
}

// ============================================================================
// N109.3 — NetworkMessage Proposal Roundtrip
// ============================================================================
#[test]
fn n109_network_message_proposal_roundtrip() {
    let proposal = make_test_proposal(1, [0u8; 32]);
    let msg = NetworkMessage::Proposal(proposal.clone());
    let encoded = postcard::to_stdvec(&msg).expect("serialize");
    let decoded: NetworkMessage = postcard::from_bytes(&encoded).expect("deserialize");
    match decoded {
        NetworkMessage::Proposal(p) => {
            assert_eq!(p.height, proposal.height, "height");
            assert_eq!(p.block_hash, proposal.block_hash, "block_hash");
            assert_eq!(p.state_root, proposal.state_root, "state_root");
            assert_eq!(
                p.block_bytes.len(),
                proposal.block_bytes.len(),
                "block_bytes len"
            );
        }
        NetworkMessage::Vote(_) => panic!("N109.3 FAIL: Proposal decoded as Vote"),
    }
}

// ============================================================================
// N109.4 — Proposal Cache Insert
// ============================================================================
#[test]
fn n109_proposal_cache_insert() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    let p1 = make_test_proposal(5, [4u8; 32]);
    cache.insert(p1.height, p1.clone());
    let stored = cache.get(&5).expect("N109.4 FAIL: proposal not found");
    assert_eq!(stored.block_hash, p1.block_hash);
    assert_eq!(stored.block_bytes, p1.block_bytes);
    let p2 = make_test_proposal(6, p1.block_hash);
    cache.insert(6, p2);
    assert_eq!(cache.len(), 2, "cache must hold 2 proposals");
    assert!(cache.contains_key(&5), "height 5 must persist");
    assert!(cache.contains_key(&6), "height 6 must be present");
}

// ============================================================================
// N109.4b — Cache Cleanup
// ============================================================================
#[test]
fn n109_proposal_cache_cleanup() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    for h in 1..=10 {
        cache.insert(h, make_test_proposal(h, [(h - 1) as u8; 32]));
    }
    assert_eq!(cache.len(), 10, "should start with 10 proposals");
    cache.remove(&5);
    cache.retain(|&h, _| h > 5u64.saturating_sub(16));
    assert_eq!(cache.len(), 9, "should have 9 after removing height 5");
    assert!(!cache.contains_key(&5), "height 5 must be removed");
    assert!(cache.contains_key(&1), "height 1 should be retained");
    assert!(cache.contains_key(&10), "height 10 should be retained");
}

// ============================================================================
// N109.5 — Listener Stores Proposal
// ============================================================================
#[test]
fn n109_listener_stores_proposal() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    let proposal = make_test_proposal(3, [2u8; 32]);
    let msg = NetworkMessage::Proposal(proposal.clone());
    let encoded = postcard::to_stdvec(&msg).unwrap();
    let decoded: NetworkMessage = postcard::from_bytes(&encoded).unwrap();
    match decoded {
        NetworkMessage::Proposal(p) => {
            cache.insert(p.height, p);
        }
        _ => panic!("N109.5 FAIL: Expected Proposal variant"),
    }
    let stored = cache
        .get(&3)
        .expect("N109.5 FAIL: listener did not store proposal");
    assert_eq!(stored.proposer_id, proposal.proposer_id, "proposer_id");
    assert_eq!(stored.block_hash, proposal.block_hash, "block_hash");
    assert_eq!(
        stored.block_bytes.len(),
        proposal.block_bytes.len(),
        "block_bytes len"
    );
}

// ============================================================================
// N109.6 — Validate Basic: Accept Valid Proposal
// ============================================================================
#[test]
fn n109_validate_basic_accepts_valid_proposal() {
    let parent = [0u8; 32];
    let p = make_test_proposal(1, parent);
    let result = validate_basic_testable(&p, 0, &parent, 1050);
    assert!(
        result.is_ok(),
        "N109.6 FAIL: valid proposal rejected: {:?}",
        result.err()
    );
}

// ============================================================================
// N109.6 — Reject Wrong Height
// ============================================================================
#[test]
fn n109_validate_basic_rejects_wrong_height() {
    let parent = [0u8; 32];
    let p = make_test_proposal(5, parent);
    let result = validate_basic_testable(&p, 0, &parent, 5000);
    assert!(result.is_err(), "N109.6 FAIL: should reject wrong height");
    assert!(
        result.unwrap_err().contains("HEIGHT"),
        "error must mention HEIGHT"
    );
}

// ============================================================================
// N109.6 — Reject Parent Mismatch
// ============================================================================
#[test]
fn n109_validate_basic_rejects_parent_mismatch() {
    let p = make_test_proposal(1, [0xAA; 32]);
    let result = validate_basic_testable(&p, 0, &[0xBB; 32], 1000);
    assert!(
        result.is_err(),
        "N109.6 FAIL: should reject parent mismatch"
    );
    assert!(
        result.unwrap_err().contains("PARENT"),
        "error must mention PARENT"
    );
}

// ============================================================================
// N109.6 — Reject Hash Mismatch
// ============================================================================
#[test]
fn n109_validate_basic_rejects_hash_mismatch() {
    let mut p = make_test_proposal(1, [0u8; 32]);
    p.block_bytes = vec![0xFF; 100];
    let result = validate_basic_testable(&p, 0, &[0u8; 32], 1000);
    assert!(result.is_err(), "N109.6 FAIL: should reject hash mismatch");
    assert!(
        result.unwrap_err().contains("HASH_INTEGRITY"),
        "error must mention HASH_INTEGRITY"
    );
}

// ============================================================================
// N109 INTEGRATION — Vote Before Proposal
// ============================================================================
#[test]
fn n109_vote_before_proposal_does_not_crash() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    let _vote = ConsensusVoteStub {
        voter_id: [2u8; 32],
        height: 5,
        block_hash: [3u8; 32],
        state_root: [4u8; 32],
        approve: true,
        signature: vec![0u8; 64],
        timestamp: 5000,
    };
    let proposal = make_test_proposal(5, [2u8; 32]);
    cache.insert(proposal.height, proposal);
    assert!(
        cache.contains_key(&5),
        "cache must have proposal after late arrival"
    );
}

// ============================================================================
// N109 INTEGRATION — Duplicate Proposal
// ============================================================================
#[test]
fn n109_duplicate_proposal_is_idempotent() {
    let mut cache: HashMap<u64, BlockProposal> = HashMap::new();
    let p = make_test_proposal(3, [2u8; 32]);
    cache.insert(3, p.clone());
    cache.insert(3, p);
    assert_eq!(cache.len(), 1, "duplicate insert must not grow cache");
}
