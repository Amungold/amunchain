use crate::error::PlatformResult;
use crate::types::id::ValidatorId;
use crate::types::record::ValidatorRecord;

pub trait ValidatorAdmin: Send + Sync {
    fn register(&self, record: ValidatorRecord) -> PlatformResult<()>;
    fn update_voting_power(&self, id: &ValidatorId, power: u64) -> PlatformResult<()>;
    fn activate(&self, id: &ValidatorId) -> PlatformResult<()>;
    fn deactivate(&self, id: &ValidatorId) -> PlatformResult<()>;
    fn remove(&self, id: &ValidatorId) -> PlatformResult<()>;
}
