use crate::error::PlatformResult;
use crate::types::id::ValidatorId;

pub trait ValidatorRead: Send + Sync {
    fn get_public_key(&self, id: &ValidatorId) -> PlatformResult<Option<[u8; 32]>>;
    fn get_voting_power(&self, id: &ValidatorId) -> PlatformResult<u64>;
    fn is_active(&self, id: &ValidatorId) -> PlatformResult<bool>;
    fn total_voting_power(&self) -> PlatformResult<u64>;
    fn validator_count(&self) -> PlatformResult<usize>;
}
