use crate::ids::ValidatorId;
use crate::record::ValidatorRecord;

pub trait ValidatorRegistryTrait {
    fn get(&self, id: &ValidatorId) -> Option<&ValidatorRecord>;
    fn get_public_key(&self, id: &ValidatorId) -> Option<[u8; 32]>;
    fn get_voting_power(&self, id: &ValidatorId) -> u64;
    fn is_active(&self, id: &ValidatorId) -> bool;
    fn total_voting_power(&self) -> u64;
    fn len(&self) -> usize;
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
