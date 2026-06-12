use serde::{Deserialize, Serialize};

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

/// A signed vote — the vote payload plus a cryptographic signature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedVote {
    pub vote: ConsensusVote,
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
}

/// An equivocation proof: evidence that a validator voted for two different blocks
/// at the same height and round.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EquivocationProof {
    pub validator_id: [u8; 32],
    pub height: u64,
    pub round: u64,
    pub vote_a: SignedVote,
    pub vote_b: SignedVote,
    pub detected_at_height: u64,
}

impl EquivocationProof {
    /// Verify the proof without any external state.
    pub fn verify_standalone(&self) -> Result<(), String> {
        let a = &self.vote_a.vote;
        let b = &self.vote_b.vote;
        if a.voter_id != self.validator_id || b.voter_id != self.validator_id {
            return Err("Validator ID mismatch".into());
        }
        if a.height != self.height || b.height != self.height {
            return Err("Height mismatch".into());
        }
        if a.timestamp != self.round || b.timestamp != self.round {
            return Err("Round mismatch".into());
        }
        if a.block_hash == b.block_hash {
            return Err("Same block hash – not equivocation".into());
        }
        if self.vote_a.signature == [0u8; 64] || self.vote_b.signature == [0u8; 64] {
            return Err("Missing signature".into());
        }
        if self.vote_a.signature == self.vote_b.signature {
            return Err("Identical signatures – likely duplicate".into());
        }
        Ok(())
    }
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
        self.votes
            .iter()
            .all(|v| v.height == self.height && v.block_hash == self.block_hash)
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

    fn make_signed_vote(voter: u8, height: u64, block_hash: [u8; 32], round: u64) -> SignedVote {
        let mut vote = make_vote(voter, height, block_hash, true);
        vote.timestamp = round;
        SignedVote {
            vote,
            signature: [1u8; 64],
        }
    }

    #[test]
    fn n101_2_valid_equivocation_proof_accepted() {
        let mut vote_a = make_signed_vote(1, 10, [0xAA; 32], 1);
        let mut vote_b = make_signed_vote(1, 10, [0xBB; 32], 1);
        vote_a.signature = [1u8; 64];
        vote_b.signature = [2u8; 64]; // different signature for different vote
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a,
            vote_b,
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_ok());
    }

    #[test]
    fn n101_2_different_validators_rejected() {
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(2, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_err());
    }

    #[test]
    fn n101_2_different_heights_rejected() {
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 11, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_err());
    }

    #[test]
    fn n101_2_same_block_hash_rejected() {
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xAA; 32], 1),
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_err());
    }

    #[test]
    fn n101_2_missing_signature_rejected() {
        let mut bad = make_signed_vote(1, 10, [0xBB; 32], 1);
        bad.signature = [0u8; 64];
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: bad,
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_err());
    }

    #[test]
    fn n101_2_different_rounds_rejected() {
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 2),
            detected_at_height: 11,
        };
        assert!(proof.verify_standalone().is_err());
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
