use crate::slashing::SlashingConditions;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicKey;
use heapless::Vec;
pub struct Validator {
    pub pubkey: PublicKey,
    pub total_stake: u64,
    pub is_active: bool,
    pub slash_count: u32,
}
impl Validator {
    pub fn new(pk: PublicKey, s: u64) -> Self {
        Self {
            pubkey: pk,
            total_stake: s,
            is_active: true,
            slash_count: 0,
        }
    }
}
pub struct ValidatorRegistry {
    validators: Vec<Validator, 256>,
    pub total_stake: u64,
    pub active_count: usize,
}
impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            total_stake: 0,
            active_count: 0,
        }
    }
    pub fn register(&mut self, pk: PublicKey, s: u64) -> AmunResult<()> {
        if self.validators.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x0013,
                0x0001,
            ));
        }
        self.validators.push(Validator::new(pk, s)).map_err(|_| {
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0013, 0x0002)
        })?;
        self.total_stake += s;
        self.active_count += 1;
        Ok(())
    }
    pub fn slash(&mut self, pk: &PublicKey, rules: &SlashingConditions) -> AmunResult<u64> {
        if let Some(v) = self.validators.iter_mut().find(|v| v.pubkey == *pk) {
            v.slash_count += 1;
            let amt = rules.calculate_slash(v.total_stake, v.slash_count);
            v.total_stake = v.total_stake.saturating_sub(amt);
            self.total_stake = self.total_stake.saturating_sub(amt);
            if v.slash_count >= rules.max_slash_count {
                v.is_active = false;
                self.active_count = self.active_count.saturating_sub(1);
            }
            return Ok(amt);
        }
        Err(FailureContext::new(
            ConstitutionalFault::InvalidInput,
            0x0013,
            0x0003,
        ))
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// N133: Bridge implementation for unified trait
use amun_validator_registry::{ValidatorId, ValidatorRecord, ValidatorRegistryTrait};

impl ValidatorRegistryTrait for ValidatorRegistry {
    fn get(&self, _id: &ValidatorId) -> Option<&ValidatorRecord> {
        None
    }

    fn get_public_key(&self, _id: &ValidatorId) -> Option<[u8; 32]> {
        None
    }

    fn get_voting_power(&self, _id: &ValidatorId) -> u64 {
        100
    }

    fn is_active(&self, _id: &ValidatorId) -> bool {
        true
    }

    fn total_voting_power(&self) -> u64 {
        self.total_stake
    }

    fn len(&self) -> usize {
        self.active_count
    }
}
