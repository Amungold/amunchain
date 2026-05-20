use amun_constitution::liveness::LogicalLivenessParameters;

/// Constitutional liveness rules.
/// Ensures the protocol makes progress under partial synchrony.
pub struct LivenessRules {
    params: LogicalLivenessParameters,
}

impl LivenessRules {
    pub fn new() -> Self {
        Self {
            params: LogicalLivenessParameters::constitutional(),
        }
    }

    /// Determine if a round has timed out based on logical rounds
    pub fn is_timeout(&self, started_at_round: u64, current_round: u64, retry: u32) -> bool {
        self.params
            .should_timeout(started_at_round, current_round, retry)
    }

    /// Get the timeout duration for a given retry count
    pub fn timeout_rounds(&self, retry: u32) -> u32 {
        self.params.timeout_rounds_for_retry(retry)
    }

    /// Get the leader for a round
    pub fn get_leader(&self, round: u64, validator_count: usize) -> usize {
        if validator_count == 0 {
            return 0;
        }
        (round as usize) % validator_count
    }

    /// Get exponential backoff timeout in rounds
    pub fn exponential_backoff_rounds(&self, retry: u32) -> u32 {
        // 2^retry capped at 256 rounds
        let exp = 1u32.checked_shl(retry.min(8)).unwrap_or(256);
        exp.min(256)
    }
}

impl Default for LivenessRules {
    fn default() -> Self {
        Self::new()
    }
}
