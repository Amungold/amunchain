use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Suspended { until_height: u64 },
    Banned,
}

pub struct ValidatorStatusRegistry {
    statuses: HashMap<[u8; 32], ValidatorStatus>,
}

impl ValidatorStatusRegistry {
    pub fn new() -> Self {
        Self {
            statuses: HashMap::new(),
        }
    }
}

impl Default for ValidatorStatusRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorStatusRegistry {}

impl ValidatorStatusRegistry {
    pub fn set_status(&mut self, validator_id: [u8; 32], status: ValidatorStatus) {
        self.statuses.insert(validator_id, status);
    }

    pub fn is_suspended(&self, validator_id: &[u8; 32], current_height: u64) -> bool {
        match self.statuses.get(validator_id) {
            Some(ValidatorStatus::Suspended { until_height }) => current_height < *until_height,
            Some(ValidatorStatus::Banned) => true,
            _ => false,
        }
    }
}
