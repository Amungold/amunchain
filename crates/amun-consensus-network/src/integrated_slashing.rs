// ============================================================================
// N109.12 — Integrated Slashing Pipeline
// ============================================================================
// Connects EvidenceStore → MisbehaviorRegistry(N109.11) → SlashingEngine → Stake Penalty
//
// This is the bridge between N109 (Evidence + Misbehavior) and
// the existing slashing infrastructure.
//
// Flow:
//   Invalid Vote
//     ↓
//   EvidenceStore::store_evidence()
//     ↓
//   MisbehaviorRegistry::record_misbehavior()
//     ↓
//   check_thresholds() → Option<ValidatorAction>
//     ↓
//   SlashingEngine::process() → penalty_bps
//     ↓
//   stake.slash(penalty_bps)
// ============================================================================

use crate::evidence_store::{EvidenceRecord, EvidenceStatus, EvidenceStore, EvidenceType};
use crate::misbehavior_registry::{MisbehaviorRegistry, MisbehaviorThresholds, ValidatorAction};

/// N109.12: Result of processing an evidence through the slashing pipeline
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineResult {
    /// Evidence was new and stored
    EvidenceStored,
    /// Evidence was a duplicate, not counted
    DuplicateIgnored,
    /// Validator crossed warning threshold
    WarningTriggered,
    /// Validator crossed suspension threshold
    SuspensionTriggered,
    /// Validator crossed slashing threshold — penalty amount in bps
    SlashingTriggered { penalty_bps: u64 },
    /// No threshold crossed yet
    NoAction,
}

/// N109.12: Integrated pipeline connecting evidence to slashing
pub struct IntegratedSlashingPipeline {
    pub evidence_store: EvidenceStore,
    pub registry: MisbehaviorRegistry,
    /// Penalty in basis points (1/10000) for slashing
    pub base_penalty_bps: u64,
    /// Additional penalty per offense beyond the first
    pub penalty_multiplier: u64,
}

impl IntegratedSlashingPipeline {
    pub fn new(thresholds: MisbehaviorThresholds) -> Self {
        Self {
            evidence_store: EvidenceStore::new(),
            registry: MisbehaviorRegistry::new(thresholds),
            base_penalty_bps: 500, // 5%
            penalty_multiplier: 2, // 2x per additional offense
        }
    }

    /// N109.12: Process a vote binding violation through the full pipeline.
    ///
    /// This is the main entry point. Every time verify_vote_binding fails,
    /// this function should be called instead of just returning Err.
    ///
    /// Returns the pipeline result indicating what action (if any) should be taken.
    pub fn process_violation(
        &mut self,
        validator_id: &[u8; 32],
        height: u64,
        evidence_type: EvidenceType,
        detail: &str,
    ) -> PipelineResult {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Step 1: Create evidence record
        let evidence = EvidenceRecord::new(
            *validator_id,
            height,
            evidence_type,
            timestamp,
            detail.as_bytes().to_vec(),
        );

        // Step 2: Store in EvidenceStore (dedup)
        if !self.evidence_store.store_evidence(evidence.clone()) {
            return PipelineResult::DuplicateIgnored;
        }

        // Step 3: Record in MisbehaviorRegistry
        let is_new = self.registry.record_misbehavior(
            validator_id,
            &evidence.evidence_id,
            &evidence.evidence_type,
            height,
        );

        if !is_new {
            return PipelineResult::DuplicateIgnored;
        }

        // Step 4: Check thresholds
        match self.registry.check_thresholds(validator_id) {
            Some(ValidatorAction::Warn) => PipelineResult::WarningTriggered,
            Some(ValidatorAction::Suspend) => PipelineResult::SuspensionTriggered,
            Some(ValidatorAction::Slash) => {
                // Calculate penalty
                let score = self.registry.get_score(validator_id);
                let penalty = self.calculate_penalty(score);
                PipelineResult::SlashingTriggered {
                    penalty_bps: penalty,
                }
            }
            None => PipelineResult::NoAction,
        }
    }

    /// N109.12: Calculate slashing penalty in basis points based on score
    fn calculate_penalty(&self, score: u64) -> u64 {
        // Base penalty + multiplier per threshold exceeded
        let multiplier = if score <= self.registry.thresholds.slashing_score {
            1
        } else {
            ((score - self.registry.thresholds.slashing_score) / 10 + 1)
                .saturating_mul(self.penalty_multiplier)
                .min(10)
        };
        (self.base_penalty_bps)
            .saturating_mul(multiplier)
            .min(10000) // Cap at 100%
    }

    /// N109.12: Get penalty for a specific validator (if slashable)
    pub fn get_penalty(&self, validator_id: &[u8; 32]) -> Option<u64> {
        match self.registry.check_thresholds(validator_id) {
            Some(ValidatorAction::Slash) => {
                let score = self.registry.get_score(validator_id);
                Some(self.calculate_penalty(score))
            }
            _ => None,
        }
    }

    /// N109.12: Mark evidence as slashed after penalty is applied
    pub fn mark_slashed(&mut self, evidence_id: &[u8; 32]) -> bool {
        self.evidence_store
            .update_status(evidence_id, EvidenceStatus::Slashed)
    }
}

impl Default for IntegratedSlashingPipeline {
    fn default() -> Self {
        Self::new(MisbehaviorThresholds::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::misbehavior_registry::ValidatorStatus;

    /// N109.12 GATEKEEPER: Evidence triggers the full slashing pipeline
    #[test]
    fn n109_12_evidence_triggers_slashing_pipeline() {
        let mut pipeline = IntegratedSlashingPipeline::default();

        // Simulate 4 state root mismatches from validator [0x42; 32]
        // Each StateRootMismatch = weight 3
        // After 4: score = 12 → Warning (>=5), not yet Suspension (<15)
        for i in 0..4 {
            let result = pipeline.process_violation(
                &[0x42; 32],
                100 + i,
                EvidenceType::StateRootMismatch,
                "test violation",
            );
            if i < 1 {
                // First violation: score=3 < 5 → NoAction
                assert_eq!(
                    result,
                    PipelineResult::NoAction,
                    "Violation {} should not trigger action yet",
                    i
                );
            } else if i < 3 {
                // Second+ violation: score >= 5 → Warning
                assert_eq!(
                    result,
                    PipelineResult::WarningTriggered,
                    "Violation {} should trigger warning (score >= 5)",
                    i
                );
            }
        }

        // Score should be 12 (4 × 3)
        assert_eq!(pipeline.registry.get_score(&[0x42; 32]), 12);
        // Status should be Warned (>=5), not Suspended (<15)
        assert_eq!(
            pipeline.registry.get_status(&[0x42; 32]),
            ValidatorStatus::Warned
        );
        // Not slashable yet
        assert!(pipeline.get_penalty(&[0x42; 32]).is_none());

        // Add one DoubleVote (weight 10) → score = 22 → Suspension (>=15), not Slash (<30)
        let result = pipeline.process_violation(
            &[0x42; 32],
            105,
            EvidenceType::DoubleVote,
            "double vote detected",
        );
        assert_eq!(result, PipelineResult::SuspensionTriggered);
        assert_eq!(pipeline.registry.get_score(&[0x42; 32]), 22);
        assert_eq!(
            pipeline.registry.get_status(&[0x42; 32]),
            ValidatorStatus::Suspended
        );

        // Add another DoubleVote (weight 10) → score = 32 → Slash Eligible (>=30)
        let result = pipeline.process_violation(
            &[0x42; 32],
            106,
            EvidenceType::DoubleVote,
            "double vote detected again",
        );
        assert!(matches!(result, PipelineResult::SlashingTriggered { .. }));
        assert_eq!(pipeline.registry.get_score(&[0x42; 32]), 32);
        assert_eq!(
            pipeline.registry.get_status(&[0x42; 32]),
            ValidatorStatus::SlashEligible
        );

        // Penalty should be calculated
        let penalty = pipeline.get_penalty(&[0x42; 32]);
        assert!(penalty.is_some());
        assert!(penalty.unwrap() > 0, "Penalty must be positive");

        eprintln!(
            "N109.12 GATEKEEPER PASSED: Score={}, Status={:?}, Penalty={}bps",
            pipeline.registry.get_score(&[0x42; 32]),
            pipeline.registry.get_status(&[0x42; 32]),
            penalty.unwrap_or(0),
        );
    }

    /// N109.12: Duplicate evidence should not trigger actions twice
    #[test]
    fn n109_12_duplicate_evidence_ignored() {
        let mut pipeline = IntegratedSlashingPipeline::default();

        // Process first violation
        let r1 = pipeline.process_violation(
            &[0x42; 32],
            1,
            EvidenceType::InvalidSignature,
            "sig invalid",
        );
        assert_eq!(r1, PipelineResult::NoAction);

        // Process same violation again (same validator, height, type, detail)
        let r2 = pipeline.process_violation(
            &[0x42; 32],
            1,
            EvidenceType::InvalidSignature,
            "sig invalid",
        );
        assert_eq!(r2, PipelineResult::DuplicateIgnored);

        // Score should still be 2 (only counted once)
        assert_eq!(pipeline.registry.get_score(&[0x42; 32]), 2);
    }

    /// N109.12: EvidenceStore records are accessible after pipeline processing
    #[test]
    fn n109_12_evidence_store_accessible() {
        let mut pipeline = IntegratedSlashingPipeline::default();

        pipeline.process_violation(
            &[0xAA; 32],
            10,
            EvidenceType::VoteBindingViolation,
            "height mismatch",
        );

        let evidence_list = pipeline.evidence_store.get_all_for_validator(&[0xAA; 32]);
        assert_eq!(evidence_list.len(), 1);
        assert_eq!(
            evidence_list[0].evidence_type,
            EvidenceType::VoteBindingViolation
        );
    }

    /// N109.12: Different validators are isolated
    #[test]
    fn n109_12_different_validators_isolated() {
        let mut pipeline = IntegratedSlashingPipeline::default();

        // Validator 1: accumulate enough for warning
        pipeline.process_violation(&[1u8; 32], 1, EvidenceType::DoubleVote, "dv"); // +10
        assert_eq!(pipeline.registry.get_score(&[1u8; 32]), 10);
        assert_eq!(
            pipeline.registry.get_status(&[1u8; 32]),
            ValidatorStatus::Warned
        );

        // Validator 2: no violations
        assert_eq!(pipeline.registry.get_score(&[2u8; 32]), 0);
        assert_eq!(
            pipeline.registry.get_status(&[2u8; 32]),
            ValidatorStatus::Active
        );
    }

    /// N109.12: Custom thresholds work in pipeline
    #[test]
    fn n109_12_custom_thresholds_pipeline() {
        let thresholds = MisbehaviorThresholds {
            warning_score: 3,
            suspension_score: 6,
            slashing_score: 9,
            double_vote_weight: 5,
            ..Default::default()
        };
        let mut pipeline = IntegratedSlashingPipeline::new(thresholds);

        // One double vote (weight 5) → >=3 warning, <6 suspension
        let r1 = pipeline.process_violation(&[1u8; 32], 1, EvidenceType::DoubleVote, "dv");
        assert_eq!(r1, PipelineResult::WarningTriggered);

        // Two double votes (weight 10) → >=9 slashing
        let r2 = pipeline.process_violation(&[1u8; 32], 2, EvidenceType::DoubleVote, "dv2");
        assert!(matches!(r2, PipelineResult::SlashingTriggered { .. }));
    }

    /// N109.12: Mark evidence as slashed after penalty applied
    #[test]
    fn n109_12_mark_slashed_updates_status() {
        let mut pipeline = IntegratedSlashingPipeline::default();

        pipeline.process_violation(&[0x42; 32], 1, EvidenceType::DoubleVote, "dv");

        let evidence_list = pipeline.evidence_store.get_all_for_validator(&[0x42; 32]);
        let evidence_id = evidence_list[0].evidence_id;

        assert!(pipeline.mark_slashed(&evidence_id));

        let updated = pipeline.evidence_store.get_by_id(&evidence_id).unwrap();
        assert_eq!(updated.status, EvidenceStatus::Slashed);
    }
}
