use crate::messages::EquivocationProof;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Classification of validator misbehavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OffenseType {
    DoubleVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MisbehaviorRecord {
    pub proof_hash: [u8; 32],
    pub validator_id: [u8; 32],
    pub offense_type: OffenseType,
    pub height: u64,
    pub round: u64,
    pub proof: EquivocationProof,
}

pub struct MisbehaviorRegistry {
    records: HashMap<[u8; 32], MisbehaviorRecord>,
    file_path: Option<std::path::PathBuf>,
}

impl MisbehaviorRegistry {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            file_path: None,
        }
    }

    pub fn open(path: &Path) -> Result<Self, String> {
        if path.exists() {
            let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let records: Vec<MisbehaviorRecord> =
                serde_json::from_str(&data).map_err(|e| e.to_string())?;
            let map: HashMap<[u8; 32], MisbehaviorRecord> =
                records.into_iter().map(|r| (r.proof_hash, r)).collect();
            Ok(Self {
                records: map,
                file_path: Some(path.to_path_buf()),
            })
        } else {
            Ok(Self {
                records: HashMap::new(),
                file_path: Some(path.to_path_buf()),
            })
        }
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(ref path) = self.file_path {
            let records: Vec<&MisbehaviorRecord> = self.records.values().collect();
            let json = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
            fs::write(path, json).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn hash_proof(proof: &EquivocationProof) -> [u8; 32] {
        let bytes = postcard::to_stdvec(proof).unwrap_or_default();
        blake3::hash(&bytes).into()
    }

    pub fn add_proof(&mut self, proof: EquivocationProof) -> Result<[u8; 32], String> {
        proof.verify_standalone()?;
        let proof_hash = Self::hash_proof(&proof);
        if self.records.contains_key(&proof_hash) {
            return Ok(proof_hash);
        }
        let record = MisbehaviorRecord {
            proof_hash,
            validator_id: proof.validator_id,
            offense_type: OffenseType::DoubleVote,
            height: proof.height,
            round: proof.round,
            proof,
        };
        self.records.insert(proof_hash, record);
        self.save()?;
        Ok(proof_hash)
    }

    pub fn validator_history(&self, validator_id: &[u8; 32]) -> Vec<&MisbehaviorRecord> {
        self.records
            .values()
            .filter(|r| r.validator_id == *validator_id)
            .collect()
    }

    pub fn offense_count(&self, validator_id: &[u8; 32]) -> u64 {
        self.records
            .values()
            .filter(|r| r.validator_id == *validator_id)
            .count() as u64
    }

    /// Return all records in the registry
    pub fn all_proofs(&self) -> Vec<&MisbehaviorRecord> {
        self.records.values().collect()
    }

    pub fn contains_proof(&self, proof_hash: &[u8; 32]) -> bool {
        self.records.contains_key(proof_hash)
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl Default for MisbehaviorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{ConsensusVote, SignedVote};

    fn make_signed_vote(voter: u8, height: u64, block_hash: [u8; 32], round: u64) -> SignedVote {
        let vote = ConsensusVote {
            voter_id: [voter; 32],
            height,
            block_hash,
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: round,
        };
        SignedVote {
            vote,
            signature: [1u8; 64],
        }
    }

    #[test]
    fn n101_3_add_valid_proof() {
        let mut registry = MisbehaviorRegistry::new();
        let mut proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        proof.vote_a.signature = [1u8; 64];
        proof.vote_b.signature = [2u8; 64];
        let hash = registry.add_proof(proof).unwrap();
        assert!(registry.contains_proof(&hash));
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.offense_count(&[1u8; 32]), 1);
    }

    #[test]
    fn n101_3_reject_invalid_proof() {
        let mut registry = MisbehaviorRegistry::new();
        let proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xAA; 32], 1),
            detected_at_height: 11,
        };
        assert!(registry.add_proof(proof).is_err());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn n101_3_validator_history() {
        let mut registry = MisbehaviorRegistry::new();
        let mut proof1 = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        proof1.vote_a.signature = [1u8; 64];
        proof1.vote_b.signature = [2u8; 64];
        registry.add_proof(proof1).unwrap();

        let mut proof2 = EquivocationProof {
            validator_id: [1u8; 32],
            height: 20,
            round: 1,
            vote_a: make_signed_vote(1, 20, [0xCC; 32], 1),
            vote_b: make_signed_vote(1, 20, [0xDD; 32], 1),
            detected_at_height: 21,
        };
        proof2.vote_a.signature = [3u8; 64];
        proof2.vote_b.signature = [4u8; 64];
        registry.add_proof(proof2).unwrap();

        let history = registry.validator_history(&[1u8; 32]);
        assert_eq!(history.len(), 2);
        assert_eq!(registry.offense_count(&[1u8; 32]), 2);
        assert_eq!(registry.offense_count(&[2u8; 32]), 0);
    }

    #[test]
    fn n101_3_duplicate_proof_deduplicated() {
        let mut registry = MisbehaviorRegistry::new();
        let mut proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        proof.vote_a.signature = [1u8; 64];
        proof.vote_b.signature = [2u8; 64];
        registry.add_proof(proof.clone()).unwrap();
        registry.add_proof(proof).unwrap();
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn n101_3_offense_count() {
        let mut registry = MisbehaviorRegistry::new();
        let mut proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        proof.vote_a.signature = [1u8; 64];
        proof.vote_b.signature = [2u8; 64];
        registry.add_proof(proof).unwrap();

        let mut proof2 = EquivocationProof {
            validator_id: [2u8; 32],
            height: 15,
            round: 2,
            vote_a: make_signed_vote(2, 15, [0xAA; 32], 2),
            vote_b: make_signed_vote(2, 15, [0xBB; 32], 2),
            detected_at_height: 16,
        };
        proof2.vote_a.signature = [5u8; 64];
        proof2.vote_b.signature = [6u8; 64];
        registry.add_proof(proof2).unwrap();

        assert_eq!(registry.offense_count(&[1u8; 32]), 1);
        assert_eq!(registry.offense_count(&[2u8; 32]), 1);
        assert_eq!(registry.offense_count(&[3u8; 32]), 0);
    }

    #[test]
    fn n101_3_persistence_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry.json");

        let mut registry = MisbehaviorRegistry::open(&path).unwrap();
        let mut proof = EquivocationProof {
            validator_id: [1u8; 32],
            height: 10,
            round: 1,
            vote_a: make_signed_vote(1, 10, [0xAA; 32], 1),
            vote_b: make_signed_vote(1, 10, [0xBB; 32], 1),
            detected_at_height: 11,
        };
        proof.vote_a.signature = [1u8; 64];
        proof.vote_b.signature = [2u8; 64];
        registry.add_proof(proof).unwrap();
        registry.save().unwrap();

        let reopened = MisbehaviorRegistry::open(&path).unwrap();
        assert_eq!(reopened.len(), 1);
        assert_eq!(reopened.offense_count(&[1u8; 32]), 1);

        let history = reopened.validator_history(&[1u8; 32]);
        assert_eq!(history[0].offense_type, OffenseType::DoubleVote);
    }
}
