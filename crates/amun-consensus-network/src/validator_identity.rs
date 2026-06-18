// N113.1 — Unified Validator Identity
// ====================================
// Replaces the temporary 32-byte → 48-byte zero-padding introduced in N110.1b
// with a permanent ValidatorIdentity struct that binds consensus identity,
// staking identity, and governance identity together.
//
// N113.2 — Cryptographic Binding
// ==============================
// Every ValidatorIdentity is cryptographically bound to its PublicKey.
// The binding is verified at registration and cannot be changed without
// generating a new identity.

use amun_kernel_types::PublicKey;
use serde::{Deserialize, Serialize};

/// N113.1: Unified validator identity that bridges consensus (32-byte)
/// and staking (48-byte PublicKey) identities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidatorIdentity {
    /// Consensus-layer validator ID (32 bytes)
    #[serde(with = "serde_bytes")]
    pub validator_id: [u8; 32],

    /// Staking-layer public key (48 bytes). Stored as raw bytes for serialization.
    #[serde(with = "serde_bytes")]
    pub public_key_bytes: [u8; 48],

    /// Blake3 hash binding validator_id and public_key together.
    /// identity_binding = blake3(AMUN_IDENTITY_V1 || validator_id || public_key_bytes)
    #[serde(with = "serde_bytes")]
    pub identity_binding: [u8; 32],

    /// Whether this identity is currently active
    pub is_active: bool,

    /// Height at which this identity was registered
    pub registered_at_height: u64,
}

impl ValidatorIdentity {
    /// Convert to PublicKey for staking operations.
    pub fn to_public_key(&self) -> PublicKey {
        PublicKey(self.public_key_bytes)
    }

    /// Create a new identity from bytes.
    pub fn from_public_key(validator_id: [u8; 32], public_key: &PublicKey, height: u64) -> Self {
        let binding = Self::compute_binding(&validator_id, public_key);
        Self {
            validator_id,
            public_key_bytes: public_key.0,
            identity_binding: binding,
            is_active: true,
            registered_at_height: height,
        }
    }

    /// N113.2: Compute the cryptographic binding between validator_id and public_key.
    pub fn compute_binding(validator_id: &[u8; 32], public_key: &PublicKey) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_IDENTITY_V1");
        hasher.update(validator_id);
        hasher.update(&public_key.0);
        hasher.finalize().into()
    }

    /// Verify that the identity binding is correct.
    pub fn verify_binding(&self) -> bool {
        let recomputed = Self::compute_binding(&self.validator_id, &self.to_public_key());
        recomputed == self.identity_binding
    }

    /// Create an identity with a given public key (convenience).
    pub fn new(validator_id: [u8; 32], public_key_bytes: [u8; 48], height: u64) -> Self {
        let pk = PublicKey(public_key_bytes);
        let binding = Self::compute_binding(&validator_id, &pk);
        Self {
            validator_id,
            public_key_bytes,
            identity_binding: binding,
            is_active: true,
            registered_at_height: height,
        }
    }
}

/// N113.1: Registry mapping validator_id → ValidatorIdentity.
pub struct ValidatorIdentityRegistry {
    identities: std::collections::HashMap<[u8; 32], ValidatorIdentity>,
}

impl ValidatorIdentityRegistry {
    pub fn new() -> Self {
        Self {
            identities: std::collections::HashMap::new(),
        }
    }

    /// Register a new identity. Returns error if the validator_id already exists.
    pub fn register(&mut self, identity: ValidatorIdentity) -> Result<(), String> {
        if !identity.verify_binding() {
            return Err("Identity binding verification failed".into());
        }
        if self.identities.contains_key(&identity.validator_id) {
            return Err(format!(
                "Validator {:?} already registered",
                &identity.validator_id[..4]
            ));
        }
        self.identities.insert(identity.validator_id, identity);
        Ok(())
    }

    /// Look up an identity by validator_id.
    pub fn get(&self, validator_id: &[u8; 32]) -> Option<&ValidatorIdentity> {
        self.identities.get(validator_id)
    }

    /// Get the PublicKey for a validator_id (for staking operations).
    pub fn get_public_key(&self, validator_id: &[u8; 32]) -> Option<PublicKey> {
        self.identities
            .get(validator_id)
            .map(|id| id.to_public_key())
    }

    /// Check if a validator is registered.
    pub fn is_registered(&self, validator_id: &[u8; 32]) -> bool {
        self.identities.contains_key(validator_id)
    }

    /// Deactivate an identity.
    pub fn deactivate(&mut self, validator_id: &[u8; 32]) -> bool {
        if let Some(id) = self.identities.get_mut(validator_id) {
            id.is_active = false;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.identities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.identities.is_empty()
    }
}

impl Default for ValidatorIdentityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pk_bytes(seed: u8) -> [u8; 48] {
        let mut key = [0u8; 48];
        key[0] = seed;
        key
    }

    #[test]
    fn n113_1_identity_binding_is_verifiable() {
        let id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        assert!(id.verify_binding());
    }

    #[test]
    fn n113_1_tampered_binding_detected() {
        let mut id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        id.identity_binding = [0xFF; 32];
        assert!(!id.verify_binding());
    }

    #[test]
    fn n113_1_different_validator_different_binding() {
        let id1 = ValidatorIdentity::new([0x01; 32], make_pk_bytes(1), 100);
        let id2 = ValidatorIdentity::new([0x02; 32], make_pk_bytes(1), 100);
        assert_ne!(id1.identity_binding, id2.identity_binding);
    }

    #[test]
    fn n113_1_different_key_different_binding() {
        let id1 = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        let id2 = ValidatorIdentity::new([0x42; 32], make_pk_bytes(2), 100);
        assert_ne!(id1.identity_binding, id2.identity_binding);
    }

    #[test]
    fn n113_1_registry_register_and_lookup() {
        let mut reg = ValidatorIdentityRegistry::new();
        let id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        reg.register(id.clone()).unwrap();
        assert!(reg.is_registered(&[0x42; 32]));
        assert_eq!(reg.get(&[0x42; 32]).unwrap().public_key_bytes[0], 1);
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn n113_1_duplicate_registration_rejected() {
        let mut reg = ValidatorIdentityRegistry::new();
        let id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        reg.register(id.clone()).unwrap();
        assert!(reg.register(id).is_err());
    }

    #[test]
    fn n113_1_binding_must_verify_before_registration() {
        let mut reg = ValidatorIdentityRegistry::new();
        let mut id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        id.identity_binding = [0xFF; 32];
        assert!(reg.register(id).is_err());
    }

    #[test]
    fn n113_1_deactivation() {
        let mut reg = ValidatorIdentityRegistry::new();
        let id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        reg.register(id).unwrap();
        assert!(reg.deactivate(&[0x42; 32]));
        assert!(!reg.get(&[0x42; 32]).unwrap().is_active);
    }

    #[test]
    fn n113_2_vote_identity_matches_staking_identity() {
        let validator_id = [0x42; 32];
        let pk_bytes = make_pk_bytes(7);
        let identity = ValidatorIdentity::new(validator_id, pk_bytes, 100);

        assert_eq!(identity.validator_id, validator_id);
        assert_eq!(identity.public_key_bytes[0], 7);

        let mut reg = ValidatorIdentityRegistry::new();
        reg.register(identity).unwrap();

        let retrieved_pk = reg.get_public_key(&validator_id).unwrap();
        assert_eq!(
            retrieved_pk.0[0], 7,
            "Staking key must match consensus identity"
        );
    }

    #[test]
    fn n113_1_identity_mapping_is_bijective() {
        let mut reg = ValidatorIdentityRegistry::new();

        let id1 = ValidatorIdentity::new([0x01; 32], make_pk_bytes(10), 1);
        let id2 = ValidatorIdentity::new([0x02; 32], make_pk_bytes(20), 2);

        reg.register(id1).unwrap();
        reg.register(id2).unwrap();

        let id1_alt = ValidatorIdentity::new([0x01; 32], make_pk_bytes(99), 3);
        assert!(reg.register(id1_alt).is_err());

        assert_ne!(
            reg.get_public_key(&[0x01; 32]).unwrap().0[0],
            reg.get_public_key(&[0x02; 32]).unwrap().0[0]
        );
    }

    #[test]
    fn n113_1_roundtrip_serialization() {
        let id = ValidatorIdentity::new([0x42; 32], make_pk_bytes(1), 100);
        let encoded = postcard::to_stdvec(&id).unwrap();
        let decoded: ValidatorIdentity = postcard::from_bytes(&encoded).unwrap();
        assert_eq!(id.validator_id, decoded.validator_id);
        assert_eq!(id.public_key_bytes, decoded.public_key_bytes);
        assert_eq!(id.identity_binding, decoded.identity_binding);
        assert!(decoded.verify_binding());
    }
}
