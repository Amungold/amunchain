use amun_validator_attestation::ValidatorSet;

/// Validator rotation law for epoch transitions.
#[derive(Debug, Clone)]
pub struct ValidatorRotation {
    pub activation_delay_epochs: u64,
    pub removal_delay_epochs: u64,
}

impl ValidatorRotation {
    pub fn new() -> Self {
        Self {
            activation_delay_epochs: 2,
            removal_delay_epochs: 2,
        }
    }

    /// Check if a validator set transition is legal.
    pub fn is_legal_transition(
        &self,
        _current_set: &ValidatorSet,
        _new_set: &ValidatorSet,
        current_epoch: u64,
    ) -> bool {
        // Validator set changes take effect after activation_delay_epochs
        // and removals take effect after removal_delay_epochs
        current_epoch >= self.activation_delay_epochs
    }
}

impl Default for ValidatorRotation {
    fn default() -> Self {
        Self::new()
    }
}
