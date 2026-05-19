use amun_quorum_certificate::QuorumCertificate;

pub fn is_finalized_simple(
    block_qc: &QuorumCertificate,
    child_qc: &QuorumCertificate,
) -> bool {
    if block_qc.votes.is_empty() || child_qc.votes.is_empty() {
        return false;
    }
    if child_qc.position.sequence != block_qc.position.sequence + 1 {
        return false;
    }
    if child_qc.parent_hash != block_qc.block_hash {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_consensus_messages::{ConsensusVote, ConsensusPhase};
    use amun_chain_position::ChainPosition;

    fn create_vote(id: u64, round: u64, block_hash: [u8; 32], position: ChainPosition) -> ConsensusVote {
        ConsensusVote::new(id, position, round, ConsensusPhase::Prevote, Some(block_hash), [id as u8; 64], 25)
    }

    #[test]
    fn test_finality() {
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let block_a = [0xAA; 32];
        let block_b = [0xBB; 32];

        let qc_a = QuorumCertificate::new(pos1, 0, block_a, [0x00; 32],
            (1..=4).map(|id| create_vote(id, 0, block_a, pos1)).collect());
        let qc_b = QuorumCertificate::new(pos2, 1, block_b, block_a,
            (1..=4).map(|id| create_vote(id, 1, block_b, pos2)).collect());

        assert!(is_finalized_simple(&qc_a, &qc_b));
    }

    #[test]
    fn test_not_finalized_wrong_parent() {
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let block_a = [0xAA; 32];
        let block_b = [0xBB; 32];

        let qc_a = QuorumCertificate::new(pos1, 0, block_a, [0x00; 32],
            (1..=4).map(|id| create_vote(id, 0, block_a, pos1)).collect());
        let qc_b = QuorumCertificate::new(pos2, 1, block_b, [0xFF; 32],
            (1..=4).map(|id| create_vote(id, 1, block_b, pos2)).collect());

        assert!(!is_finalized_simple(&qc_a, &qc_b));
    }
}
