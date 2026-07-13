use amun_validator_attestation::ValidatorSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct LeaderSchedule {
    validators: ValidatorSet,
    seed: u64,
}

impl LeaderSchedule {
    pub fn new(validators: ValidatorSet) -> Self {
        let mut hasher = DefaultHasher::new();
        for id in validators.validator_ids() {
            id.hash(&mut hasher);
            if let Some(v) = validators.get_validator(id) {
                v.stake.hash(&mut hasher);
            }
        }
        let seed = hasher.finish();

        Self { validators, seed }
    }

    pub fn leader_for_round(&self, round: u64) -> u64 {
        let ids = self.validators.validator_ids();
        if ids.is_empty() {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        round.hash(&mut hasher);
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();

        let total_stake = self.validators.total_stake();
        let selection = hash % total_stake;

        let mut cumulative = 0u64;
        for &id in &ids {
            if let Some(v) = self.validators.get_validator(id) {
                cumulative += v.stake;
                if selection < cumulative {
                    return id;
                }
            }
        }

        ids[(round as usize) % ids.len()]
    }
}
