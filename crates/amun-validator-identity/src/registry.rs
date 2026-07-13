use crate::validator_id::ValidatorId;
use std::collections::HashMap;

/// Validator key registry: maps validator_id -> public_key
#[derive(Clone)]
pub struct ValidatorKeyRegistry {
    keys: HashMap<ValidatorId, [u8; 32]>,
    peer_to_validator: HashMap<[u8; 32], ValidatorId>,
}

impl Default for ValidatorKeyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorKeyRegistry {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            peer_to_validator: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

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

    pub fn insert(&mut self, id: ValidatorId, pk: [u8; 32]) {
        self.keys.insert(id, pk);
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.peer_to_validator.clear();
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn get_validator_id_from_peer(&self, peer_id: &[u8; 32]) -> Option<&ValidatorId> {
        self.peer_to_validator.get(peer_id)
    }
}

// =====================================================
// IdentityRegistry — Constitutional Runtime Cache (AC-1.0)
// =====================================================

pub struct IdentityRegistry {
    keys: HashMap<[u8; 32], [u8; 32]>,
    power: HashMap<[u8; 32], u64>,
}

impl Default for IdentityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityRegistry {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            power: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: [u8; 32], pk: [u8; 32], vp: u64) {
        self.keys.insert(id, pk);
        self.power.insert(id, vp);
    }

    pub fn get_key(&self, id: &[u8; 32]) -> Option<&[u8; 32]> {
        self.keys.get(id)
    }
    pub fn get_power(&self, id: &[u8; 32]) -> u64 {
        self.power.get(id).copied().unwrap_or(0)
    }
    pub fn total_power(&self) -> u64 {
        self.power.values().sum()
    }
    pub fn contains(&self, id: &[u8; 32]) -> bool {
        self.keys.contains_key(id)
    }
    pub fn ids(&self) -> Vec<[u8; 32]> {
        self.keys.keys().copied().collect()
    }

    pub fn to_engine_format(&self) -> HashMap<[u8; 32], ([u8; 32], u64)> {
        let mut map = HashMap::new();
        for id in self.ids() {
            if let Some(pk) = self.get_key(&id) {
                map.insert(id, (*pk, self.get_power(&id)));
            }
        }
        map
    }
}
