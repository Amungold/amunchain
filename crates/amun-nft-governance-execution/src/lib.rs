use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use amun_nft_governance::GovernanceLedger;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: [u8; 32],
    pub proposer: [u8; 32],
    pub description: String,
    pub created_height: u64,
    pub expires_height: u64,
    pub executed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: [u8; 32],
    pub voter: [u8; 32],
    pub support: bool,
    pub weight: u64,
}

#[derive(Debug, Clone, Default)]
pub struct GovernanceExecution {
    pub proposals: BTreeMap<[u8; 32], Proposal>,
    pub votes: Vec<Vote>,
}

impl GovernanceExecution {
    pub fn new() -> Self {
        Self { proposals: BTreeMap::new(), votes: Vec::new() }
    }

    pub fn propose(
        &mut self,
        ledger: &GovernanceLedger,
        proposer: &[u8; 32],
        description: String,
        created_height: u64,
        expires_height: u64,
    ) -> Option<[u8; 32]> {
        let has_rights = ledger.rights.values().any(|r| r.owner == *proposer && r.can_propose);
        if !has_rights {
            return None;
        }
        let mut hasher = Sha256::new();
        hasher.update(b"GOVERNANCE_PROPOSAL");
        hasher.update(proposer);
        hasher.update(description.as_bytes());
        hasher.update(created_height.to_le_bytes());
        let proposal_id: [u8; 32] = hasher.finalize().into();
        self.proposals.insert(proposal_id, Proposal {
            proposal_id,
            proposer: *proposer,
            description,
            created_height,
            expires_height,
            executed: false,
        });
        Some(proposal_id)
    }

    pub fn vote(
        &mut self,
        ledger: &GovernanceLedger,
        proposal_id: &[u8; 32],
        voter: &[u8; 32],
        support: bool,
    ) -> bool {
        if !self.proposals.contains_key(proposal_id) {
            return false;
        }
        let weight = ledger.rights.values()
            .filter(|r| r.owner == *voter)
            .map(|r| r.voting_power)
            .sum();
        if weight == 0 {
            return false;
        }
        self.votes.push(Vote {
            proposal_id: *proposal_id,
            voter: *voter,
            support,
            weight,
        });
        true
    }

    pub fn tally(&self, proposal_id: &[u8; 32]) -> (u64, u64) {
        let (mut support, mut against) = (0u64, 0u64);
        for vote in &self.votes {
            if vote.proposal_id == *proposal_id {
                if vote.support { support += vote.weight; }
                else { against += vote.weight; }
            }
        }
        (support, against)
    }

    pub fn execute(&mut self, proposal_id: &[u8; 32], current_height: u64) -> bool {
        let (support, against) = self.tally(proposal_id);
        if support <= against {
            return false;
        }
        if let Some(p) = self.proposals.get_mut(proposal_id) {
            if !p.executed && current_height <= p.expires_height {
                p.executed = true;
                return true;
            }
        }
        false
    }

    pub fn compute_execution_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_GOVERNANCE_EXECUTION_V1");
        for (id, proposal) in &self.proposals {
            hasher.update(id);
            let bytes = serde_json::to_vec(proposal).unwrap();
            hasher.update(&bytes);
        }
        for vote in &self.votes {
            let bytes = serde_json::to_vec(vote).unwrap();
            hasher.update(&bytes);
        }
        hasher.finalize().into()
    }
}
