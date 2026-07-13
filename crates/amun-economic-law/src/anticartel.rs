pub struct AntiCartelLaw {
    pub max_stake_per_validator_bps: u16,
    pub min_validator_diversity: u8,
}

impl Default for AntiCartelLaw {
    fn default() -> Self {
        Self::new()
    }
}

impl AntiCartelLaw {
    pub fn new() -> Self {
        Self {
            max_stake_per_validator_bps: 2000,
            min_validator_diversity: 4,
        }
    }

    pub fn is_cartel_risk(&self, stake_bps: u16) -> bool {
        stake_bps > self.max_stake_per_validator_bps
    }

    pub fn enforce_diversity(&self, active_validators: u8) -> Result<(), &'static str> {
        if active_validators < self.min_validator_diversity {
            Err("insufficient validator diversity")
        } else {
            Ok(())
        }
    }
}
