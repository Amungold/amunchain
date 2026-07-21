use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::validator::GenesisValidator;

/// A trust anchor defined in the genesis configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisTrustAnchor {
    pub peer_id: String,
    pub public_key: String,
}

/// The constitutional genesis document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genesis {
    pub chain_id: String,
    pub timestamp: u64,
    pub validators: Vec<GenesisValidator>,
    pub trust_anchors: Vec<GenesisTrustAnchor>,
}

impl Genesis {
    /// Compute a deterministic Blake3 hash.
    pub fn genesis_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_GENESIS_V1");
        hasher.update(self.chain_id.as_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        for v in &self.validators {
            hasher.update(&v.public_key);
            hasher.update(&v.stake.to_le_bytes());
        }
        for t in &self.trust_anchors {
            hasher.update(t.peer_id.as_bytes());
            hasher.update(t.public_key.as_bytes());
        }
        hasher.finalize().into()
    }

    /// Validate the genesis document.
    pub fn validate(&self) -> Result<(), String> {
        if self.chain_id.is_empty() { return Err("Chain ID must not be empty".into()); }
        if self.validators.is_empty() { return Err("Validator set must not be empty".into()); }
        let mut seen_pubkeys = HashSet::new();
        for (i, v) in self.validators.iter().enumerate() {
            if v.public_key == [0u8; 32] {
                return Err(format!("Validator {} has empty public key", i));
            }
            if !seen_pubkeys.insert(v.public_key) {
                return Err(format!("Duplicate validator public key: {:?}", &v.public_key[..4]));
            }
        }
        for (i, v) in self.validators.iter().enumerate() {
            if v.stake == 0 {
                return Err(format!("Validator {} has zero stake", i));
            }
        }
        Ok(())
    }
}

/// Load genesis from a JSON file.
pub fn load_from_file(path: &std::path::Path) -> Result<Genesis, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read genesis: {}", e))?;
    let genesis: Genesis = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid genesis JSON: {}", e))?;
    genesis.validate()?;
    Ok(genesis)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_genesis() -> Genesis {
        Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator::new([1u8; 32], 100)],
            trust_anchors: vec![],
        }
    }

    #[test] fn test_valid() { assert!(valid_genesis().validate().is_ok()); }
    #[test] fn test_hash_deterministic() { let g = valid_genesis(); assert_eq!(g.genesis_hash(), g.genesis_hash()); }
}
