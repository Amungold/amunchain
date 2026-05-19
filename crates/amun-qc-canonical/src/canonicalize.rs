use amun_quorum_certificate::QuorumCertificate;
use amun_consensus_messages::ConsensusVote;
use amun_validator_attestation::ValidatorSet;

/// Canonicalize a QC by removing duplicate and invalid votes
pub fn canonicalize_qc(
    qc: &QuorumCertificate,
    validator_set: &ValidatorSet,
) -> QuorumCertificate {
    use std::collections::HashSet;

    let mut seen_validators = HashSet::new();
    let mut canonical_votes = Vec::new();

    for vote in &qc.votes {
        // Skip if validator already voted
        if !seen_validators.insert(vote.message.validator_id) {
            continue;
        }

        // Skip invalid votes
        if !is_vote_valid(vote, validator_set) {
            continue;
        }

        canonical_votes.push(vote.clone());
    }

    QuorumCertificate::new(
        qc.position,
        qc.round,
        qc.block_hash,
        qc.parent_hash,
        canonical_votes,
    )
}

/// Check if a vote is structurally valid
fn is_vote_valid(vote: &ConsensusVote, validator_set: &ValidatorSet) -> bool {
    // Validator must exist
    if validator_set.get_validator(vote.message.validator_id).is_none() {
        return false;
    }

    // Must have a block hash
    if vote.message.block_hash.is_none() {
        return false;
    }

    // Signature must not be zero
    if vote.signature == [0u8; 64] {
        return false;
    }

    true
}

/// Check if canonicalized QC reaches quorum
pub fn has_quorum_after_canonicalization(
    qc: &QuorumCertificate,
    validator_set: &ValidatorSet,
) -> bool {
    let canonical = canonicalize_qc(qc, validator_set);
    let total_weight: u64 = canonical.votes.iter()
        .filter_map(|v| validator_set.get_validator(v.message.validator_id))
        .map(|vi| vi.stake)
        .sum();

    validator_set.has_quorum(total_weight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_consensus_messages::ConsensusPhase;
    use amun_chain_position::ChainPosition;
    use amun_validator_attestation::validator_set::ValidatorInfo;

    #[test]
    fn test_canonicalize_removes_duplicates() {
        let set = ValidatorSet::new(1, vec![
            ValidatorInfo { id: 1, public_key: [1u8; 32], stake: 25 },
            ValidatorInfo { id: 2, public_key: [2u8; 32], stake: 25 },
        ]).unwrap();

        let pos = ChainPosition::new(0, 1);
        let block = [0xAA; 32];

        let mut votes = Vec::new();
        votes.push(ConsensusVote::new(1, pos, 0, ConsensusPhase::Prevote, Some(block), [1u8; 64], 25));
        votes.push(ConsensusVote::new(1, pos, 0, ConsensusPhase::Prevote, Some(block), [2u8; 64], 25)); // Duplicate

        let qc = QuorumCertificate::new(pos, 0, block, [0x00; 32], votes);
        let canonical = canonicalize_qc(&qc, &set);

        assert_eq!(canonical.votes.len(), 1);
    }
}
