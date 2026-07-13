//! Canonical Witness Ordering — deterministic proof surface.
//!
//! Ensures that two constitutionally equivalent proofs produce
//! IDENTICAL witness hashes. This is essential for:
//!   - Proof portability across runtimes
//!   - Witness caching and equivalence checking
//!   - Federation agreement on proof identity
//!
//! RULE: Witness entries are ordered canonically by:
//!   1. WitnessType priority (HardDependency first, then Supporting, Audit, Elidable)
//!   2. Artifact hash (lexicographic) within same WitnessType

use crate::constitutional_witness::WitnessEntry;
use crate::witness_type::WitnessType;

/// Order witness entries canonically for deterministic hashing.
///
/// Ordering rules:
///   1. HardDependency entries come first (they are required for validity)
///   2. SupportingDependency entries come second
///   3. AuditDependency entries come third
///   4. CompressionElidable entries come last
///   5. Within the same WitnessType, entries are sorted by artifact_hash (lexicographic)
///
/// This ordering is CONSTITUTIONALLY DETERMINISTIC:
/// same entries → same order → same witness hash.
pub fn canonical_order(entries: &mut [WitnessEntry]) {
    entries.sort_by(|a, b| {
        // Primary sort: WitnessType priority
        let priority_a = type_priority(a.witness_type);
        let priority_b = type_priority(b.witness_type);
        match priority_a.cmp(&priority_b) {
            core::cmp::Ordering::Equal => {
                // Secondary sort: lexicographic by artifact hash
                a.artifact_hash.cmp(&b.artifact_hash)
            }
            other => other,
        }
    });
}

/// Priority for canonical ordering. Lower = earlier in witness.
fn type_priority(wt: WitnessType) -> u8 {
    match wt {
        WitnessType::HardDependency => 0,
        WitnessType::SupportingDependency => 1,
        WitnessType::AuditDependency => 2,
        WitnessType::CompressionElidable => 3,
    }
}

/// Normalize a witness: return a new witness with canonically ordered entries.
/// The original witness is not modified.
pub fn normalize(
    witness: &crate::constitutional_witness::ConstitutionalWitness,
) -> crate::constitutional_witness::ConstitutionalWitness {
    let mut entries = witness.entries.clone();
    canonical_order(&mut entries);
    crate::constitutional_witness::ConstitutionalWitness::new(
        witness.witness_id,
        witness.constitutional_revision,
        witness.replay_revision,
        witness.target_artifact_hash,
        witness.context_hash,
        entries,
    )
}

/// Verify that entries are in canonical order.
pub fn is_canonical(entries: &[WitnessEntry]) -> bool {
    for i in 1..entries.len() {
        let prev = &entries[i - 1];
        let curr = &entries[i];
        let prev_prio = type_priority(prev.witness_type);
        let curr_prio = type_priority(curr.witness_type);
        if prev_prio > curr_prio {
            return false;
        }
        if prev_prio == curr_prio && prev.artifact_hash > curr.artifact_hash {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(wt: WitnessType, hash: [u8; 32]) -> WitnessEntry {
        WitnessEntry::new(hash, wt)
    }

    #[test]
    fn test_canonical_order_respects_priority() {
        let mut entries = vec![
            entry(WitnessType::CompressionElidable, [0x01; 32]),
            entry(WitnessType::HardDependency, [0x02; 32]),
            entry(WitnessType::SupportingDependency, [0x03; 32]),
            entry(WitnessType::AuditDependency, [0x04; 32]),
        ];
        canonical_order(&mut entries);
        // Hard should be first, CompressionElidable last
        assert_eq!(entries[0].witness_type, WitnessType::HardDependency);
        assert_eq!(entries[3].witness_type, WitnessType::CompressionElidable);
    }

    #[test]
    fn test_canonical_order_same_type_lexicographic() {
        let mut entries = vec![
            entry(WitnessType::HardDependency, [0xCC; 32]),
            entry(WitnessType::HardDependency, [0xAA; 32]),
            entry(WitnessType::HardDependency, [0xBB; 32]),
        ];
        canonical_order(&mut entries);
        assert_eq!(entries[0].artifact_hash, [0xAA; 32]);
        assert_eq!(entries[1].artifact_hash, [0xBB; 32]);
        assert_eq!(entries[2].artifact_hash, [0xCC; 32]);
    }

    #[test]
    fn test_is_canonical() {
        let entries = vec![
            entry(WitnessType::HardDependency, [0xAA; 32]),
            entry(WitnessType::HardDependency, [0xBB; 32]),
            entry(WitnessType::SupportingDependency, [0xCC; 32]),
        ];
        assert!(is_canonical(&entries));
    }

    #[test]
    fn test_is_not_canonical_wrong_priority_order() {
        let entries = vec![
            entry(WitnessType::SupportingDependency, [0xAA; 32]),
            entry(WitnessType::HardDependency, [0xBB; 32]),
        ];
        assert!(!is_canonical(&entries));
    }

    #[test]
    fn test_normalize_produces_canonical() {
        use crate::constitutional_witness::ConstitutionalWitness;
        let entries = vec![
            entry(WitnessType::CompressionElidable, [0x01; 32]),
            entry(WitnessType::HardDependency, [0x02; 32]),
        ];
        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
        let normalized = normalize(&w);
        assert!(is_canonical(&normalized.entries));
        assert_ne!(w.witness_hash, normalized.witness_hash);
    }

    #[test]
    fn test_normalize_idempotent() {
        use crate::constitutional_witness::ConstitutionalWitness;
        let entries = vec![
            entry(WitnessType::HardDependency, [0xAA; 32]),
            entry(WitnessType::SupportingDependency, [0xBB; 32]),
        ];
        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
        let n1 = normalize(&w);
        let n2 = normalize(&n1);
        assert_eq!(n1.witness_hash, n2.witness_hash);
    }
}
