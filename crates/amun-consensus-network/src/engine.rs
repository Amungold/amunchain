use crate::messages::{ConsensusVote, QuorumCertificate, FinalityCertificate};
use std::collections::HashMap;

/// A single consensus round for one block height.
#[derive(Debug, Clone)]
pub struct ConsensusRound {
    pub height: u64,
    pub proposer_id: [u8; 32],
    pub proposed_block_hash: Option<[u8; 32]>,
    pub proposed_state_root: Option<[u8; 32]>,
    pub votes: Vec<ConsensusVote>,
    pub qc: Option<QuorumCertificate>,
    pub finality: Option<FinalityCertificate>,
    pub complete: bool,
}

impl ConsensusRound {
    pub fn new(height: u64, proposer_id: [u8; 32]) -> Self {
        Self {
            height,
            proposer_id,
            proposed_block_hash: None,
            proposed_state_root: None,
            votes: Vec::new(),
            qc: None,
            finality: None,
            complete: false,
        }
    }

    /// Proposer sets the block for this round.
    pub fn propose(&mut self, block_hash: [u8; 32], state_root: [u8; 32]) {
        self.proposed_block_hash = Some(block_hash);
        self.proposed_state_root = Some(state_root);
    }

    /// Add a validator's vote. Rejects duplicate voters.
    pub fn add_vote(&mut self, vote: ConsensusVote) -> Result<(), String> {
        if self.proposed_block_hash.is_none() {
            self.proposed_block_hash = Some(vote.block_hash);
            self.proposed_state_root = Some(vote.state_root);
        }
        if vote.height != self.height {
            return Err(format!("Vote height {} != round height {}", vote.height, self.height));
        }
        if self.votes.iter().any(|v| v.voter_id == vote.voter_id) {
            return Err("Duplicate vote from validator".into());
        }
        if let Some(hash) = self.proposed_block_hash {
            if vote.block_hash != hash {
                return Err("Vote for different block".into());
            }
        }
        self.votes.push(vote);
        Ok(())
    }

    /// Try to form a QC from collected votes.
    /// Returns Some(QC) if >2/3 validators approved.
    pub fn try_form_qc(&mut self, total_validators: usize) -> Option<QuorumCertificate> {
        let approvals: Vec<ConsensusVote> = self.votes.iter()
            .filter(|v| v.approve)
            .cloned()
            .collect();

        if approvals.len() * 3 <= total_validators * 2 {
            return None; // Insufficient quorum
        }

        let qc = QuorumCertificate {
            height: self.height,
            block_hash: self.proposed_block_hash?,
            state_root: self.proposed_state_root?,
            votes: approvals,
            quorum_size: total_validators * 2 / 3 + 1,
            total_validators,
        };

        if !qc.verify() {
            return None;
        }

        self.qc = Some(qc.clone());
        Some(qc)
    }

    /// Finalize the round with a QC, producing a finality certificate.
    pub fn finalize(&mut self, history_root: [u8; 32]) -> Option<FinalityCertificate> {
        let qc = self.qc.as_ref()?;
        let cert = FinalityCertificate {
            height: self.height,
            block_hash: qc.block_hash,
            state_root: qc.state_root,
            history_root,
            qc: qc.clone(),
            timestamp: 0,
        };
        self.finality = Some(cert.clone());
        self.complete = true;
        Some(cert)
    }
}

/// Full consensus engine managing rounds and history.
pub struct ConsensusEngine {
    pub validator_id: [u8; 32],
    pub total_validators: usize,
    pub current_height: u64,
    pub history_root: [u8; 32],
    pub rounds: HashMap<u64, ConsensusRound>,
    finality_chain: Vec<FinalityCertificate>,
}

impl ConsensusEngine {
    pub fn new(validator_id: [u8; 32], total_validators: usize) -> Self {
        Self {
            validator_id,
            total_validators,
            current_height: 0,
            history_root: [0u8; 32],
            rounds: HashMap::new(),
            finality_chain: Vec::new(),
        }
    }

    /// Start a new round for the given height.
    pub fn start_round(&mut self, height: u64, proposer_id: [u8; 32]) {
        self.rounds.entry(height).or_insert_with(|| ConsensusRound::new(height, proposer_id));
    }

    /// Get a mutable reference to the current round.
    pub fn round_mut(&mut self, height: u64) -> Option<&mut ConsensusRound> {
        self.rounds.get_mut(&height)
    }

    /// Process a vote for a round.
    pub fn process_vote(&mut self, vote: ConsensusVote) -> Result<(), String> {
        let height = vote.height;
        if height > self.current_height + 1 {
            return Err(format!(
                "Future vote height {} > current+1 {}", height, self.current_height + 1
            ));
        }
        if !self.rounds.contains_key(&height) {
            self.start_round(height, vote.voter_id);
        }
        let round = self.rounds.get_mut(&height)
            .ok_or_else(|| format!("No active round at height {}", height))?;
        round.add_vote(vote)
    }

    /// Try to advance: form QC, finalize, update history.
    pub fn try_advance(&mut self, height: u64, history_root: [u8; 32]) -> Option<FinalityCertificate> {
        let total = self.total_validators;
        let round = self.rounds.get_mut(&height)?;

        if round.qc.is_none() {
            round.try_form_qc(total)?;
        }

        let cert = round.finalize(history_root)?;
        self.current_height = height;
        self.history_root = history_root;
        self.finality_chain.push(cert.clone());
        Some(cert)
    }

    /// Get the proposer for a given height (round-robin).
    pub fn proposer_for(&self, height: u64) -> usize {
        ((height - 1) as usize) % self.total_validators
    }

    pub fn is_finalized(&self, height: u64) -> bool {
        self.rounds.get(&height).is_some_and(|r| r.complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n68_round_propose_vote_qc_finalize() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);

        let proposer = [1u8; 32];
        engine.start_round(1, proposer);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        // 3 validators approve (>2/3 of 4)
        for id in [1u8, 2, 3] {
            let vote = ConsensusVote {
                voter_id: [id; 32],
                height: 1,
                block_hash: [0xAA; 32],
                state_root: [0xBB; 32],
                approve: true,
                signature: [0u8; 64],
                timestamp: 1000,
            };
            engine.process_vote(vote).unwrap();
        }

        let cert = engine.try_advance(1, [0xCC; 32]).unwrap();
        assert_eq!(cert.height, 1);
        assert_eq!(cert.block_hash, [0xAA; 32]);
        assert_eq!(engine.current_height, 1);
        assert_eq!(engine.history_root, [0xCC; 32]);
        assert!(engine.is_finalized(1));
    }

    #[test]
    fn n68_insufficient_quorum_no_qc() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        // Only 2 approvals (50%, not >66%)
        for id in [1u8, 2] {
            engine.process_vote(ConsensusVote {
                voter_id: [id; 32],
                height: 1,
                block_hash: [0xAA; 32],
                state_root: [0xBB; 32],
                approve: true,
                signature: [0u8; 64],
                timestamp: 1000,
            }).unwrap();
        }

        let qc = engine.round_mut(1).unwrap().try_form_qc(4);
        assert!(qc.is_none(), "Should not form QC with only 2/4 votes");
    }

    #[test]
    fn n68_duplicate_vote_rejected() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);
        engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

        let vote = ConsensusVote {
            voter_id: [1u8; 32],
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote.clone()).is_ok());
        assert!(engine.process_vote(vote).is_err(), "Duplicate vote must be rejected");
    }

    #[test]
    fn n68_byzantine_wrong_height_rejected() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);
        engine.start_round(1, [1u8; 32]);

        let vote = ConsensusVote {
            voter_id: [2u8; 32],
            height: 99, // Wrong height
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        assert!(engine.process_vote(vote).is_err());
    }

    #[test]
    fn n68_multi_round_consensus() {
        let mut engine = ConsensusEngine::new([0u8; 32], 4);

        for height in 1..=3 {
            let proposer_idx = engine.proposer_for(height);
            let proposer = [(proposer_idx + 1) as u8; 32];
            engine.start_round(height, proposer);
            engine.round_mut(height).unwrap().propose([height as u8; 32], [0xBB; 32]);

            for id in 1..=3 {
                engine.process_vote(ConsensusVote {
                    voter_id: [id; 32],
                    height,
                    block_hash: [height as u8; 32],
                    state_root: [0xBB; 32],
                    approve: true,
                    signature: [0u8; 64],
                    timestamp: 1000,
                }).unwrap();
            }

            let history = [height as u8; 32];
            let cert = engine.try_advance(height, history).unwrap();
            assert_eq!(cert.height, height);
            assert!(engine.is_finalized(height));
        }

        assert_eq!(engine.current_height, 3);
        assert_eq!(engine.history_root, [3u8; 32]);
    }
}
