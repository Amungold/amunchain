use amun_orchestrator_core::types::{PublicKey, ValidatorId};
use serde::{Deserialize, Serialize};

/// The genesis block configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Genesis {
    pub chain_id: String,
    pub genesis_time: String,
    pub initial_protocol_version: u32,
    pub validators: Vec<GenesisValidator>,
    pub trust_anchors: Vec<TrustAnchor>,
    pub genesis_hash: String,
}

/// A validator entry in genesis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenesisValidator {
    pub validator_id: ValidatorId,
    pub public_key: PublicKey,
    pub voting_power: u64,
    pub name: String,
}

/// A trust anchor (root certificate authority).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustAnchor {
    pub id: String,
    pub certificate: String,
}

impl Default for Genesis {
    fn default() -> Self {
        Self {
            chain_id: String::new(),
            genesis_time: chrono::Utc::now().to_rfc3339(),
            initial_protocol_version: 1,
            validators: Vec::new(),
            trust_anchors: Vec::new(),
            genesis_hash: String::new(),
        }
    }
}

impl Genesis {
    /// Total voting power of all validators.
    pub fn total_voting_power(&self) -> u64 {
        self.validators.iter().map(|v| v.voting_power).sum()
    }

    /// Number of validators.
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
}
