// ============================================================================
// N109.13 — Unified Slashing Interface
// ============================================================================
// This replaces the old slashing.rs that depended on misbehavior.rs (N101).
// Now uses the N109 misbehavior_registry and integrated_slashing pipeline.
//
// The old should_slash() and slash_percentage() are reimplemented on top
// of the new MisbehaviorRegistry, maintaining backward compatibility while
// unifying the architecture.
// ============================================================================

use crate::misbehavior_registry::{MisbehaviorRegistry, ValidatorAction, ValidatorStatus};

/// N109.13: Returns true if the validator should be slashed.
/// Now uses the N109 MisbehaviorRegistry instead of the old misbehavior.rs.
pub fn should_slash(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> bool {
    registry.check_thresholds(validator_id) == Some(ValidatorAction::Slash)
}

/// N109.13: Returns the slash percentage for a validator based on their misbehavior score.
/// Maps score ranges to penalty percentages.
pub fn slash_percentage(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> u8 {
    let score = registry.get_score(validator_id);
    match score {
        0 => 0,
        1..=14 => 5,   // Warning level
        15..=29 => 10, // Suspension level
        _ => 25,       // Slashing level
    }
}

/// N109.13: Returns the validator's current status
pub fn validator_status(
    registry: &MisbehaviorRegistry,
    validator_id: &[u8; 32],
) -> ValidatorStatus {
    registry.get_status(validator_id)
}

/// N109.13: Returns the total misbehavior score
pub fn misbehavior_score(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> u64 {
    registry.get_score(validator_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::misbehavior_registry::{MisbehaviorRegistry, MisbehaviorThresholds};

    fn make_registry() -> MisbehaviorRegistry {
        MisbehaviorRegistry::new(MisbehaviorThresholds::default())
    }

    #[test]
    fn n109_13_no_offenses_no_slash() {
        let registry = make_registry();
        assert!(!should_slash(&registry, &[1u8; 32]));
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 0);
        assert_eq!(misbehavior_score(&registry, &[1u8; 32]), 0);
    }

    #[test]
    fn n109_13_single_offense_below_threshold() {
        let mut registry = make_registry();
        // One StateRootMismatch (weight 3) → score=3, below warning (5)
        registry.record_misbehavior(
            &[1u8; 32],
            &[0xA1; 32],
            &EvidenceType::StateRootMismatch,
            10,
        );
        assert_eq!(registry.get_score(&[1u8; 32]), 3);
        assert!(
            !should_slash(&registry, &[1u8; 32]),
            "Single offense should not trigger slash"
        );
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 5); // Warning level
    }

    #[test]
    fn n109_13_two_offenses_triggers_warning() {
        let mut registry = make_registry();
        // Two DoubleVotes (weight 10 each) → score=20, suspension (15-29)
        registry.record_misbehavior(&[1u8; 32], &[0xB1; 32], &EvidenceType::DoubleVote, 10);
        registry.record_misbehavior(&[1u8; 32], &[0xB2; 32], &EvidenceType::DoubleVote, 20);
        assert_eq!(registry.get_score(&[1u8; 32]), 20);
        assert_eq!(registry.get_status(&[1u8; 32]), ValidatorStatus::Suspended);
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 10); // Suspension level
    }

    #[test]
    fn n109_13_three_offenses_triggers_slash() {
        let mut registry = make_registry();
        // Three DoubleVotes (weight 10 each) → score=30, slashing (>=30)
        registry.record_misbehavior(&[1u8; 32], &[0xC1; 32], &EvidenceType::DoubleVote, 10);
        registry.record_misbehavior(&[1u8; 32], &[0xC2; 32], &EvidenceType::DoubleVote, 20);
        registry.record_misbehavior(&[1u8; 32], &[0xC3; 32], &EvidenceType::DoubleVote, 30);
        assert_eq!(registry.get_score(&[1u8; 32]), 30);
        assert!(
            should_slash(&registry, &[1u8; 32]),
            "Three offenses should trigger slash"
        );
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 25); // Slashing level
    }

    #[test]
    fn n109_13_mixed_offenses_accumulate() {
        let mut registry = make_registry();
        // Mix of offense types
        registry.record_misbehavior(&[1u8; 32], &[0xD1; 32], &EvidenceType::InvalidSignature, 1); // +2
        registry.record_misbehavior(&[1u8; 32], &[0xD2; 32], &EvidenceType::StateRootMismatch, 2); // +3
        registry.record_misbehavior(
            &[1u8; 32],
            &[0xD3; 32],
            &EvidenceType::VoteBindingViolation,
            3,
        ); // +2
        registry.record_misbehavior(&[1u8; 32], &[0xD4; 32], &EvidenceType::FutureVote, 4); // +1
        assert_eq!(registry.get_score(&[1u8; 32]), 8);
        assert_eq!(registry.get_status(&[1u8; 32]), ValidatorStatus::Warned);
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 5);
    }

    #[test]
    fn n109_13_all_slashable_offenses_flow_through_unified_api() {
        let mut registry = make_registry();

        // Process various offense types through the unified API
        let offenses = vec![
            (EvidenceType::DoubleVote, 10),
            (EvidenceType::StateRootMismatch, 13),
            (EvidenceType::InvalidSignature, 2),
            (EvidenceType::VoteBindingViolation, 2),
            (EvidenceType::ExecutionFailure, 3),
            (EvidenceType::FutureVote, 1),
        ];

        let mut evidence_counter = 0u8;
        for (offense_type, _weight) in &offenses {
            evidence_counter += 1;
            registry.record_misbehavior(
                &[0x42; 32],
                &[evidence_counter; 32],
                offense_type,
                100 + evidence_counter as u64,
            );
        }

        // Total: 10+3+2+2+3+1 = 21 → Suspension (15-29)
        assert_eq!(registry.get_score(&[0x42; 32]), 21);
        assert_eq!(registry.get_status(&[0x42; 32]), ValidatorStatus::Suspended);
        assert!(!should_slash(&registry, &[0x42; 32])); // 21 < 30
        assert_eq!(slash_percentage(&registry, &[0x42; 32]), 10); // Suspension level

        // Add one more DoubleVote → 31 → Slashing
        registry.record_misbehavior(&[0x42; 32], &[0xFF; 32], &EvidenceType::DoubleVote, 200);
        assert!(should_slash(&registry, &[0x42; 32]));
        assert_eq!(slash_percentage(&registry, &[0x42; 32]), 25);
    }
}
