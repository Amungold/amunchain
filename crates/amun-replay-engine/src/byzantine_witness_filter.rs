//! ByzantineWitnessFilter — prevent malicious witness propagation.
//!
//! In a distributed environment, workers may propagate:
//!   - Incomplete witnesses (missing hard dependencies)
//!   - Misleading closure fragments
//!   - Dependency inflation attacks
//!   - Circular reference chains
//!
//! This filter validates structural properties of incoming witnesses
//! BEFORE they are used for constitutional derivation.
//! It does NOT judge constitutional validity — that remains the kernel's role.

use amun_constitutional::ConstitutionalWitness;
use amun_constitutional::WitnessType;

/// Result of filtering a witness from the network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterResult {
    /// Witness passes all structural checks.
    Accepted,
    /// Witness is structurally incomplete (missing hard dependencies).
    Incomplete {
        missing_hard_count: usize,
    },
    /// Witness appears to be part of a dependency inflation attack.
    InflationSuspected {
        total_entries: usize,
        hard_entries: usize,
        suspicious_ratio: bool,
    },
    /// Witness contains self-referential entries.
    SelfReferenceDetected,
    /// Witness is empty (no entries).
    EmptyWitness,
}

/// Filter a witness from the network for structural validity.
///
/// This is a PRE-VERIFICATION filter. It checks structural properties
/// before the constitutional kernel performs full semantic verification.
/// It prevents obviously malicious witnesses from consuming kernel resources.
pub fn filter_incoming_witness(witness: &ConstitutionalWitness) -> FilterResult {
    // Empty witness
    if witness.entries.is_empty() {
        return FilterResult::EmptyWitness;
    }

    // Check for self-references
    for entry in &witness.entries {
        if entry.artifact_hash == witness.target_artifact_hash {
            return FilterResult::SelfReferenceDetected;
        }
    }

    // Check hard dependency count
    let hard_count = witness.entries.iter()
        .filter(|e| matches!(e.witness_type, WitnessType::HardDependency))
        .count();

    if hard_count == 0 && witness.total_entry_count > 0 {
        // Witness has entries but no hard dependencies — suspicious
        return FilterResult::Incomplete { missing_hard_count: 0 };
    }

    // Check for dependency inflation (too many non-essential entries)
    let total = witness.entries.len();
    if total > 0 {
        let non_essential = total - hard_count;
        let ratio = non_essential as f64 / total as f64;
        if ratio > 0.9 && total > 10 {
            return FilterResult::InflationSuspected {
                total_entries: total,
                hard_entries: hard_count,
                suspicious_ratio: true,
            };
        }
    }

    FilterResult::Accepted
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_constitutional::{WitnessEntry, WitnessType};

    fn make_witness(entries: Vec<WitnessEntry>) -> ConstitutionalWitness {
        ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries)
    }

    #[test]
    fn test_empty_witness_rejected() {
        let w = make_witness(vec![]);
        assert!(matches!(filter_incoming_witness(&w), FilterResult::EmptyWitness));
    }

    #[test]
    fn test_valid_witness_accepted() {
        let entries = vec![
            WitnessEntry::new([0x01; 32], WitnessType::HardDependency),
            WitnessEntry::new([0x02; 32], WitnessType::SupportingDependency),
        ];
        let w = make_witness(entries);
        assert!(matches!(filter_incoming_witness(&w), FilterResult::Accepted));
    }

    #[test]
    fn test_inflation_suspected() {
        let mut entries = Vec::new();
        entries.push(WitnessEntry::new([0x01; 32], WitnessType::HardDependency));
        for i in 0..50 {
            entries.push(WitnessEntry::new([i as u8; 32], WitnessType::AuditDependency));
        }
        let w = make_witness(entries);
        let result = filter_incoming_witness(&w);
        assert!(matches!(result, FilterResult::InflationSuspected { .. }));
    }

    #[test]
    fn test_self_reference_rejected() {
        let entries = vec![
            WitnessEntry::new([0xAA; 32], WitnessType::HardDependency), // same as target
        ];
        let w = make_witness(entries);
        assert!(matches!(filter_incoming_witness(&w), FilterResult::SelfReferenceDetected));
    }
}
