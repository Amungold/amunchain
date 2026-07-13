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
