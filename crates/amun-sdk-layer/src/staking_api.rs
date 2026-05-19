use amun_kernel_types::PublicKey;
use amun_staking::validator::ValidatorRegistry;
use amun_staking::delegation::DelegationManager;
use amun_staking::slashing::SlashingConditions;
use crate::types::SdkResult;

pub struct StakingApi {
    pub registry: ValidatorRegistry,
    pub delegation: DelegationManager,
    pub slashing: SlashingConditions,
}

impl StakingApi {
    pub fn new() -> Self {
        Self { registry: ValidatorRegistry::new(), delegation: DelegationManager::new(), slashing: SlashingConditions::new() }
    }

    pub fn register_validator(&mut self, pubkey: PublicKey, stake: u64) -> SdkResult<()> {
        match self.registry.register(pubkey, stake) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Registration failed"),
        }
    }

    pub fn delegate(&mut self, validator_stake: &mut u64, amount: u64) -> SdkResult<()> {
        match self.delegation.delegate(validator_stake, amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Delegation failed"),
        }
    }

    pub fn slash_validator(&mut self, pubkey: &PublicKey) -> SdkResult<u64> {
        match self.registry.slash(pubkey, &self.slashing) {
            Ok(amount) => SdkResult::ok(amount),
            Err(_) => SdkResult::err("Slash failed"),
        }
    }

    pub fn get_validator_count(&self) -> SdkResult<usize> {
        SdkResult::ok(self.registry.active_count)
    }

    pub fn get_total_stake(&self) -> SdkResult<u64> {
        SdkResult::ok(self.registry.total_stake)
    }
}
