pub mod authority_store;
pub mod certificate_store;
pub mod compat;
pub mod identity_loader;
pub mod identity_service;
pub mod key_store;
pub mod sig;

pub use authority_store::{AuthorityStore, TrustAnchor};
pub use certificate_store::{CertificateData, CertificateStore};
pub use compat::derive_validator_id;
pub use compat::verify_ed25519;
pub use compat::vote_signing_payload;
pub use compat::ValidatorKeyRegistry;
pub use identity_loader::IdentityLoader;
pub use identity_service::IdentityService;
pub use key_store::KeyStore;
/// Legacy re-export: signature constants
pub mod signature {
    pub use crate::sig::DEFAULT_CHAIN_ID;
}

use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::{PublicKey, ValidatorId};

/// The identity provider trait — consumed by Bootstrap, Consensus, Networking, RPC.
/// There is exactly ONE implementation of this trait in the system.
/// It delegates to the stores; it does NOT hold its own copies.
pub trait IdentityProvider: Send + Sync {
    fn validator_id(&self) -> &ValidatorId;
    fn public_key(&self) -> &PublicKey;
    fn certificate_hash(&self) -> &[u8; 32];

    /// Perform a self-check to validate that the identity is ready.
    /// The implementation decides what constitutes a valid identity.
    /// Bootstrap and other consumers call this without knowing the rules.
    fn self_check(&self) -> PlatformResult<()>;

    /// Sign a message with the validator's private key.
    fn sign(&self, message: &[u8]) -> PlatformResult<Vec<u8>>;

    /// Verify a signature against the validator's public key.
    fn verify(&self, message: &[u8], signature: &[u8]) -> PlatformResult<()>;
}
