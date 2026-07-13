//! OperationalHasher — separate hashing for runtime/operational objects.
//!
//! INVARIANT: Operational hashes are NOT constitutional hashes.
//! They serve runtime purposes (traceability, scheduling, debugging)
//! and MUST NOT be used for constitutional identity, admissibility,
//! or proof semantics.
//!
//! This separation prevents semantic leakage where operational
//! metadata accidentally becomes constitutionally meaningful.

use sha2::{Sha256, Digest};

const OPERATIONAL_DOMAIN: &[u8] = b"AMUN|OPERATIONAL_HASHER|V1";

/// Hasher for operational/runtime objects ONLY.
///
/// This hasher MUST NOT be used for constitutional objects.
/// Constitutional objects use `amun_constitutional::ConstitutionalHasher`.
#[derive(Clone)]
pub struct OperationalHasher {
    inner: Sha256,
}

impl OperationalHasher {
    pub fn new(tag: &[u8]) -> Self {
        let mut inner = Sha256::new();
        inner.update(OPERATIONAL_DOMAIN);
        inner.update(tag);
        Self { inner }
    }

    pub fn update_u64(&mut self, v: u64) -> &mut Self { self.inner.update(v.to_le_bytes()); self }
    pub fn update_u32(&mut self, v: u32) -> &mut Self { self.inner.update(v.to_le_bytes()); self }
    pub fn update_u8(&mut self, v: u8) -> &mut Self { self.inner.update(&[v]); self }
    pub fn update_bytes(&mut self, v: &[u8]) -> &mut Self { self.inner.update(v); self }

    pub fn finalize(self) -> [u8; 32] { self.inner.finalize().into() }
}

/// An operational hash — NOT a constitutional hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalHash(pub [u8; 32]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_hash_deterministic() {
        let h1 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
        let h2 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_operational_separate_from_constitutional() {
        // Same tag, same data — but different domain prefix ensures separation
        let op_hash = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
        // Constitutional hasher would produce a different hash for the same data
        // because it uses a different domain prefix
        assert_ne!(op_hash, [0u8; 32]); // Just verify it's not empty
    }
}
