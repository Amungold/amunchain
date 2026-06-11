use crate::types::{BlockProposal, QuorumCertificate, Vote};
use amun_constitutional_commitments::smt::MerkleProof;

#[derive(Debug, Clone)]
pub enum ConsensusAction {
    BroadcastProposal(BlockProposal),
    BroadcastPrevote(Vote),
    BroadcastPrecommit(Vote),
    Commit(QuorumCertificate),
    AdvanceRound { from: u64, to: u64 },
    None,
}

#[derive(Debug, Clone)]
pub struct ActionRecord {
    pub action: ConsensusAction,
    pub height: u64,
    pub round: u64,
    pub step: String,
    pub validator_id: [u8; 32],
    pub prev_hash: Option<[u8; 32]>,
    pub hash: [u8; 32],
}

impl ActionRecord {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut state = [0u8; 32];
        for (i, b) in self.height.to_le_bytes().iter().enumerate() {
            state[i] ^= b;
        }
        for (i, b) in self.round.to_le_bytes().iter().enumerate() {
            state[i] ^= b;
        }
        for (i, b) in self.validator_id.iter().enumerate() {
            state[i] ^= b;
        }
        if let Some(prev) = &self.prev_hash {
            for i in 0..32 {
                state[i] ^= prev[i];
            }
        }
        state
    }
}

#[derive(Debug, Clone, Default)]
pub struct ActionLog {
    pub records: Vec<ActionRecord>,
}

impl ActionLog {
    pub fn record(
        &mut self,
        action: ConsensusAction,
        height: u64,
        round: u64,
        step: &str,
        validator_id: [u8; 32],
    ) {
        let prev_hash = self.records.last().map(|r| r.hash);
        let mut record = ActionRecord {
            action,
            height,
            round,
            step: step.to_string(),
            validator_id,
            prev_hash,
            hash: [0u8; 32],
        };
        record.hash = record.compute_hash();
        self.records.push(record);
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn verify(&self) -> bool {
        for i in 0..self.records.len() {
            let expected_prev = if i > 0 {
                Some(self.records[i - 1].hash)
            } else {
                None
            };
            if self.records[i].prev_hash != expected_prev {
                return false;
            }
            if self.records[i].hash != self.records[i].compute_hash() {
                return false;
            }
        }
        true
    }

    pub fn evidence_root(&self) -> [u8; 32] {
        let mut tree =
            amun_constitutional_commitments::SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
        for (i, record) in self.records.iter().enumerate() {
            let key = format!("{:020}_{:020}_{:020}", record.height, record.round, i);
            tree.insert(key.as_bytes(), &record.hash);
        }
        tree.root()
    }

    pub fn prove(&self, index: usize) -> Option<EvidenceProof> {
        if index >= self.records.len() {
            return None;
        }
        let mut tree =
            amun_constitutional_commitments::SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
        for (i, r) in self.records.iter().enumerate() {
            let key = format!("{:020}_{:020}_{:020}", r.height, r.round, i);
            tree.insert(key.as_bytes(), &r.hash);
        }
        let record = &self.records[index];
        let key = format!("{:020}_{:020}_{:020}", record.height, record.round, index);
        let proof = tree.prove(key.as_bytes());
        Some(EvidenceProof {
            action_hash: record.hash,
            proof,
            evidence_root: tree.root(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EvidenceProof {
    pub action_hash: [u8; 32],
    pub proof: MerkleProof,
    pub evidence_root: [u8; 32],
}

impl EvidenceProof {
    pub fn verify(&self) -> bool {
        let tree = amun_constitutional_commitments::SparseMerkleTree::new(b"AMUN_EVIDENCE_DOMAIN");
        tree.verify(&self.evidence_root, &self.proof)
    }
}
