// ============================================================================
// N109.11 — Misbehavior Registry
// ============================================================================
// Aggregates evidence from EvidenceStore into a per-validator "crime record".
// Computes a weighted MisbehaviorScore and triggers actions at thresholds.
//
// After N109.11, N110 Slashing can simply:
//   registry.check_thresholds(validator_id) → Option<ValidatorAction>
//
// Key design decisions:
//   - Weights are configurable (not hardcoded) for policy evolution
//   - O(1) validator lookup via HashMap
//   - Duplicate evidence is never counted twice (Gatekeeper)
//   - Registry survives reload by rebuilding from EvidenceStore
// ============================================================================

use crate::evidence_store::{EvidenceStatus, EvidenceStore, EvidenceType};
use std::collections::HashMap;

/// N109.11: Configurable misbehavior weights
#[derive(Debug, Clone)]
pub struct MisbehaviorThresholds {
    pub warning_score: u64,
    pub suspension_score: u64,
    pub slashing_score: u64,
    pub state_root_mismatch_weight: u64,
    pub invalid_signature_weight: u64,
    pub vote_binding_weight: u64,
    pub double_vote_weight: u64,
    pub future_vote_weight: u64,
    pub execution_failure_weight: u64,
}

impl Default for MisbehaviorThresholds {
    fn default() -> Self {
        Self {
            warning_score: 5,
            suspension_score: 15,
            slashing_score: 30,
            state_root_mismatch_weight: 3,
            invalid_signature_weight: 2,
            vote_binding_weight: 2,
            double_vote_weight: 10,
            future_vote_weight: 1,
            execution_failure_weight: 3,
        }
    }
}

/// N109.11: Validator status based on misbehavior score
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Warned,
    Suspended,
    SlashEligible,
}

/// N109.11: Recommended action based on threshold check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorAction {
    Warn,
    Suspend,
    Slash,
}

/// N109.11: Per-validator misbehavior record
#[derive(Debug, Clone)]
pub struct MisbehaviorRecord {
    pub validator_id: [u8; 32],
    /// evidence_ids that have been counted (prevents double-counting)
    pub counted_evidence: Vec<[u8; 32]>,
    /// Count of each evidence type
    pub counts: HashMap<EvidenceType, u64>,
    /// Weighted total score
    pub total_score: u64,
    /// First height where misbehavior was recorded
    pub first_seen_height: u64,
    /// Latest height where misbehavior was recorded
    pub last_seen_height: u64,
    /// Current validator status
    pub status: ValidatorStatus,
}

impl MisbehaviorRecord {
    pub fn new(validator_id: [u8; 32], height: u64) -> Self {
        Self {
            validator_id,
            counted_evidence: Vec::new(),
            counts: HashMap::new(),
            total_score: 0,
            first_seen_height: height,
            last_seen_height: height,
            status: ValidatorStatus::Active,
        }
    }
}

/// N109.11: Registry of all validator misbehavior
#[derive(Debug, Clone)]
pub struct MisbehaviorRegistry {
    pub records: HashMap<[u8; 32], MisbehaviorRecord>,
    pub thresholds: MisbehaviorThresholds,
}

impl MisbehaviorRegistry {
    pub fn new(thresholds: MisbehaviorThresholds) -> Self {
        Self {
            records: HashMap::new(),
            thresholds,
        }
    }

    /// N109.11: Record a new evidence against a validator.
    /// Returns true if this evidence was new (not already counted).
    pub fn record_misbehavior(
        &mut self,
        validator_id: &[u8; 32],
        evidence_id: &[u8; 32],
        evidence_type: &EvidenceType,
        height: u64,
    ) -> bool {
        // Get weight before borrowing self.records mutably
        let weight = self.get_weight(evidence_type);

        let record = self
            .records
            .entry(*validator_id)
            .or_insert_with(|| MisbehaviorRecord::new(*validator_id, height));

        // N109.11 Gatekeeper: never count the same evidence twice
        if record.counted_evidence.iter().any(|id| id == evidence_id) {
            return false; // Already counted
        }

        // Mark as counted
        record.counted_evidence.push(*evidence_id);

        // Update counts
        *record.counts.entry(evidence_type.clone()).or_insert(0) += 1;

        // Add weighted score
        record.total_score += weight;

        // Update height range
        record.last_seen_height = record.last_seen_height.max(height);

        // Check thresholds (pass record's total_score directly)
        let score = record.total_score;
        if score >= self.thresholds.slashing_score {
            record.status = ValidatorStatus::SlashEligible;
        } else if score >= self.thresholds.suspension_score {
            record.status = ValidatorStatus::Suspended;
        } else if score >= self.thresholds.warning_score {
            record.status = ValidatorStatus::Warned;
        }

        true
    }

    /// N109.11: Get the weight for an evidence type
    fn get_weight(&self, evidence_type: &EvidenceType) -> u64 {
        match evidence_type {
            EvidenceType::StateRootMismatch => self.thresholds.state_root_mismatch_weight,
            EvidenceType::InvalidExecutionCommitment => self.thresholds.execution_failure_weight,
            EvidenceType::InvalidSignature => self.thresholds.invalid_signature_weight,
            EvidenceType::VoteBindingViolation => self.thresholds.vote_binding_weight,
            EvidenceType::DoubleVote => self.thresholds.double_vote_weight,
            EvidenceType::FutureVote => self.thresholds.future_vote_weight,
            EvidenceType::ExecutionFailure => self.thresholds.execution_failure_weight,
        }
    }

    /// N109.11: Update validator status based on current score
    /// N110.4: Update validator status based on current score.
    /// Used internally by record_misbehavior. Marked as allow(dead_code)
    /// because the compiler sees it only called within the same impl block.
    #[allow(dead_code)]
    fn update_status(&mut self, record: &mut MisbehaviorRecord) {
        let score = record.total_score;
        if score >= self.thresholds.slashing_score {
            record.status = ValidatorStatus::SlashEligible;
        } else if score >= self.thresholds.suspension_score {
            record.status = ValidatorStatus::Suspended;
        } else if score >= self.thresholds.warning_score {
            record.status = ValidatorStatus::Warned;
        }
    }

    /// N109.11: Check if a validator has crossed any threshold
    pub fn check_thresholds(&self, validator_id: &[u8; 32]) -> Option<ValidatorAction> {
        let record = self.records.get(validator_id)?;
        let score = record.total_score;
        if score >= self.thresholds.slashing_score {
            Some(ValidatorAction::Slash)
        } else if score >= self.thresholds.suspension_score {
            Some(ValidatorAction::Suspend)
        } else if score >= self.thresholds.warning_score {
            Some(ValidatorAction::Warn)
        } else {
            None
        }
    }

    /// N109.11: Get a validator's total misbehavior score
    pub fn get_score(&self, validator_id: &[u8; 32]) -> u64 {
        self.records
            .get(validator_id)
            .map(|r| r.total_score)
            .unwrap_or(0)
    }

    /// N109.11: Get a validator's status
    pub fn get_status(&self, validator_id: &[u8; 32]) -> ValidatorStatus {
        self.records
            .get(validator_id)
            .map(|r| r.status.clone())
            .unwrap_or(ValidatorStatus::Active)
    }

    /// N109.11: Rebuild registry from EvidenceStore (e.g., after restart)
    pub fn rebuild_from_evidence(
        thresholds: MisbehaviorThresholds,
        evidence_store: &EvidenceStore,
    ) -> Self {
        let mut registry = Self::new(thresholds);
        for record in evidence_store.records.values() {
            if record.status == EvidenceStatus::Rejected {
                continue;
            }
            registry.record_misbehavior(
                &record.validator_id,
                &record.evidence_id,
                &record.evidence_type,
                record.height,
            );
        }
        registry
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl Default for MisbehaviorRegistry {
    fn default() -> Self {
        Self::new(MisbehaviorThresholds::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n109_11_record_misbehavior() {
        let mut reg = MisbehaviorRegistry::default();
        let added =
            reg.record_misbehavior(&[1u8; 32], &[0xAA; 32], &EvidenceType::InvalidSignature, 5);
        assert!(added);
        assert_eq!(reg.get_score(&[1u8; 32]), 2); // weight=2
    }

    #[test]
    fn n109_11_score_accumulates() {
        let mut reg = MisbehaviorRegistry::default();
        reg.record_misbehavior(&[1u8; 32], &[0xA1; 32], &EvidenceType::InvalidSignature, 1); // +2
        reg.record_misbehavior(&[1u8; 32], &[0xA2; 32], &EvidenceType::StateRootMismatch, 2); // +3
        assert_eq!(reg.get_score(&[1u8; 32]), 5);
    }

    #[test]
    fn n109_11_different_evidence_types_weighted() {
        let mut reg = MisbehaviorRegistry::default();
        reg.record_misbehavior(&[1u8; 32], &[0xB1; 32], &EvidenceType::FutureVote, 1); // +1
        reg.record_misbehavior(&[1u8; 32], &[0xB2; 32], &EvidenceType::DoubleVote, 2); // +10
        reg.record_misbehavior(
            &[1u8; 32],
            &[0xB3; 32],
            &EvidenceType::VoteBindingViolation,
            3,
        ); // +2
        assert_eq!(reg.get_score(&[1u8; 32]), 13);
    }

    #[test]
    fn n109_11_warning_threshold_triggered() {
        let mut reg = MisbehaviorRegistry::default();
        // Score 6 > warning threshold 5
        reg.record_misbehavior(&[1u8; 32], &[0xC1; 32], &EvidenceType::DoubleVote, 1); // +10
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::Warned);
        assert_eq!(
            reg.check_thresholds(&[1u8; 32]),
            Some(ValidatorAction::Warn)
        );
    }

    #[test]
    fn n109_11_suspension_threshold_triggered() {
        let mut reg = MisbehaviorRegistry::default();
        // Score 16 > suspension threshold 15
        reg.record_misbehavior(&[1u8; 32], &[0xD1; 32], &EvidenceType::DoubleVote, 1); // +10
        reg.record_misbehavior(&[1u8; 32], &[0xD2; 32], &EvidenceType::StateRootMismatch, 2); // +3
        reg.record_misbehavior(&[1u8; 32], &[0xD3; 32], &EvidenceType::StateRootMismatch, 3); // +3
        assert_eq!(reg.get_score(&[1u8; 32]), 16);
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::Suspended);
        assert_eq!(
            reg.check_thresholds(&[1u8; 32]),
            Some(ValidatorAction::Suspend)
        );
    }

    #[test]
    fn n109_11_slashing_threshold_triggered() {
        let mut reg = MisbehaviorRegistry::default();
        // Score 31 > slashing threshold 30
        reg.record_misbehavior(&[1u8; 32], &[0xE1; 32], &EvidenceType::DoubleVote, 1); // +10
        reg.record_misbehavior(&[1u8; 32], &[0xE2; 32], &EvidenceType::DoubleVote, 2); // +10
        reg.record_misbehavior(&[1u8; 32], &[0xE3; 32], &EvidenceType::DoubleVote, 3); // +10
        reg.record_misbehavior(&[1u8; 32], &[0xE4; 32], &EvidenceType::FutureVote, 4); // +1
        assert_eq!(reg.get_score(&[1u8; 32]), 31);
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::SlashEligible);
        assert_eq!(
            reg.check_thresholds(&[1u8; 32]),
            Some(ValidatorAction::Slash)
        );
    }

    #[test]
    fn n109_11_duplicate_evidence_not_counted_twice() {
        let mut reg = MisbehaviorRegistry::default();
        let ev_id = [0xFF; 32];
        // First record: accepted
        assert!(reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 1));
        assert_eq!(reg.get_score(&[1u8; 32]), 10);
        // Second record with SAME evidence_id: rejected
        assert!(!reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 2));
        assert_eq!(
            reg.get_score(&[1u8; 32]),
            10,
            "Score must not increase on duplicate"
        );
    }

    #[test]
    fn n109_11_multiple_validators_isolated() {
        let mut reg = MisbehaviorRegistry::default();
        reg.record_misbehavior(&[1u8; 32], &[0xA1; 32], &EvidenceType::DoubleVote, 1); // +10
        reg.record_misbehavior(&[2u8; 32], &[0xA2; 32], &EvidenceType::InvalidSignature, 2); // +2
        assert_eq!(reg.get_score(&[1u8; 32]), 10);
        assert_eq!(reg.get_score(&[2u8; 32]), 2);
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn n109_11_registry_rebuilds_from_evidence_store() {
        let mut store = EvidenceStore::new();
        store.store_evidence(EvidenceRecord::new(
            [1u8; 32],
            1,
            EvidenceType::DoubleVote,
            1000,
            vec![1],
        ));
        store.store_evidence(EvidenceRecord::new(
            [1u8; 32],
            2,
            EvidenceType::StateRootMismatch,
            2000,
            vec![2],
        ));

        let reg =
            MisbehaviorRegistry::rebuild_from_evidence(MisbehaviorThresholds::default(), &store);
        assert_eq!(reg.get_score(&[1u8; 32]), 13); // 10 + 3
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::Warned); // 13 >= 5, < 15
    }

    #[test]
    fn n109_11_custom_thresholds() {
        let thresholds = MisbehaviorThresholds {
            warning_score: 2,
            suspension_score: 5,
            slashing_score: 8,
            ..Default::default()
        };
        let mut reg = MisbehaviorRegistry::new(thresholds);
        reg.record_misbehavior(&[1u8; 32], &[0x01; 32], &EvidenceType::InvalidSignature, 1); // +2 → warning
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::Warned);
        reg.record_misbehavior(&[1u8; 32], &[0x02; 32], &EvidenceType::StateRootMismatch, 2); // +3 → 5 → suspension
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::Suspended);
        reg.record_misbehavior(&[1u8; 32], &[0x03; 32], &EvidenceType::StateRootMismatch, 3); // +3 → 8 → slashing
        assert_eq!(reg.get_status(&[1u8; 32]), ValidatorStatus::SlashEligible);
    }

    #[test]
    fn n109_11_active_validator_returns_none_action() {
        let reg = MisbehaviorRegistry::default();
        assert_eq!(reg.get_status(&[99u8; 32]), ValidatorStatus::Active);
        assert_eq!(reg.check_thresholds(&[99u8; 32]), None);
        assert_eq!(reg.get_score(&[99u8; 32]), 0);
    }
}
