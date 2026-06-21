use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

pub struct DefiParameters {
    pub amm_fee_bps: u64,
    pub lending_interest_base_bps: u64,
    pub collateral_ratio_min: u64,
    pub liquidation_threshold: u64,
    pub stablecoin_mint_ratio: u64,
}

impl Default for DefiParameters {
    fn default() -> Self {
        Self {
            amm_fee_bps: 30,
            lending_interest_base_bps: 500,
            collateral_ratio_min: 150,
            liquidation_threshold: 8000,
            stablecoin_mint_ratio: 66,
        }
    }
}

pub struct GovernanceProposal {
    pub proposal_id: [u8; 32],
    pub proposer: [u8; 32],
    pub parameter: String,
    pub new_value: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
}

pub struct GovernanceEngine {
    pub parameters: DefiParameters,
    pub proposals: BTreeMap<[u8; 32], GovernanceProposal>,
    pub next_proposal_id: u64,
}

impl Default for GovernanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GovernanceEngine {
    pub fn new() -> Self {
        Self {
            parameters: DefiParameters::default(),
            proposals: BTreeMap::new(),
            next_proposal_id: 0,
        }
    }

    pub fn propose(
        &mut self,
        proposer: [u8; 32],
        parameter: String,
        new_value: u64,
    ) -> [u8; 32] {
        self.next_proposal_id += 1;
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_GOVERNANCE_PROPOSAL_V1");
        hasher.update(proposer);
        hasher.update(self.next_proposal_id.to_le_bytes());
        let proposal_id: [u8; 32] = hasher.finalize().into();
        self.proposals.insert(proposal_id, GovernanceProposal {
            proposal_id,
            proposer,
            parameter,
            new_value,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        });
        proposal_id
    }

    pub fn vote(&mut self, proposal_id: &[u8; 32], _voter: [u8; 32], support: bool, weight: u64) -> bool {
        if let Some(p) = self.proposals.get_mut(proposal_id) {
            if support { p.votes_for += weight; }
            else { p.votes_against += weight; }
            return true;
        }
        false
    }

    pub fn execute(&mut self, proposal_id: &[u8; 32]) -> bool {
        if let Some(p) = self.proposals.get(proposal_id) {
            if p.executed || p.votes_for <= p.votes_against {
                return false;
            }
            match p.parameter.as_str() {
                "amm_fee_bps" => self.parameters.amm_fee_bps = p.new_value,
                "lending_interest_base_bps" => self.parameters.lending_interest_base_bps = p.new_value,
                "collateral_ratio_min" => self.parameters.collateral_ratio_min = p.new_value,
                "liquidation_threshold" => self.parameters.liquidation_threshold = p.new_value,
                "stablecoin_mint_ratio" => self.parameters.stablecoin_mint_ratio = p.new_value,
                _ => return false,
            }
            return true;
        }
        false
    }

    pub fn compute_governance_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_DEFI_GOVERNANCE_V1");
        hasher.update(self.parameters.amm_fee_bps.to_le_bytes());
        hasher.update(self.parameters.lending_interest_base_bps.to_le_bytes());
        hasher.update(self.parameters.collateral_ratio_min.to_le_bytes());
        hasher.update(self.parameters.liquidation_threshold.to_le_bytes());
        hasher.update(self.parameters.stablecoin_mint_ratio.to_le_bytes());
        for (id, proposal) in &self.proposals {
            hasher.update(id);
            hasher.update(proposal.votes_for.to_le_bytes());
            hasher.update(proposal.votes_against.to_le_bytes());
            hasher.update([proposal.executed as u8]);
        }
        hasher.finalize().into()
    }
}
