use crate::verification::types::SupplyBreakdown;
use crate::verification::error::FormalError;

pub type ChainId = u32;
pub type EpochId = u64;
pub type StateRoot = [u8; 32];

pub trait StateReader {
    fn supply_breakdown(&self) -> Result<SupplyBreakdown, FormalError>;
    fn state_root(&self) -> Result<StateRoot, FormalError>;
    fn block_height(&self) -> u64;
    fn chain_id(&self) -> ChainId;
    fn epoch(&self) -> EpochId;
}
