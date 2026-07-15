use crate::error::PlatformResult;

pub trait GenesisProvider: Send + Sync {
    fn genesis_hash(&self) -> PlatformResult<[u8; 32]>;
    fn chain_id(&self) -> PlatformResult<String>;
    fn verify(&self) -> PlatformResult<()>;
    fn is_compatible(&self, other_hash: &[u8; 32]) -> PlatformResult<bool>;
}
