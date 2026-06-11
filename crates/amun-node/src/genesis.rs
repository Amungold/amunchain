use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A validator defined in the genesis configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisValidator {
    pub peer_id: String,
    pub public_key: String,
    pub voting_power: u64,
}

/// A trust anchor defined in the genesis configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisTrustAnchor {
    pub peer_id: String,
    pub public_key: String,
}

/// The constitutional genesis document establishing the initial network state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genesis {
    pub chain_id: String,
    pub timestamp: u64,
    pub validators: Vec<GenesisValidator>,
    pub trust_anchors: Vec<GenesisTrustAnchor>,
}

impl Genesis {
    /// Compute a deterministic Blake3 hash of the genesis document.
    /// All nodes must produce the same hash for the same genesis.
    pub fn genesis_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_GENESIS_V1");
        hasher.update(self.chain_id.as_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        for v in &self.validators {
            hasher.update(v.peer_id.as_bytes());
            hasher.update(v.public_key.as_bytes());
            hasher.update(&v.voting_power.to_le_bytes());
        }
        for t in &self.trust_anchors {
            hasher.update(t.peer_id.as_bytes());
            hasher.update(t.public_key.as_bytes());
        }
        hasher.finalize().into()
    }

    /// Validate the genesis document against constitutional rules.
    pub fn validate(&self) -> Result<(), String> {
        // Chain ID must not be empty
        if self.chain_id.is_empty() {
            return Err("Chain ID must not be empty".into());
        }

        // Validator set must not be empty
        if self.validators.is_empty() {
            return Err("Validator set must not be empty".into());
        }

        // Check for duplicate validator peer IDs
        let mut seen_peer_ids = HashSet::new();
        for (i, v) in self.validators.iter().enumerate() {
            if v.peer_id.is_empty() {
                return Err(format!("Validator {} has empty peer ID", i));
            }
            if !seen_peer_ids.insert(&v.peer_id) {
                return Err(format!("Duplicate validator peer ID: {}", v.peer_id));
            }
        }

        // Check for duplicate validator public keys
        let mut seen_pubkeys = HashSet::new();
        for (i, v) in self.validators.iter().enumerate() {
            if v.public_key.is_empty() {
                return Err(format!("Validator {} has empty public key", i));
            }
            if !seen_pubkeys.insert(&v.public_key) {
                return Err(format!("Duplicate validator public key: {}", v.public_key));
            }
        }

        // Voting power must be positive
        for (i, v) in self.validators.iter().enumerate() {
            if v.voting_power == 0 {
                return Err(format!("Validator {} has zero voting power", i));
            }
        }

        // Check trust anchors
        let mut seen_anchor_ids = HashSet::new();
        for (i, t) in self.trust_anchors.iter().enumerate() {
            if t.peer_id.is_empty() {
                return Err(format!("Trust anchor {} has empty peer ID", i));
            }
            if t.public_key.is_empty() {
                return Err(format!("Trust anchor {} has empty public key", i));
            }
            if !seen_anchor_ids.insert(&t.peer_id) {
                return Err(format!("Duplicate trust anchor peer ID: {}", t.peer_id));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_genesis() -> Genesis {
        Genesis {
            chain_id: "amunchain-testnet-1".into(),
            timestamp: 0,
            validators: vec![
                GenesisValidator {
                    peer_id: "v1".into(),
                    public_key: "pk1".into(),
                    voting_power: 100,
                },
                GenesisValidator {
                    peer_id: "v2".into(),
                    public_key: "pk2".into(),
                    voting_power: 100,
                },
            ],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "ta1".into(),
                public_key: "tapk1".into(),
            }],
        }
    }

    #[test]
    fn n22_3_valid_genesis_accepted() {
        let genesis = valid_genesis();
        assert!(genesis.validate().is_ok());
    }

    #[test]
    fn n22_3_duplicate_validator_rejected() {
        let mut genesis = valid_genesis();
        genesis.validators[1].peer_id = genesis.validators[0].peer_id.clone();
        assert!(genesis.validate().is_err());
    }

    #[test]
    fn n22_3_empty_validator_set_rejected() {
        let mut genesis = valid_genesis();
        genesis.validators.clear();
        assert!(genesis.validate().is_err());
    }

    #[test]
    fn n22_3_duplicate_trust_anchor_rejected() {
        let mut genesis = valid_genesis();
        genesis.trust_anchors.push(GenesisTrustAnchor {
            peer_id: "ta1".into(),
            public_key: "tapk2".into(),
        });
        assert!(genesis.validate().is_err());
    }

    #[test]
    fn n22_3_genesis_hash_deterministic() {
        let genesis = valid_genesis();
        let h1 = genesis.genesis_hash();
        let h2 = genesis.genesis_hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn n22_3_different_validator_set_changes_hash() {
        let g1 = valid_genesis();
        let mut g2 = valid_genesis();
        g2.validators[0].voting_power = 200;
        assert_ne!(g1.genesis_hash(), g2.genesis_hash());
    }

    #[test]
    fn n22_3_empty_chain_id_rejected() {
        let mut genesis = valid_genesis();
        genesis.chain_id = String::new();
        assert!(genesis.validate().is_err());
    }

    #[test]
    fn n22_3_zero_voting_power_rejected() {
        let mut genesis = valid_genesis();
        genesis.validators[0].voting_power = 0;
        assert!(genesis.validate().is_err());
    }
}
