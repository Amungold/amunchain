use amun_consensus_messages::{ConsensusVote, ConsensusPhase};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EquivocationEvidence {
    pub vote_a: ConsensusVote,
    pub vote_b: ConsensusVote,
}

impl EquivocationEvidence {
    pub fn new(vote_a: ConsensusVote, vote_b: ConsensusVote) -> Self {
        let (first, second) = if vote_a.signature <= vote_b.signature {
            (vote_a, vote_b)
        } else {
            (vote_b, vote_a)
        };
        Self { vote_a: first, vote_b: second }
    }

    pub fn verify(&self) -> bool {
        let a = &self.vote_a.message;
        let b = &self.vote_b.message;
        a.validator_id == b.validator_id
            && a.round == b.round
            && a.phase == b.phase
            && a.block_hash != b.block_hash
    }

    pub fn validator_id(&self) -> u64 {
        self.vote_a.message.validator_id
    }

    pub fn round(&self) -> u64 {
        self.vote_a.message.round
    }
}

pub fn detect_equivocation(votes: &[ConsensusVote]) -> Vec<EquivocationEvidence> {
    let mut seen: HashMap<(u64, u64, ConsensusPhase), &ConsensusVote> = HashMap::new();
    let mut evidence = Vec::new();

    for vote in votes {
        let key = (vote.message.validator_id, vote.message.round, vote.message.phase);
        if let Some(&existing) = seen.get(&key) {
            if existing.message.block_hash != vote.message.block_hash {
                evidence.push(EquivocationEvidence::new(existing.clone(), vote.clone()));
            }
        } else {
            seen.insert(key, vote);
        }
    }
    evidence
}
