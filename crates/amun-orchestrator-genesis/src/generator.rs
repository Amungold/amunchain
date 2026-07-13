use super::types::{Genesis, GenesisValidator};
use amun_orchestrator_core::types::{PublicKey, ValidatorId};
use sha2::{Digest, Sha256};

/// Create a new genesis block with the given validators.
pub fn create_genesis(chain_id: &str, validators: &[(ValidatorId, PublicKey, u64)]) -> Genesis {
    let genesis_validators: Vec<GenesisValidator> = validators
        .iter()
        .map(|(id, pk, power)| GenesisValidator {
            validator_id: *id,
            public_key: *pk,
            voting_power: *power,
            name: format!("validator-{}", hex::encode(&id.0[..4])),
        })
        .collect();

    let mut genesis = Genesis {
        chain_id: chain_id.to_string(),
        genesis_time: chrono::Utc::now().to_rfc3339(),
        initial_protocol_version: 1,
        validators: genesis_validators,
        trust_anchors: Vec::new(),
        genesis_hash: String::new(),
    };

    recompute_hash(&mut genesis);
    genesis
}

/// Recompute genesis_hash after modifications.
pub fn recompute_hash(genesis: &mut Genesis) {
    genesis.genesis_hash = String::new(); // Clear before hashing
    let json = serde_json::to_string(genesis).unwrap_or_default();
    let hash = Sha256::digest(json.as_bytes());
    genesis.genesis_hash = hex::encode(hash);
}
