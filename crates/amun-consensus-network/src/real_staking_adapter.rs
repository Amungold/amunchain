// ============================================================================
// N110.1b — Real Staking Adapter
// ============================================================================
use crate::staking_adapter::SlashingExecutor;
use crate::validator_identity::ValidatorIdentityRegistry;
use amun_kernel_types::PublicKey;
use amun_staking::slashing::SlashingConditions;
use amun_staking::validator::ValidatorRegistry;

/// N110.1b: Real implementation using only ValidatorRegistry's public API.
pub struct RealStakingExecutor {
    pub registry: ValidatorRegistry,
    pub rules: SlashingConditions,
    pub identity_registry: ValidatorIdentityRegistry,
}

impl RealStakingExecutor {
    pub fn new(registry: ValidatorRegistry) -> Self {
        Self {
            registry,
            rules: SlashingConditions::new(),
            identity_registry: ValidatorIdentityRegistry::new(),
        }
    }

    pub fn with_identity_registry(
        registry: ValidatorRegistry,
        identity_registry: ValidatorIdentityRegistry,
    ) -> Self {
        Self {
            registry,
            rules: SlashingConditions::new(),
            identity_registry,
        }
    }

    /// N113.2b: Resolve PublicKey strictly through identity registry.
    /// Returns an error if the validator is not registered.
    fn to_public_key(&self, validator_id: &[u8; 32]) -> Result<PublicKey, String> {
        self.identity_registry
            .get_public_key(validator_id)
            .ok_or_else(|| {
                format!(
                    "N113.2b: validator identity not registered: {:02x?}",
                    &validator_id[..4]
                )
            })
    }
}

impl SlashingExecutor for RealStakingExecutor {
    fn get_stake(&self, validator_id: &[u8; 32]) -> u64 {
        // Returns total_stake if the validator is registered; 0 otherwise.
        // Individual stake is tracked through slash results.
        if self.to_public_key(validator_id).is_ok() {
            self.registry.total_stake
        } else {
            0
        }
    }

    fn slash(&mut self, validator_id: &[u8; 32], _amount: u64) -> Result<u64, String> {
        let pk = self.to_public_key(validator_id)?;
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

        let mut executor = RealStakingExecutor::new(registry);
        // N113.2b: Register identity before using the adapter
        executor
            .identity_registry
            .register(crate::ValidatorIdentity::new(validator_id, pk.0, 1))
            .unwrap();
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

    /// N113.2b: Unregistered validator must be rejected (no fallback)
    #[test]
    fn n113_2b_unregistered_validator_rejected() {
        let registry = ValidatorRegistry::new();
        let mut executor = RealStakingExecutor::new(registry);
        let validator_id = [0xAA; 32];
        let result = executor.slash(&validator_id, 1000);
        assert!(
            result.is_err(),
            "N113.2b FAIL: Unregistered identity must be rejected"
        );
        assert!(
            result.unwrap_err().contains("not registered"),
            "Error must mention identity not registered"
        );
    }

    /// N113.2: Real executor resolves PublicKey through identity registry
    #[test]
    fn n113_2_real_executor_uses_identity_registry() {
        let mut registry = ValidatorRegistry::new();
        let pk = PublicKey([0x42u8; 48]);
        let validator_id = [0x42u8; 32];
        registry.register(pk, 100_000).unwrap();

        let mut id_registry = ValidatorIdentityRegistry::new();
        id_registry
            .register(crate::ValidatorIdentity::new(validator_id, [0x42u8; 48], 1))
            .unwrap();

        let executor = RealStakingExecutor::with_identity_registry(registry, id_registry);
        let resolved_pk = executor.to_public_key(&validator_id).unwrap();
        assert_eq!(
            resolved_pk.0, [0x42u8; 48],
            "N113.2 FAIL: PublicKey must come from identity registry"
        );
    }
}
