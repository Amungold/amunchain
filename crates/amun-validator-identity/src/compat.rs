use crate::identity_service::IdentityService;
use crate::IdentityProvider;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════
// COMPATIBILITY LAYER — Pure Wrapper (Zero Business Logic)
// ═══════════════════════════════════════════════════════════════
// Every function below delegates to signature.rs (the SSOT).
// This layer exists ONLY so legacy crates continue to compile.
// When all crates migrate to the new API, this file can be deleted.
// ═══════════════════════════════════════════════════════════════

/// Legacy Ed25519 verification → delegates to crate::sig::verify_ed25519
pub fn verify_ed25519(pk: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    crate::sig::verify_ed25519(pk, msg, sig)
}

/// Legacy vote signing payload → delegates to crate::sig::build_vote_signing_payload
pub fn vote_signing_payload(
    voter_id: &[u8; 32],
    chain_id: u64,
    height: u64,
    round: u64,
    block_hash: &[u8; 32],
) -> Vec<u8> {
    crate::sig::build_vote_signing_payload(voter_id, chain_id, height, round, block_hash)
}

/// Legacy validator ID derivation → delegates to crate::sig::derive_validator_id
pub fn derive_validator_id(public_key: &[u8; 32]) -> [u8; 32] {
    crate::sig::derive_validator_id(public_key)
}

/// Legacy signature constants → delegates to crate::sig::DEFAULT_CHAIN_ID
pub mod sig_constants {
    pub const DEFAULT_CHAIN_ID: u64 = crate::sig::DEFAULT_CHAIN_ID;
}

/// Legacy validator key registry — wraps IdentityService.
/// Does NOT hold its own state; delegates everything to IdentityService.
pub struct ValidatorKeyRegistry {
    identity: Option<Arc<IdentityService>>,
}

impl Default for ValidatorKeyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorKeyRegistry {
    pub fn new() -> Self {
        ValidatorKeyRegistry { identity: None }
    }

    pub fn with_identity(identity: Arc<IdentityService>) -> Self {
        ValidatorKeyRegistry {
            identity: Some(identity),
        }
    }

    /// Delegates to IdentityService → SSOT for public keys.
    pub fn get(&self, _validator_id: &[u8; 32]) -> Option<[u8; 32]> {
        self.identity.as_ref().map(|id| *id.public_key().as_bytes())
    }

    /// No-op: IdentityService owns the truth.
    pub fn insert(&mut self, _validator_id: [u8; 32], _public_key: [u8; 32]) {}

    /// No-op: IdentityService owns the truth.
    pub fn register_identity(
        &mut self,
        _peer_id: [u8; 32],
        _validator_id: [u8; 32],
        _public_key: [u8; 32],
    ) {
    }

    pub fn is_empty(&self) -> bool {
        self.identity.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compat_verify_delegates() {
        let mut sig = [0u8; 64];
        sig[0] = 0x42;
        assert!(!verify_ed25519(&[0x42u8; 32], b"msg", &sig));
    }

    #[test]
    fn test_compat_vote_payload_delegates() {
        let payload = vote_signing_payload(&[1u8; 32], 1, 100, 2, &[3u8; 32]);
        assert_eq!(payload.len(), 88);
    }

    #[test]
    fn test_compat_derive_id_delegates() {
        let pk = [0xFFu8; 32];
        let id = derive_validator_id(&pk);
        assert_eq!(id, pk);
    }

    #[test]
    fn test_compat_registry_delegates() {
        let reg = ValidatorKeyRegistry::new();
        assert!(reg.is_empty());
    }
}
