// Weighted stake quorum algebra for proof-of-stake systems.

use amun_kernel_types::ValidatorId;
use heapless::Vec;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StakeWeight(pub u64);

#[derive(Clone, Copy, Debug)]
pub struct WeightedValidator {
    pub id: ValidatorId,
    pub weight: StakeWeight,
}

#[derive(Clone, Debug)]
pub struct WeightedValidatorSet {
    pub validators: Vec<WeightedValidator, 256>,
    pub total_stake: u64,
}

impl WeightedValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            total_stake: 0,
        }
    }

    pub fn byzantine_threshold_stake(&self) -> u64 {
        if self.total_stake == 0 {
            return 0;
        }
        (self.total_stake - 1) / 3
    }

    pub fn quorum_threshold_stake(&self) -> u64 {
        2 * self.byzantine_threshold_stake() + 1
    }

    pub fn intersection_stake(&self, other: &Self) -> u64 {
        let mut intersection = 0u64;
        for v in self.validators.iter() {
            if let Some(other_v) = other.validators.iter().find(|o| o.id == v.id) {
                intersection += v.weight.0.min(other_v.weight.0);
            }
        }
        intersection
    }

    pub fn verify_weighted_transition(&self, new_set: &Self) -> Result<(), &'static str> {
        let f_old = self.byzantine_threshold_stake();
        let f_new = new_set.byzantine_threshold_stake();
        let required = f_old.max(f_new) + 1;
        let actual = self.intersection_stake(new_set);
        if actual >= required {
            Ok(())
        } else {
            Err("Insufficient stake overlap for safe transition")
        }
    }

    pub fn add_validator(
        &mut self,
        id: ValidatorId,
        weight: StakeWeight,
    ) -> Result<(), &'static str> {
        if self.validators.is_full() {
            return Err("Validator set is full");
        }
        self.validators
            .push(WeightedValidator { id, weight })
            .map_err(|_| "Push failed")?;
        self.total_stake += weight.0;
        Ok(())
    }
}

impl Default for WeightedValidatorSet {
    fn default() -> Self {
        Self::new()
    }
}
