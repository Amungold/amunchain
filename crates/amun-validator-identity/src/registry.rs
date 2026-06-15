use crate::validator_id::ValidatorId;
use std::collections::BTreeMap;

/// Maps ValidatorId → Ed25519 public key, and optionally PeerId → ValidatorId.
#[derive(Debug, Clone, Default)]
pub struct ValidatorKeyRegistry {
    keys: BTreeMap<ValidatorId, [u8; 32]>,
    peer_to_validator: BTreeMap<[u8; 32], ValidatorId>, // PeerId → ValidatorId
}

impl ValidatorKeyRegistry {
    pub fn new() -> Self {
        Self {
            keys: BTreeMap::new(),
            peer_to_validator: BTreeMap::new(),
        }
    }

    /// Insert a validator with only its ValidatorId (legacy, no PeerId).
    pub fn insert(&mut self, id: ValidatorId, public_key: [u8; 32]) {
        self.keys.insert(id, public_key);
    }

    /// Register a validator with both PeerId and ValidatorId.
    pub fn register_identity(
        &mut self,
        peer_id: [u8; 32],
        validator_id: ValidatorId,
        public_key: [u8; 32],
    ) {
        self.keys.insert(validator_id, public_key);
        self.peer_to_validator.insert(peer_id, validator_id);
    }

    pub fn get(&self, id: &ValidatorId) -> Option<&[u8; 32]> {
        self.keys.get(id)
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Resolve a PeerId to the corresponding ValidatorId (if registered).
    pub fn get_validator_id_from_peer(&self, peer_id: &[u8; 32]) -> Option<&ValidatorId> {
        self.peer_to_validator.get(peer_id)
    }
}
