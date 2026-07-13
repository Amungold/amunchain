use std::collections::BTreeMap;
use crate::ids::{PeerId, PublicKey, ValidatorId};
use crate::record::ValidatorRecord;

#[derive(Debug, Clone)]
pub struct ValidatorRegistry {
    keys: BTreeMap<u64, [u8; 32]>,
    records: BTreeMap<ValidatorId, ValidatorRecord>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            keys: BTreeMap::new(),
            records: BTreeMap::new(),
        }
    }

    // === LEGACY API (backward-compatible) ===

    pub fn register(
        &mut self,
        validator_id: u64,
        public_key: [u8; 32],
    ) -> Result<(), &'static str> {
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

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    // === N133 CANONICAL API ===

    pub fn register_full(&mut self, record: ValidatorRecord) -> Result<(), &'static str> {
        if self.records.contains_key(&record.validator_id) {
            return Err("validator already registered in canonical registry");
        }
        let short_id = u64::from_le_bytes(record.validator_id.0[..8].try_into().unwrap());
        self.keys.insert(short_id, record.public_key.0);
        self.records.insert(record.validator_id, record);
        Ok(())
    }

    pub fn get_record(&self, id: &ValidatorId) -> Option<&ValidatorRecord> {
        self.records.get(id)
    }

    pub fn get_voting_power(&self, id: &ValidatorId) -> u64 {
        self.records.get(id).map(|r| r.voting_power).unwrap_or(0)
    }

    pub fn is_active_validator(&self, id: &ValidatorId) -> bool {
        self.records.get(id).map(|r| r.active).unwrap_or(false)
    }

    pub fn total_voting_power(&self) -> u64 {
        self.records.values().map(|r| r.voting_power).sum()
    }

    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// N133: implement unified trait
use crate::traits::ValidatorRegistryTrait;

impl ValidatorRegistryTrait for ValidatorRegistry {
    fn get(&self, id: &ValidatorId) -> Option<&ValidatorRecord> {
        self.get_record(id)
    }

    fn get_public_key(&self, id: &ValidatorId) -> Option<[u8; 32]> {
        self.records.get(id).map(|r| r.public_key.0)
    }

    fn get_voting_power(&self, id: &ValidatorId) -> u64 {
        self.get_voting_power(id)
    }

    fn is_active(&self, id: &ValidatorId) -> bool {
        self.is_active_validator(id)
    }

    fn total_voting_power(&self) -> u64 {
        self.total_voting_power()
    }

    fn len(&self) -> usize {
        self.record_count()
    }
}
