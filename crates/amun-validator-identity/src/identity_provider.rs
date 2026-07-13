use crate::derive_validator_id;
use crate::registry::IdentityRegistry;
use ed25519_dalek::SigningKey;
use std::collections::HashMap;

/// Constitutional Identity Provider (AC-1.0 Article VII)
pub struct IdentityProvider {
    pub validator_id: [u8; 32],
    pub signing_key: SigningKey,
    pub public_key: [u8; 32],
    peer_keys: HashMap<[u8; 32], [u8; 32]>,
    voting_power: HashMap<[u8; 32], u64>,
}

impl IdentityProvider {
    #[inline]
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        let public_key = signing_key.verifying_key().to_bytes();
        let validator_id = derive_validator_id(&public_key);
        Self {
            validator_id,
            signing_key,
            public_key,
            peer_keys: HashMap::new(),
            voting_power: HashMap::new(),
        }
    }

    #[inline]
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        use ed25519_dalek::Signer;
        self.signing_key.sign(message).to_bytes()
    }

    /// Issue a validator certificate (bootstrap only, never on hot path).
    pub fn issue_certificate(
        &self,
        authority_version: u64,
        authority_id: amun_identity_core::AuthorityId,
    ) -> amun_identity_core::ValidatorCertificate {
        amun_identity_core::ValidatorCertificate {
            peer_id: amun_identity_core::PeerId::from_bytes(self.public_key),
            validator_id: self.validator_id,
            public_key: self.public_key,
            authority_version,
            authority_id,
            valid_from: 0,
            valid_until: u64::MAX,
        }
    }

    /// Build a read-only registry from identity data.
    /// The registry is a runtime cache — never authoritative.
    pub fn build_registry(&self) -> IdentityRegistry {
        let mut registry = IdentityRegistry::new();
        registry.insert(self.validator_id, self.public_key, 100);
        for (id, pk) in &self.peer_keys {
            let vp = self.voting_power.get(id).copied().unwrap_or(0);
            registry.insert(*id, *pk, vp);
        }
        registry
    }

    /// Register a peer during bootstrap.
    pub fn register_peer(
        &mut self,
        validator_id: [u8; 32],
        public_key: [u8; 32],
        voting_power: u64,
    ) {
        self.peer_keys.insert(validator_id, public_key);
        self.voting_power.insert(validator_id, voting_power);
    }

    /// Get peer count
    pub fn peer_count(&self) -> usize {
        self.peer_keys.len()
    }
}
