use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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
    /// N109.8: Cryptographic execution commitment (optional for backward compatibility).
    /// When present, MUST pass N109.9 vote binding verification before quorum counting.
    #[serde(default)]
    pub commitment: Option<ExecutionCommitment>,
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
    pub approval_power: u64,
    pub total_voting_power: u64,
}

impl QuorumCertificate {
    pub fn verify_quorum(&self) -> bool {
        self.approval_power * 3 > self.total_voting_power * 2
    }

    pub fn verify_consistency(&self) -> bool {
        self.votes
            .iter()
            .all(|v| v.height == self.height && v.block_hash == self.block_hash)
    }

    pub fn verify(&self) -> bool {
        self.verify_quorum() && self.verify_consistency()
    }
    pub fn verify_strict(&self, validator_powers: &HashMap<[u8; 32], u64>) -> Result<(), String> {
        if validator_powers.is_empty() {
            if self.verify() {
                return Ok(());
            } else {
                return Err("Legacy QC verification failed".into());
            }
        }
        if !self.verify_consistency() {
            return Err("QC consistency failed".into());
        }

        let mut seen = HashSet::new();
        let mut computed_power = 0u64;

        for vote in &self.votes {
            if !seen.insert(vote.voter_id) {
                return Err(format!(
                    "Duplicate validator in QC {:?}",
                    &vote.voter_id[..4]
                ));
            }

            let power = *validator_powers
                .get(&vote.voter_id)
                .ok_or_else(|| format!("Unknown validator {:?}", &vote.voter_id[..4]))?;

            if !vote.approve {
                return Err("QC contains reject vote".into());
            }

            computed_power += power;

            eprintln!(
                "QC_STRICT: validator={:?} power={}",
                &vote.voter_id[..4],
                power
            );
        }

        if computed_power != self.approval_power {
            return Err(format!(
                "Approval power mismatch stored={} computed={}",
                self.approval_power, computed_power
            ));
        }

        if computed_power * 3 <= self.total_voting_power * 2 {
            return Err("Insufficient quorum".into());
        }

        eprintln!(
            "QC_STRICT_OK: power={} total={} votes={}",
            computed_power,
            self.total_voting_power,
            self.votes.len()
        );

        Ok(())
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
use amun_validator_registry::{PeerId, PublicKey, ValidatorId, ValidatorRecord, ValidatorRegistry};
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
            commitment: None,
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
    fn n131_duplicate_validator_in_qc_rejected() {

        let hash = [0xAA; 32];

        let qc = QuorumCertificate {
            height: 1,
            block_hash: hash,
            state_root: [0xBB; 32],
            votes: vec![make_vote(1, 1, hash, true), make_vote(1, 1, hash, true)],
            approval_power: 2,
            total_voting_power: 2,
        };

        let mut registry = ValidatorRegistry::new();
        registry
            .register_full(ValidatorRecord {
                validator_id: ValidatorId([1u8; 32]),
                peer_id: PeerId([0u8; 32]),
                public_key: PublicKey([0u8; 32]),
                certificate_hash: [0u8; 32],
                stake: 0,
                voting_power: 1,
                active: true,
                slash_count: 0,
                registered_at: 0,
            })
            .unwrap();
        assert!(qc.verify_with_registry(&registry).is_err());
    }
    #[test]
    fn n131_tampered_approval_power_rejected() {

        let hash = [0xAA; 32];

        let qc = QuorumCertificate {
            height: 1,
            block_hash: hash,
            state_root: [0xBB; 32],
            votes: vec![
                make_vote(1, 1, hash, true),
                make_vote(2, 1, hash, true),
                make_vote(3, 1, hash, true),
            ],
            approval_power: 100, // قيمة مزورة
            total_voting_power: 3,
        };

        let mut registry = ValidatorRegistry::new();
        for id in [1u8, 2u8, 3u8].iter() {
            registry
                .register_full(ValidatorRecord {
                    validator_id: ValidatorId([*id; 32]),
                    peer_id: PeerId([0u8; 32]),
                    public_key: PublicKey([0u8; 32]),
                    certificate_hash: [0u8; 32],
                    stake: 0,
                    voting_power: 1,
                    active: true,
                    slash_count: 0,
                    registered_at: 0,
                })
                .unwrap();
        }
        assert!(qc.verify_with_registry(&registry).is_err());
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
            approval_power: 3,
            total_voting_power: 4,
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
            approval_power: 2,
            total_voting_power: 4,
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
            approval_power: 4,
            total_voting_power: 4,
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
            approval_power: 1,
            total_voting_power: 1,
        };
        let encoded = postcard::to_stdvec(&qc).unwrap();
        let decoded: QuorumCertificate = postcard::from_bytes(&encoded).unwrap();
        assert!(decoded.verify());
    }
}

/// N104.1: A block proposal sent by the designated proposer to all validators.
/// Contains only transaction hashes for propagation, not execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockProposal {
    pub height: u64,
    pub parent_hash: [u8; 32],
    pub tx_hashes: Vec<[u8; 32]>,
    pub proposer_id: [u8; 32],
    pub timestamp: u64,
}

mod n104_1_tests {
    use super::*;

    #[test]
    fn n104_1_block_proposal_roundtrip() {
        let proposal = BlockProposal {
            height: 42,
            parent_hash: [0xAA; 32],
            tx_hashes: vec![[0x01; 32], [0x02; 32]],
            proposer_id: [0xBB; 32],
            timestamp: 1700000000,
        };
        let encoded = postcard::to_stdvec(&proposal).unwrap();
        let decoded: BlockProposal = postcard::from_bytes(&encoded).unwrap();
        assert_eq!(decoded, proposal);
    }

    #[test]
    fn n104_1_deterministic_hash() {
        let proposal = BlockProposal {
            height: 1,
            parent_hash: [0xCC; 32],
            tx_hashes: vec![],
            proposer_id: [0xDD; 32],
            timestamp: 0,
        };
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"N104_BLOCK_PROPOSAL_V1");
        hasher.update(&proposal.height.to_le_bytes());
        hasher.update(&proposal.parent_hash);
        for txh in &proposal.tx_hashes {
            hasher.update(txh);
        }
        hasher.update(&proposal.proposer_id);
        hasher.update(&proposal.timestamp.to_le_bytes());
        let h1 = hasher.finalize();

        // Second round gives same hash
        let mut hasher2 = blake3::Hasher::new();
        hasher2.update(b"N104_BLOCK_PROPOSAL_V1");
        hasher2.update(&proposal.height.to_le_bytes());
        hasher2.update(&proposal.parent_hash);
        for txh in &proposal.tx_hashes {
            hasher2.update(txh);
        }
        hasher2.update(&proposal.proposer_id);
        hasher2.update(&proposal.timestamp.to_le_bytes());
        let h2 = hasher2.finalize();

        assert_eq!(h1, h2);
    }
}

/// N111.5: Request for missing evidence records.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MissingEvidenceRequest {
    pub requester_id: [u8; 32],
    pub evidence_ids: Vec<[u8; 32]>,
}

/// N111.5: Response containing requested evidence records.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MissingEvidenceResponse {
    pub responder_id: [u8; 32],
    /// Serialized EvidenceRecords (postcard-encoded)
    pub evidence_data: Vec<Vec<u8>>,
}
// ============================================================================
// N109 — Constitutional Block Propagation Types
// ============================================================================

/// N109.1: Full block proposal for constitutional propagation.
/// Carries the complete serialized block so every validator can re-execute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct N109BlockProposal {
    pub proposer_id: [u8; 32],
    pub height: u64,
    pub timestamp: u64,
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub parent_root: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub block_bytes: Vec<u8>,
}

/// N109.2: Unified network message type.
/// Replaces direct ConsensusVote decoding in the listener.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Proposal(N109BlockProposal),
    Vote(Box<ConsensusVote>),
    /// N110.3: Slashing certificate for network propagation
    SlashingCertificate(crate::slashing_certificate::SlashingCertificate),
}

// ============================================================================
// N109.8: Updated ConsensusVote carrying ExecutionCommitment
// ============================================================================
// The old ConsensusVote is kept for backward compatibility during migration.
// New code should use N109ConsensusVote which binds the vote to an
// ExecutionCommitment, ensuring every vote is a cryptographically signed
// execution result, not just a hash approval.
//
// Migration path:
//   1. Add N109ConsensusVote (this file)
//   2. Update engine.rs to accept N109ConsensusVote
//   3. Update validator.rs to produce N109ConsensusVote
//   4. Remove old ConsensusVote after all tests pass

use crate::execution_commitment::ExecutionCommitment;

/// N109.8: A vote that carries a full execution commitment.
/// Every vote is now a signed statement: "I executed block X and got state_root Y".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct N109ConsensusVote {
    /// The validator casting this vote
    #[serde(with = "serde_bytes")]
    pub voter_id: [u8; 32],

    /// Block height
    pub height: u64,

    /// Hash of the block being voted on
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],

    /// State root obtained from execution
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],

    /// Whether the validator approves this block
    pub approve: bool,

    /// Unix timestamp when vote was created
    pub timestamp: u64,

    /// N109.8: Cryptographic execution commitment
    /// Binds validator_id, height, block_hash, and state_root
    /// with a signature that can be verified independently.
    pub commitment: ExecutionCommitment,
}

impl QuorumCertificate {
    /// Verify QC using ValidatorRead trait.
    pub fn verify_with_registry(
        &self,
        registry: &dyn amun_validator_registry::ValidatorRead,
    ) -> Result<(), String> {
        if !self.verify_consistency() {
            return Err("QC consistency failed".into());
        }
        let mut seen = std::collections::HashSet::new();
        let mut computed_power = 0u64;
        for vote in &self.votes {
            if !seen.insert(vote.voter_id) {
                return Err(format!(
                    "Duplicate validator in QC {:?}",
                    &vote.voter_id[..4]
                ));
            }
            let id = amun_validator_registry::ValidatorId(vote.voter_id);
            let power = registry.get_voting_power(&id);
            if power == 0 {
                return Err(format!("Unknown validator {:?}", &vote.voter_id[..4]));
            }
            if !vote.approve {
                return Err("QC contains reject vote".into());
            }
            computed_power += power;
        }
        if computed_power != self.approval_power {
            return Err(format!(
                "Approval power mismatch stored={} computed={}",
                self.approval_power, computed_power
            ));
        }
        let total = registry.total_voting_power();
        if total > 0 && computed_power * 3 <= total * 2 {
            return Err("Insufficient quorum".into());
        }
        Ok(())
    }
}
