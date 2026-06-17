// ============================================================================
// N109.9 — Vote Binding Verification
// ============================================================================
use crate::execution_commitment::ExecutionCommitment;
use crate::messages::ConsensusVote;

/// N109.9: Verify that a vote's ExecutionCommitment is valid and
/// matches the vote's target (height, block_hash).
///
/// If the vote has NO commitment (legacy vote), the check passes
/// with a warning. This allows gradual migration to N109.8.
///
/// If the vote HAS a commitment, all four binding checks are enforced:
///   1. vote.height     == commitment.height
///   2. vote.block_hash == commitment.block_hash
///   3. execution_root is recomputed and verified
///   4. commitment signature is valid
pub fn verify_vote_binding(vote: &ConsensusVote) -> Result<(), String> {
    // N109.9: If no commitment present, allow (backward compat)
    let c = match &vote.commitment {
        Some(commitment) => commitment,
        None => {
            // Legacy vote — no commitment to verify
            // TODO: Remove this branch once all validators migrate to N109.8
            return Ok(());
        }
    };

    // N109.9.1: Height binding
    if vote.height != c.height {
        return Err(format!(
            "N109.9 HEIGHT_MISMATCH: vote.height={} commitment.height={}",
            vote.height, c.height
        ));
    }

    // N109.9.2: Block binding
    if vote.block_hash != c.block_hash {
        return Err(format!(
            "N109.9 BLOCK_MISMATCH: vote.block_hash={:?} commitment.block_hash={:?}",
            &vote.block_hash[..4],
            &c.block_hash[..4]
        ));
    }

    // N109.9.3: Execution root integrity
    let recomputed = ExecutionCommitment::compute_execution_root(
        &c.validator_id,
        c.height,
        &c.block_hash,
        &c.state_root,
    );
    if recomputed != c.execution_root {
        return Err(format!(
            "N109.9 EXEC_ROOT_MISMATCH: stated={:?} recomputed={:?}",
            &c.execution_root[..4],
            &recomputed[..4]
        ));
    }

    // N109.9.4: Signature verification
    c.verify()
        .map_err(|e| format!("N109.9 SIGNATURE_INVALID: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_commitment::ExecutionCommitment;
    use crate::messages::ConsensusVote;
    use ed25519_dalek::SigningKey;

    fn make_keypair(seed: u8) -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    fn make_signed_commitment(
        sk: &SigningKey,
        pk: [u8; 32],
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
    ) -> ExecutionCommitment {
        let mut commit = ExecutionCommitment::new(pk, height, block_hash, state_root);
        commit.sign(sk);
        commit
    }

    fn make_vote_with_commitment(
        sk: &SigningKey,
        pk: [u8; 32],
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
    ) -> ConsensusVote {
        ConsensusVote {
            voter_id: pk,
            height,
            block_hash,
            state_root,
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
            commitment: Some(make_signed_commitment(
                sk, pk, height, block_hash, state_root,
            )),
        }
    }

    fn make_legacy_vote(pk: [u8; 32], height: u64, block_hash: [u8; 32]) -> ConsensusVote {
        ConsensusVote {
            voter_id: pk,
            height,
            block_hash,
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
            commitment: None, // Legacy vote — no commitment
        }
    }

    // ========================================================================
    // N109.9 Tests: Commitment-Bearing Votes
    // ========================================================================

    #[test]
    fn n109_9_valid_vote_with_commitment_passes() {
        let (sk, pk) = make_keypair(42);
        let vote = make_vote_with_commitment(&sk, pk, 5, [0xAA; 32], [0xBB; 32]);
        assert!(verify_vote_binding(&vote).is_ok());
    }

    #[test]
    fn n109_9_height_mismatch_rejected() {
        let (sk, pk) = make_keypair(1);
        let mut vote = make_vote_with_commitment(&sk, pk, 5, [0xAA; 32], [0xBB; 32]);
        vote.height = 99; // Tamper vote height
        let err = verify_vote_binding(&vote).unwrap_err();
        assert!(err.contains("HEIGHT_MISMATCH"));
    }

    #[test]
    fn n109_9_block_hash_mismatch_rejected() {
        let (sk, pk) = make_keypair(2);
        let mut vote = make_vote_with_commitment(&sk, pk, 5, [0xAA; 32], [0xBB; 32]);
        vote.block_hash = [0xFF; 32]; // Tamper vote block_hash
        let err = verify_vote_binding(&vote).unwrap_err();
        assert!(err.contains("BLOCK_MISMATCH"));
    }

    #[test]
    fn n109_9_tampered_execution_root_rejected() {
        let (sk, pk) = make_keypair(3);
        let mut vote = make_vote_with_commitment(&sk, pk, 5, [0xAA; 32], [0xBB; 32]);
        vote.commitment.as_mut().unwrap().execution_root = [0xDE; 32];
        let err = verify_vote_binding(&vote).unwrap_err();
        assert!(err.contains("EXEC_ROOT_MISMATCH"));
    }

    #[test]
    fn n109_9_unsigned_commitment_rejected() {
        let (sk, pk) = make_keypair(4);
        let mut vote = make_vote_with_commitment(&sk, pk, 5, [0xAA; 32], [0xBB; 32]);
        vote.commitment.as_mut().unwrap().signature = [0u8; 64];
        let err = verify_vote_binding(&vote).unwrap_err();
        assert!(err.contains("SIGNATURE_INVALID"));
    }

    #[test]
    fn n109_9_wrong_signer_rejected() {
        let (sk1, pk1) = make_keypair(5);
        let (_sk2, pk2) = make_keypair(6);
        let mut vote = make_vote_with_commitment(&sk1, pk1, 5, [0xAA; 32], [0xBB; 32]);
        // Tamper: change validator_id AND keep old execution_root (attack attempt)
        vote.commitment.as_mut().unwrap().validator_id = pk2;
        // execution_root is still computed for pk1, but validator_id claims pk2
        // This mismatch will be caught by execution_root recomputation (N109.9.3)
        let err = verify_vote_binding(&vote).unwrap_err();
        assert!(
            err.contains("EXEC_ROOT_MISMATCH") || err.contains("SIGNATURE_INVALID"),
            "Expected EXEC_ROOT_MISMATCH or SIGNATURE_INVALID, got: {}",
            err
        );
    }

    // ========================================================================
    // N109.9 Tests: Legacy Votes (Backward Compatibility)
    // ========================================================================

    #[test]
    fn n109_9_legacy_vote_without_commitment_passes() {
        let pk = [0x42; 32];
        let vote = make_legacy_vote(pk, 5, [0xAA; 32]);
        assert!(
            verify_vote_binding(&vote).is_ok(),
            "Legacy votes without commitment must be accepted for backward compatibility"
        );
    }

    #[test]
    fn n109_9_legacy_vote_still_allowed_during_migration() {
        // Simulate: 3 votes total, 2 with commitment, 1 legacy — all pass
        let (sk1, pk1) = make_keypair(10);
        let (sk2, pk2) = make_keypair(20);
        let pk3 = [0x30; 32];

        let v1 = make_vote_with_commitment(&sk1, pk1, 10, [0xCC; 32], [0xDD; 32]);
        let v2 = make_vote_with_commitment(&sk2, pk2, 10, [0xCC; 32], [0xDD; 32]);
        let v3 = make_legacy_vote(pk3, 10, [0xCC; 32]);

        assert!(
            verify_vote_binding(&v1).is_ok(),
            "vote with commitment must pass"
        );
        assert!(
            verify_vote_binding(&v2).is_ok(),
            "vote with commitment must pass"
        );
        assert!(
            verify_vote_binding(&v3).is_ok(),
            "legacy vote must pass during migration"
        );
    }
}
