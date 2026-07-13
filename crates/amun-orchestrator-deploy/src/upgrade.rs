/// Upgrade plan generated before execution.
#[derive(Debug, Clone)]
pub struct UpgradePlan {
    pub validators_to_upgrade: Vec<String>,
    pub estimated_duration_secs: u64,
    pub can_rollback: bool,
}

impl UpgradePlan {
    /// Create an upgrade plan for a set of validators.
    pub fn new(validator_names: Vec<String>) -> Self {
        let count = validator_names.len() as u64;
        Self {
            validators_to_upgrade: validator_names,
            estimated_duration_secs: count * 15, // ~15s per validator
            can_rollback: true,
        }
    }
}
