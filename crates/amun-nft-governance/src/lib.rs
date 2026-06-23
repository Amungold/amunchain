use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Governance right attached to an NFT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRight {
    pub token_id: [u8; 32],
    pub owner: [u8; 32],
    pub can_propose: bool,
    pub can_veto: bool,
    pub voting_power: u64,
}

/// Ledger tracking governance rights by token
#[derive(Debug, Clone, Default)]
pub struct GovernanceLedger {
    pub rights: BTreeMap<[u8; 32], GovernanceRight>,
}

impl GovernanceLedger {
    pub fn new() -> Self {
        Self {
            rights: BTreeMap::new(),
        }
    }

    /// Grant or update governance rights for a token
    pub fn set_rights(&mut self, right: GovernanceRight) {
        self.rights.insert(right.token_id, right);
    }

    /// Revoke rights for a token
    pub fn revoke_rights(&mut self, token_id: &[u8; 32]) {
        self.rights.remove(token_id);
    }

    /// Check if a token holder can propose
    pub fn can_propose(&self, token_id: &[u8; 32], owner: &[u8; 32]) -> bool {
        self.rights
            .get(token_id)
            .map(|r| r.owner == *owner && r.can_propose)
            .unwrap_or(false)
    }

    /// Check if a token holder can veto
    pub fn can_veto(&self, token_id: &[u8; 32], owner: &[u8; 32]) -> bool {
        self.rights
            .get(token_id)
            .map(|r| r.owner == *owner && r.can_veto)
            .unwrap_or(false)
    }

    /// Get voting power of a token
    pub fn voting_power(&self, token_id: &[u8; 32], owner: &[u8; 32]) -> u64 {
        self.rights
            .get(token_id)
            .filter(|r| r.owner == *owner)
            .map(|r| r.voting_power)
            .unwrap_or(0)
    }

    /// Compute deterministic governance root
    pub fn compute_governance_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_GOVERNANCE_RIGHTS_V1");
        for (id, right) in &self.rights {
            hasher.update(id);
            hasher.update(right.owner);
            hasher.update([right.can_propose as u8, right.can_veto as u8]);
            hasher.update(right.voting_power.to_le_bytes());
        }
        hasher.finalize().into()
    }
}
