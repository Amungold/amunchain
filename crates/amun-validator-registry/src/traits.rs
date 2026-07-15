use crate::ids::ValidatorId;
use crate::record::ValidatorRecord;

pub trait ValidatorRegistryTrait {
    fn get(&self, id: &ValidatorId) -> Option<&ValidatorRecord>;
    fn get_public_key(&self, id: &ValidatorId) -> Option<[u8; 32]>;
    fn get_voting_power(&self, id: &ValidatorId) -> u64;
    fn is_active(&self, id: &ValidatorId) -> bool;
    fn total_voting_power(&self) -> u64;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Read-only trait for validator identity queries.
/// Used by consensus to look up public keys, voting power, and active status
/// without depending on mutation APIs (slash, register, deactivate).
pub trait ValidatorRead {
    fn get_public_key(&self, id: &ValidatorId) -> Option<[u8; 32]>;
    fn get_voting_power(&self, id: &ValidatorId) -> u64;
    fn is_active(&self, id: &ValidatorId) -> bool;
    fn total_voting_power(&self) -> u64;
    fn validator_count(&self) -> usize;
}

/// Write interface for validator lifecycle management.
/// Used by bootstrapping, orchestration, and registration flows.
/// Kept separate from ValidatorRead to maintain read/write segregation.
pub trait ValidatorAdmin {
    fn register(&mut self, record: crate::record::ValidatorRecord) -> Result<(), &'static str>;
    fn update_voting_power(&mut self, id: &ValidatorId, power: u64) -> Result<(), &'static str>;
    fn activate(&mut self, id: &ValidatorId) -> Result<(), &'static str>;
    fn deactivate(&mut self, id: &ValidatorId) -> Result<(), &'static str>;
    fn remove(&mut self, id: &ValidatorId) -> Result<(), &'static str>;
}
