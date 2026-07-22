use blake3::Hasher;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisValidator {
    pub public_key: [u8; 32],
    pub stake: u64,
    pub address: [u8; 20],
}

impl GenesisValidator {
    pub fn new(public_key: [u8; 32], stake: u64) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&public_key);
        let hash = hasher.finalize();
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash.as_bytes()[..20]);
        Self {
            public_key,
            stake,
            address,
        }
    }
}

pub fn compute_validator_set_hash(validators: &[GenesisValidator]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(b"AMUN_VALIDATOR_SET_V1");
    for v in validators {
        hasher.update(&v.public_key);
        hasher.update(&v.stake.to_le_bytes());
    }
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
    hash
}
