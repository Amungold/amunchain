use std::collections::BTreeMap;

/// Validator key registry: maps validator_id -> public_key.
/// Used by the signature verifier to look up keys.
#[derive(Debug, Clone)]
pub struct ValidatorRegistry {
    keys: BTreeMap<u64, [u8; 32]>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self { keys: BTreeMap::new() }
    }

    pub fn register(&mut self, validator_id: u64, public_key: [u8; 32]) -> Result<(), &'static str> {
        if self.keys.contains_key(&validator_id) {
            return Err("validator already registered");
        }
        self.keys.insert(validator_id, public_key);
        Ok(())
    }

    pub fn remove(&mut self, validator_id: u64) -> Option<[u8; 32]> {
        self.keys.remove(&validator_id)
    }

    pub fn get(&self, validator_id: u64) -> Option<&[u8; 32]> {
        self.keys.get(&validator_id)
    }

    pub fn contains(&self, validator_id: u64) -> bool {
        self.keys.contains_key(&validator_id)
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self { Self::new() }
}
