//! WitnessType — classifies artifacts within a constitutional witness.
//!
//! Not every causal dependency belongs in the minimal witness.
//! This module defines which dependencies are mandatory, which are
//! supporting, which are audit-only, and which can be elided.

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// The proof role of an artifact within a constitutional witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessType {
    /// Hard dependency — proof is INVALID without this artifact.
    /// Example: the certificate that attests admissibility for a receipt.
    HardDependency = 0x01,

    /// Supporting dependency — helps verification but is not mandatory.
    /// The proof remains valid without it, but verification may be less efficient.
    /// Example: intermediate journal entries between two anchors.
    SupportingDependency = 0x02,

    /// Audit dependency — included for traceability only.
    /// Does not affect constitutional validity.
    /// Example: a parent receipt in an audit chain.
    AuditDependency = 0x03,

    /// Compression-elidable — can be dropped without affecting proof sufficiency.
    /// The causal information is preserved by other artifacts in the witness.
    /// Example: an intermediate commitment when the anchor covers the same span.
    CompressionElidable = 0x04,
}

impl WitnessType {
    pub fn type_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"WITNESS_TYPE").update_u8(*self as u8);
        h.finalize()
    }

    /// Returns true if this witness type is required for proof validity.
    pub fn is_required(&self) -> bool {
        matches!(self, WitnessType::HardDependency)
    }

    /// Returns true if this witness type can be safely removed.
    pub fn is_elidable(&self) -> bool {
        matches!(self, WitnessType::CompressionElidable)
    }

    /// Returns true if this witness type contributes to proof understanding
    /// but not to proof validity.
    pub fn is_non_essential(&self) -> bool {
        matches!(
            self,
            WitnessType::AuditDependency | WitnessType::CompressionElidable
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hard_is_required() {
        assert!(WitnessType::HardDependency.is_required());
        assert!(!WitnessType::SupportingDependency.is_required());
        assert!(!WitnessType::AuditDependency.is_required());
        assert!(!WitnessType::CompressionElidable.is_required());
    }
    #[test]
    fn test_elidable() {
        assert!(WitnessType::CompressionElidable.is_elidable());
        assert!(!WitnessType::HardDependency.is_elidable());
    }
    #[test]
    fn test_non_essential() {
        assert!(WitnessType::AuditDependency.is_non_essential());
        assert!(WitnessType::CompressionElidable.is_non_essential());
        assert!(!WitnessType::HardDependency.is_non_essential());
    }
}
