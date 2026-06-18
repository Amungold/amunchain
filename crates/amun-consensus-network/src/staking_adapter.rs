// ============================================================================
// N110.1 — Staking Adapter
// ============================================================================
// Bridges N109 MisbehaviorRegistry → Economic Slashing.
//
// Design decisions:
//   - Self-contained: does NOT depend on amun-staking (which is no_std)
//   - Uses a trait for testability and future integration with amun-staking
//   - Translates MisbehaviorScore → offense_count → penalty_bps → amount
//
// In N110.2, the real amun-staking::ValidatorRegistry will be plugged in
// via amun-sdk-layer. For now, a test implementation proves the pipeline.
// ============================================================================

use crate::misbehavior_registry::{MisbehaviorRegistry, ValidatorAction, ValidatorStatus};

/// N110.1: Result of executing a slash on a validator's stake
#[derive(Debug, Clone, PartialEq)]
pub struct SlashResult {
    pub validator_id: [u8; 32],
    pub amount_slashed: u64,
    pub remaining_stake: u64,
    pub penalty_bps: u64,
    pub offense_count: u32,
    pub is_active: bool,
}

/// N110.1: Trait for executing slashing on real stake.
/// In production, this will be implemented by amun-staking::ValidatorRegistry.
/// In tests, a simulated implementation verifies the pipeline.
pub trait SlashingExecutor {
    fn get_stake(&self, validator_id: &[u8; 32]) -> u64;
    fn slash(&mut self, validator_id: &[u8; 32], amount: u64) -> Result<u64, String>;
    fn deactivate(&mut self, validator_id: &[u8; 32]);
}

/// N110.1: Adapter that connects MisbehaviorRegistry to SlashingExecutor.
pub struct StakingAdapter<E: SlashingExecutor> {
    pub registry: MisbehaviorRegistry,
    pub executor: E,
    pub base_penalty_bps: u64,
}

impl<E: SlashingExecutor> StakingAdapter<E> {
    pub fn new(registry: MisbehaviorRegistry, executor: E) -> Self {
        Self {
            registry,
            executor,
            base_penalty_bps: 500, // 5% base penalty
        }
    }

    /// N110.1: Check if a validator is slashable and execute the slash.
    ///
    /// Returns SlashResult if slashing was executed, None if validator
    /// hasn't crossed the slashing threshold.
    pub fn try_slash(&mut self, validator_id: &[u8; 32]) -> Option<SlashResult> {
        // Check if validator crossed slashing threshold
        match self.registry.check_thresholds(validator_id) {
            Some(ValidatorAction::Slash) => Some(self.execute_slash(validator_id)),
            _ => None,
        }
    }

    /// N118.2: The unified entry point for slashing.
    /// Requires finality + signature + quorum before executing the slash.
    pub fn execute_after_finality(
        &mut self,
        validator_id: &[u8; 32],
        cert: &crate::MultiSignerCertificate,
        finalized_height: u64,
    ) -> Result<Option<SlashResult>, String> {
        if !crate::finality_gate::is_certificate_finalized(cert, finalized_height) {
            return Err("N118.2: certificate not finalized".to_string());
        }
        if cert.certificate.signature == [0u8; 64] {
            return Err("N118.2: unsigned certificate".into());
        }
        cert.certificate
            .verify_signature()
            .map_err(|e| format!("N118.2: signature: {}", e))?;
        if !cert.has_quorum()? {
            return Err("N118.2: quorum not reached".into());
        }
        match self.registry.check_thresholds(validator_id) {
            Some(ValidatorAction::Slash) => Ok(Some(self.execute_slash(validator_id))),
            _ => Ok(None),
        }
    }

    /// N115: Execute slash only if the certificate carries a valid signature.
    /// Returns an error if the certificate is unsigned or tampered.
    pub fn try_slash_with_certificate(
        &mut self,
        validator_id: &[u8; 32],
        cert: &crate::SlashingCertificate,
    ) -> Result<Option<SlashResult>, String> {
        // N115: Verify certificate signature before executing slash
        if cert.signature == [0u8; 64] {
            return Err("N115: unsigned certificate rejected".into());
        }
        cert.verify_signature()
            .map_err(|e| format!("N115: certificate verification failed: {}", e))?;

        // Check if validator crossed slashing threshold
        match self.registry.check_thresholds(validator_id) {
            Some(ValidatorAction::Slash) => Ok(Some(self.execute_slash(validator_id))),
            _ => Ok(None),
        }
    }

    /// N110.1: Execute the slash unconditionally.
    fn execute_slash(&mut self, validator_id: &[u8; 32]) -> SlashResult {
        let score = self.registry.get_score(validator_id);
        let status = self.registry.get_status(validator_id);
        let current_stake = self.executor.get_stake(validator_id);

        // Translate MisbehaviorScore → offense_count
        // Each 10 points ≈ 1 offense for penalty calculation
        let offense_count = ((score / 10) as u32).clamp(1, 10);

        // Calculate penalty in basis points
        let penalty_bps = (self.base_penalty_bps * offense_count as u64).min(10000);

        // Calculate amount to slash
        let amount = current_stake
            .saturating_mul(penalty_bps)
            .saturating_div(10000);

        // Execute the slash on real stake
        let _ = self.executor.slash(validator_id, amount);

        // Deactivate if status is SlashEligible and this isn't first slash
        if status == ValidatorStatus::SlashEligible && offense_count >= 3 {
            self.executor.deactivate(validator_id);
        }

        let remaining = self.executor.get_stake(validator_id);

        SlashResult {
            validator_id: *validator_id,
            amount_slashed: amount,
            remaining_stake: remaining,
            penalty_bps,
            offense_count,
            is_active: status != ValidatorStatus::SlashEligible || offense_count < 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::misbehavior_registry::MisbehaviorRegistry;
    use crate::misbehavior_registry::MisbehaviorThresholds;
    use crate::ValidatorStatus;
    use std::collections::HashMap;

    /// N110.1: Simulated staking implementation for testing
    struct SimulatedStaking {
        stakes: HashMap<[u8; 32], u64>,
    }

    impl SimulatedStaking {
        fn new() -> Self {
            Self {
                stakes: HashMap::new(),
            }
        }

        fn set_stake(&mut self, validator_id: [u8; 32], amount: u64) {
            self.stakes.insert(validator_id, amount);
        }
    }

    impl SlashingExecutor for SimulatedStaking {
        fn get_stake(&self, validator_id: &[u8; 32]) -> u64 {
            *self.stakes.get(validator_id).unwrap_or(&0)
        }

        fn slash(&mut self, validator_id: &[u8; 32], amount: u64) -> Result<u64, String> {
            if let Some(stake) = self.stakes.get_mut(validator_id) {
                *stake = stake.saturating_sub(amount);
                Ok(amount)
            } else {
                Err("Validator not found".into())
            }
        }

        fn deactivate(&mut self, _validator_id: &[u8; 32]) {
            // In real implementation, sets validator.is_active = false
        }
    }

    /// N110.1 GATEKEEPER: Slashing reduces real validator stake
    #[test]
    fn n110_1_slashing_reduces_real_validator_stake() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0x42; 32];
        staking.set_stake(validator_id, 100_000); // 100k initial stake

        let registry =
            MisbehaviorRegistry::new(crate::misbehavior_registry::MisbehaviorThresholds::default());
        let mut adapter = StakingAdapter::new(registry, staking);

        // Accumulate misbehavior until slashing threshold
        // 3 DoubleVotes (weight 10 each) = score 30 → Slashing
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xA1; 32],
            &EvidenceType::DoubleVote,
            1,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xA2; 32],
            &EvidenceType::DoubleVote,
            2,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xA3; 32],
            &EvidenceType::DoubleVote,
            3,
        );

        assert_eq!(adapter.registry.get_score(&validator_id), 30);
        assert_eq!(
            adapter.registry.get_status(&validator_id),
            ValidatorStatus::SlashEligible
        );

        // Execute slash
        let result = adapter.try_slash(&validator_id);
        assert!(
            result.is_some(),
            "N110.1 FAIL: should execute slash when threshold crossed"
        );

        let slash_result = result.unwrap();
        assert_eq!(slash_result.validator_id, validator_id);
        assert!(
            slash_result.amount_slashed > 0,
            "N110.1 FAIL: slash amount must be positive"
        );
        assert!(
            slash_result.remaining_stake < 100_000,
            "N110.1 FAIL: remaining stake ({}) must be less than initial (100000)",
            slash_result.remaining_stake
        );
        assert_eq!(
            slash_result.remaining_stake,
            100_000 - slash_result.amount_slashed,
            "N110.1 FAIL: stake accounting mismatch"
        );

        eprintln!("N110.1 GATEKEEPER PASSED: initial=100000, slashed={}, remaining={}, penalty={}bps, offenses={}",
            slash_result.amount_slashed,
            slash_result.remaining_stake,
            slash_result.penalty_bps,
            slash_result.offense_count,
        );
    }

    /// N110.1: Validator below threshold is not slashed
    #[test]
    fn n110_1_validator_below_threshold_not_slashed() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0xAA; 32];
        staking.set_stake(validator_id, 50_000);

        let registry =
            MisbehaviorRegistry::new(crate::misbehavior_registry::MisbehaviorThresholds::default());
        let mut adapter = StakingAdapter::new(registry, staking);

        // Only one offense — score=10, below slashing threshold (30)
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xB1; 32],
            &EvidenceType::DoubleVote,
            1,
        );

        let result = adapter.try_slash(&validator_id);
        assert!(
            result.is_none(),
            "N110.1 FAIL: should not slash validator below threshold"
        );
        assert_eq!(
            adapter.executor.get_stake(&validator_id),
            50_000,
            "N110.1 FAIL: stake should not change when not slashed"
        );
    }

    /// N110.1: Multiple slashes accumulate correctly
    #[test]
    fn n110_1_multiple_slashes_accumulate() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0xCC; 32];
        staking.set_stake(validator_id, 200_000);

        let registry =
            MisbehaviorRegistry::new(crate::misbehavior_registry::MisbehaviorThresholds::default());
        let mut adapter = StakingAdapter::new(registry, staking);

        // First set of offenses → first slash
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD1; 32],
            &EvidenceType::DoubleVote,
            1,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD2; 32],
            &EvidenceType::DoubleVote,
            2,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD3; 32],
            &EvidenceType::DoubleVote,
            3,
        );

        let result1 = adapter.try_slash(&validator_id).unwrap();
        assert!(result1.amount_slashed > 0);

        // More offenses → score increases → should be slashable again
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD4; 32],
            &EvidenceType::DoubleVote,
            4,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD5; 32],
            &EvidenceType::DoubleVote,
            5,
        );
        adapter.registry.record_misbehavior(
            &validator_id,
            &[0xD6; 32],
            &EvidenceType::DoubleVote,
            6,
        );

        let result2 = adapter.try_slash(&validator_id).unwrap();
        assert!(result2.amount_slashed > 0);
        assert!(
            result2.offense_count > result1.offense_count,
            "Second slash should have higher offense_count"
        );
        assert!(
            result2.remaining_stake < result1.remaining_stake,
            "Stake should decrease further on second slash"
        );
    }

    /// N110.1: Different validators have independent slashing
    #[test]
    fn n110_1_different_validators_independent() {
        let mut staking = SimulatedStaking::new();
        staking.set_stake([1u8; 32], 100_000);
        staking.set_stake([2u8; 32], 100_000);

        let registry =
            MisbehaviorRegistry::new(crate::misbehavior_registry::MisbehaviorThresholds::default());
        let mut adapter = StakingAdapter::new(registry, staking);

        // Validator 1: commit offenses
        adapter
            .registry
            .record_misbehavior(&[1u8; 32], &[0xE1; 32], &EvidenceType::DoubleVote, 1);
        adapter
            .registry
            .record_misbehavior(&[1u8; 32], &[0xE2; 32], &EvidenceType::DoubleVote, 2);
        adapter
            .registry
            .record_misbehavior(&[1u8; 32], &[0xE3; 32], &EvidenceType::DoubleVote, 3);

        // Validator 1 should be slashed
        let r1 = adapter.try_slash(&[1u8; 32]);
        assert!(r1.is_some());

        // Validator 2 should NOT be slashed (no offenses)
        let r2 = adapter.try_slash(&[2u8; 32]);
        assert!(r2.is_none());

        // Validator 2's stake should be intact
        assert_eq!(adapter.executor.get_stake(&[2u8; 32]), 100_000);
    }

    /// N115: Unsigned certificate is rejected before slash execution
    #[test]
    fn n115_unsigned_certificate_rejected_before_slash() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0x42; 32];
        staking.set_stake(validator_id, 100_000);

        let mut registry = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
        registry.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
        registry.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
        registry.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);

        let mut adapter = StakingAdapter::new(registry, staking);

        // Create an unsigned certificate
        let cert = crate::SlashingCertificate::from_slash_result(
            validator_id,
            30,
            vec![[0xA1; 32]],
            vec![],
            1500,
            15000,
            85000,
            3,
            crate::ValidatorStatus::SlashEligible,
            100,
        );

        let result = adapter.try_slash_with_certificate(&validator_id, &cert);
        assert!(
            result.is_err(),
            "N115 FAIL: Unsigned certificate must be rejected"
        );
        assert!(result.unwrap_err().contains("unsigned"));
    }

    /// N115: Signed certificate executes slash successfully
    #[test]
    fn n115_signed_certificate_executes_slash() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0x42; 32];
        staking.set_stake(validator_id, 100_000);

        let mut registry = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
        registry.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
        registry.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
        registry.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);

        let mut adapter = StakingAdapter::new(registry, staking);

        // Create and sign a certificate
        let mut cert = crate::SlashingCertificate::from_slash_result(
            validator_id,
            30,
            vec![[0xA1; 32]],
            vec![],
            1500,
            15000,
            85000,
            3,
            crate::ValidatorStatus::SlashEligible,
            100,
        );
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&[0x42; 32]);
        cert.sign(&signing_key);

        let result = adapter.try_slash_with_certificate(&validator_id, &cert);
        assert!(
            result.is_ok(),
            "N115 FAIL: Signed certificate must be accepted"
        );
        let slash_result = result.unwrap();
        assert!(
            slash_result.is_some(),
            "N115 FAIL: Slash must execute with signed certificate"
        );
        assert!(slash_result.unwrap().amount_slashed > 0);
    }

    /// N115: Tampered certificate is rejected
    #[test]
    fn n115_tampered_certificate_rejected_before_slash() {
        let mut staking = SimulatedStaking::new();
        let validator_id = [0x42; 32];
        staking.set_stake(validator_id, 100_000);

        let mut registry = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
        registry.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
        registry.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
        registry.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);

        let mut adapter = StakingAdapter::new(registry, staking);

        let mut cert = crate::SlashingCertificate::from_slash_result(
            validator_id,
            30,
            vec![[0xA1; 32]],
            vec![],
            1500,
            15000,
            85000,
            3,
            crate::ValidatorStatus::SlashEligible,
            100,
        );
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&[0x42; 32]);
        cert.sign(&signing_key);

        // Tamper after signing
        cert.amount_slashed = 1;

        let result = adapter.try_slash_with_certificate(&validator_id, &cert);
        assert!(
            result.is_err(),
            "N115 FAIL: Tampered certificate must be rejected"
        );
        assert!(result.unwrap_err().contains("verification failed"));
    }

    #[test]
    fn n118_2a_unfinalized_certificate_rejected() {
        let mut staking = SimulatedStaking::new();
        let vid = [0x42; 32];
        staking.set_stake(vid, 100_000);
        let mut reg = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
        reg.record_misbehavior(&vid, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
        reg.record_misbehavior(&vid, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
        reg.record_misbehavior(&vid, &[0xA3; 32], &EvidenceType::DoubleVote, 3);
        let mut adapter = StakingAdapter::new(reg, staking);
        let mut cert = crate::MultiSignerCertificate::new(
            crate::SlashingCertificate::from_slash_result(
                vid,
                30,
                vec![[0xA1; 32]],
                vec![],
                1500,
                15000,
                85000,
                3,
                crate::ValidatorStatus::SlashEligible,
                100,
            ),
            3,
            5,
        );
        let k = ed25519_dalek::SigningKey::from_bytes(&[0x42; 32]);
        cert.add_approval(k.verifying_key().to_bytes(), &k).unwrap();
        cert.certificate.sign(&k);
        let r = adapter.execute_after_finality(&vid, &cert, 99);
        assert!(
            r.is_err() && r.unwrap_err().contains("not finalized"),
            "N118.2a FAIL"
        );
    }

    #[test]
    fn n118_2b_finalized_certificate_executes() {
        let mut staking = SimulatedStaking::new();
        let vid = [0x42; 32];
        staking.set_stake(vid, 100_000);
        let mut reg = MisbehaviorRegistry::new(MisbehaviorThresholds::default());
        reg.record_misbehavior(&vid, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
        reg.record_misbehavior(&vid, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
        reg.record_misbehavior(&vid, &[0xA3; 32], &EvidenceType::DoubleVote, 3);
        let mut adapter = StakingAdapter::new(reg, staking);
        let mut cert = crate::MultiSignerCertificate::new(
            crate::SlashingCertificate::from_slash_result(
                vid,
                30,
                vec![[0xA1; 32]],
                vec![],
                1500,
                15000,
                85000,
                3,
                crate::ValidatorStatus::SlashEligible,
                100,
            ),
            3,
            5,
        );
        // N118: Sign certificate FIRST so signing_bytes is stable for approvals
        let sk = ed25519_dalek::SigningKey::from_bytes(&[0x42; 32]);
        cert.certificate.sign(&sk);
        for seed in [1u8, 2, 3] {
            let ak = ed25519_dalek::SigningKey::from_bytes(&[seed; 32]);
            cert.add_approval(ak.verifying_key().to_bytes(), &ak)
                .unwrap();
        }
        let r = adapter.execute_after_finality(&vid, &cert, 100);
        if let Err(ref e) = r {
            eprintln!("N118.2b DEBUG: {}", e);
        }
        assert!(r.is_ok() && r.unwrap().is_some(), "N118.2b FAIL");
    }
}
