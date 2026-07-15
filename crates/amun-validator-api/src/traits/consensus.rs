use crate::error::PlatformResult;
use crate::traits::read::ValidatorRead;

pub trait ConsensusProvider: Send + Sync {
    fn is_active(&self) -> PlatformResult<bool>;
    fn current_height(&self) -> PlatformResult<u64>;
    fn is_leader(&self) -> PlatformResult<bool>;
    fn validator_registry(&self) -> &dyn ValidatorRead;
}
