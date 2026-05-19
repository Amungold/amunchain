use amun_validator_attestation::ValidatorSet;

#[derive(Debug, Clone)]
pub struct LeaderSchedule {
    validator_set: ValidatorSet,
}

impl LeaderSchedule {
    pub fn new(validator_set: ValidatorSet) -> Self {
        Self { validator_set }
    }

    pub fn leader_for_round(&self, round: u64) -> u64 {
        let count = self.validator_set.validator_count().max(1) as u64;
        let idx = (round % count) as usize;
        self.validator_set.validators.get(idx)
            .map(|v| v.id)
            .unwrap_or(1)
    }

    pub fn is_leader(&self, validator_id: u64, round: u64) -> bool {
        self.leader_for_round(round) == validator_id
    }
}
