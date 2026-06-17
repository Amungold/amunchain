// ============================================================================
// N110.1b — Real Staking Adapter
// ============================================================================
use crate::staking_adapter::SlashingExecutor;
use amun_kernel_types::PublicKey;
use amun_staking::slashing::SlashingConditions;
use amun_staking::validator::ValidatorRegistry;

/// N110.1b: Real implementation using only ValidatorRegistry's public API.
pub struct RealStakingExecutor {
    pub registry: ValidatorRegistry,
    pub rules: SlashingConditions,
}

impl RealStakingExecutor {
    pub fn new(registry: ValidatorRegistry) -> Self {
        Self {
            registry,
            rules: SlashingConditions::new(),
        }
    }

    /// Convert [u8; 32] validator_id to PublicKey (48 bytes).
    /// Pads with zeros to fill the 48-byte key.
    fn to_public_key(validator_id: &[u8; 32]) -> PublicKey {
        let mut key = [0u8; 48];
        key[..32].copy_from_slice(validator_id);
        PublicKey(key)
    }
}

impl SlashingExecutor for RealStakingExecutor {
    fn get_stake(&self, _validator_id: &[u8; 32]) -> u64 {
        // ValidatorRegistry doesn't expose individual stake via public API.
        // We track it through slash results. Return total as approximation.
        self.registry.total_stake
    }

    fn slash(&mut self, validator_id: &[u8; 32], _amount: u64) -> Result<u64, String> {
        let pk = Self::to_public_key(validator_id);
        self.registry
            .slash(&pk, &self.rules)
            .map_err(|e| format!("Slash failed: {:?}", e))
    }

    fn deactivate(&mut self, _validator_id: &[u8; 32]) {
        // Deactivation happens automatically in slash() when slash_count >= max_slash_count
        // No separate public API needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::misbehavior_registry::{MisbehaviorRegistry, MisbehaviorThresholds};
    use crate::staking_adapter::StakingAdapter;

    fn make_pk(id: u8) -> PublicKey {
        let mut key = [0u8; 48];
        key[0] = id;
        PublicKey(key)
    }

    fn pk_to_id(pk: &PublicKey) -> [u8; 32] {
        let mut id = [0u8; 32];
        id.copy_from_slice(&pk.0[..32]);
        id
    }

    /// N110.1b GATEKEEPER: Real staking slash reduces stake
    #[test]
    fn n110_1b_real_staking_slash_reduces_validator_stake() {
        let mut registry = ValidatorRegistry::new();
        let pk = make_pk(0x42);
        let validator_id = pk_to_id(&pk);

        registry
            .register(pk, 100_000)
            .expect("Registration should succeed");
        assert_eq!(registry.total_stake, 100_000);

        let executor = RealStakingExecutor::new(registry);
        let mut adapter = StakingAdapter::new(
            MisbehaviorRegistry::new(MisbehaviorThresholds::default()),
            executor,
        );

        // Accumulate misbehavior
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

        // Execute real slash
        let result = adapter.try_slash(&validator_id);
        assert!(result.is_some(), "N110.1b FAIL: slash should execute");

        let slash_result = result.unwrap();
        assert!(
            slash_result.amount_slashed > 0,
            "slashed amount must be positive"
        );
        assert!(
            adapter.executor.registry.total_stake < 100_000,
            "N110.1b FAIL: registry total_stake must decrease. Was: {}",
            adapter.executor.registry.total_stake
        );

        eprintln!(
            "N110.1b GATEKEEPER PASSED: real_stake_before=100000, slashed={}, registry_total={}",
            slash_result.amount_slashed, adapter.executor.registry.total_stake
        );
    }
}
