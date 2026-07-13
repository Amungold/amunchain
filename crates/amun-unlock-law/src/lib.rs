use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;

/// Check if a validator can be unlocked (has contributed to a valid QC)
pub fn can_unlock(validator_id: u64, qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
    qc.votes
        .iter()
        .any(|v| v.message.validator_id == validator_id)
        && validator_set.get_validator(validator_id).is_some()
}

/// Verify unlock condition: QC must have quorum
pub fn verify_unlock_condition(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
    let total_weight: u64 = qc
        .votes
        .iter()
        .filter_map(|v| validator_set.get_validator(v.message.validator_id))
        .map(|vi| vi.stake)
        .sum();

    validator_set.has_quorum(total_weight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_chain_position::ChainPosition;
    use amun_consensus_messages::ConsensusPhase;
    use amun_consensus_messages::ConsensusVote;
    use amun_validator_attestation::validator_set::ValidatorInfo;

    #[test]
    fn test_unlock() {
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

        assert!(can_unlock(1, &qc, &set));
        assert!(!can_unlock(99, &qc, &set));
        assert!(verify_unlock_condition(&qc, &set));
    }
}
