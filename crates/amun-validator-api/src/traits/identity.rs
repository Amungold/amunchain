use crate::error::PlatformResult;
use crate::types::id::ValidatorId;

pub trait IdentityProvider: Send + Sync {
    fn validator_id(&self) -> &ValidatorId;
    fn public_key(&self) -> &[u8; 32];
    fn certificate_hash(&self) -> &[u8; 32];
    fn verify(&self) -> PlatformResult<()>;
    fn rotate_keys(&self) -> PlatformResult<()>;
}
