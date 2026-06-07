use serde::{Serialize, Deserialize};

/// A validator's vote for a proposed block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsensusVote {
    #[serde(with = "serde_bytes")]
    pub voter_id: [u8; 32],
    pub height: u64,
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],
    pub approve: bool,
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
    pub timestamp: u64,
}

/// A quorum certificate: proof that >2/3 validators approved.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuorumCertificate {
    pub height: u64,
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],
    pub votes: Vec<ConsensusVote>,
    pub quorum_size: usize,
    pub total_validators: usize,
}

impl QuorumCertificate {
    pub fn verify_quorum(&self) -> bool {
        let approvals = self.votes.iter().filter(|v| v.approve).count();
        approvals * 3 > self.total_validators * 2
    }

    pub fn verify_consistency(&self) -> bool {
        self.votes.iter().all(|v| {
            v.height == self.height && v.block_hash == self.block_hash
        })
    }

    pub fn verify(&self) -> bool {
        self.verify_quorum() && self.verify_consistency()
    }
}

/// A finality certificate issued after a QC is formed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FinalityCertificate {
    pub height: u64,
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub history_root: [u8; 32],
    pub qc: QuorumCertificate,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vote(voter: u8, height: u64, block_hash: [u8; 32], approve: bool) -> ConsensusVote {
        ConsensusVote {
            voter_id: [voter; 32],
            height,
            block_hash,
            state_root: [0xBB; 32],
            approve,
            signature: [0u8; 64],
            timestamp: 1000,
        }
    }

    #[test]
    fn n68_qc_verifies_with_supermajority() {
        let hash = [0xAA; 32];
        let qc = QuorumCertificate {
            height: 1,
            block_hash: hash,
            state_root: [0xBB; 32],
            votes: vec![
                make_vote(1, 1, hash, true),
                make_vote(2, 1, hash, true),
                make_vote(3, 1, hash, true),
                make_vote(4, 1, hash, false),
            ],
            quorum_size: 3,
            total_validators: 4,
        };
        assert!(qc.verify());
    }

    #[test]
    fn n68_qc_rejects_insufficient_quorum() {
        let hash = [0xAA; 32];
        let qc = QuorumCertificate {
            height: 1,
            block_hash: hash,
            state_root: [0xBB; 32],
            votes: vec![
                make_vote(1, 1, hash, true),
                make_vote(2, 1, hash, false),
                make_vote(3, 1, hash, false),
                make_vote(4, 1, hash, true),
            ],
            quorum_size: 2,
            total_validators: 4,
        };
        assert!(!qc.verify_quorum());
    }

    #[test]
    fn n68_qc_rejects_inconsistent_votes() {
        let qc = QuorumCertificate {
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            votes: vec![
                make_vote(1, 1, [0xAA; 32], true),
                make_vote(2, 1, [0xBB; 32], true),
                make_vote(3, 1, [0xAA; 32], true),
                make_vote(4, 1, [0xAA; 32], true),
            ],
            quorum_size: 4,
            total_validators: 4,
        };
        assert!(!qc.verify_consistency());
    }

    #[test]
    fn n68_roundtrip_vote_serialization() {
        let vote = make_vote(1, 42, [0xAA; 32], true);
        let encoded = postcard::to_stdvec(&vote).unwrap();
        let decoded: ConsensusVote = postcard::from_bytes(&encoded).unwrap();
        assert_eq!(decoded.voter_id, vote.voter_id);
        assert_eq!(decoded.height, 42);
        assert!(decoded.approve);
    }

    #[test]
    fn n68_roundtrip_qc_serialization() {
        let hash = [0xAA; 32];
        let qc = QuorumCertificate {
            height: 1,
            block_hash: hash,
            state_root: [0xBB; 32],
            votes: vec![make_vote(1, 1, hash, true)],
            quorum_size: 1,
            total_validators: 1,
        };
        let encoded = postcard::to_stdvec(&qc).unwrap();
        let decoded: QuorumCertificate = postcard::from_bytes(&encoded).unwrap();
        assert!(decoded.verify());
    }
}
