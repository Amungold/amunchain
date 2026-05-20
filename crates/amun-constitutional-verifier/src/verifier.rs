use amun_consensus_messages::ConsensusVote;
use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;

/// Verify a QuorumCertificate structurally
pub fn verify_qc(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
    // Must have votes
    if qc.votes.is_empty() {
        return false;
    }

    // All votes must be for the same block
    let expected_block = qc.block_hash;
    for vote in &qc.votes {
        if vote.message.block_hash != Some(expected_block) {
            return false;
        }
    }

    // Check quorum
    let total_weight: u64 = qc
        .votes
        .iter()
        .filter_map(|v| validator_set.get_validator(v.message.validator_id))
        .map(|vi| vi.stake)
        .sum();

    if !validator_set.has_quorum(total_weight) {
        return false;
    }

    true
}

/// Verify individual vote
pub fn verify_vote(vote: &ConsensusVote, validator_set: &ValidatorSet) -> bool {
    // Validator must exist
    if validator_set
        .get_validator(vote.message.validator_id)
        .is_none()
    {
        return false;
    }

    // Must have a block hash
    if vote.message.block_hash.is_none() {
        return false;
    }

    // Signature exists (full verification would check crypto)
    if vote.signature == [0u8; 64] {
        return false;
    }

    true
}

/// Verify vote uniqueness in a QC
pub fn verify_vote_uniqueness(qc: &QuorumCertificate) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for vote in &qc.votes {
        if !seen.insert(vote.message.validator_id) {
            return false; // Duplicate validator
        }
    }
    true
}

/// Get quorum status for a QC
pub fn check_quorum(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> Result<u64, String> {
    let total_weight: u64 = qc
        .votes
        .iter()
        .filter_map(|v| validator_set.get_validator(v.message.validator_id))
        .map(|vi| vi.stake)
        .sum();

    let threshold = validator_set.quorum_threshold();

    if total_weight >= threshold {
        Ok(total_weight)
    } else {
        Err(format!(
            "Quorum not reached: have {}, need {}",
            total_weight, threshold
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_chain_position::ChainPosition;
    use amun_consensus_messages::ConsensusPhase;
    use amun_validator_attestation::validator_set::ValidatorInfo;

    #[test]
    fn test_verify_qc() {
        let set = ValidatorSet::new(
            1,
            vec![
                ValidatorInfo {
                    id: 1,
                    public_key: [1u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 2,
                    public_key: [2u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 3,
                    public_key: [3u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 4,
                    public_key: [4u8; 32],
                    stake: 25,
                },
            ],
        )
        .unwrap();

        let pos = ChainPosition::new(0, 1);
        let block = [0xAA; 32];
        let votes: Vec<ConsensusVote> = (1..=4)
            .map(|id| {
                ConsensusVote::new(
                    id,
                    pos,
                    0,
                    ConsensusPhase::Prevote,
                    Some(block),
                    [id as u8; 64],
                    25,
                )
            })
            .collect();

        let qc = QuorumCertificate::new(pos, 0, block, [0x00; 32], votes);

        assert!(verify_qc(&qc, &set));
        assert!(verify_vote_uniqueness(&qc));
        assert!(check_quorum(&qc, &set).is_ok());
    }
}
