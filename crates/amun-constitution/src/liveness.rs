// Logical-time liveness parameters. No wall clock.

#[derive(Clone, Copy, Debug)]
pub struct LogicalLivenessParameters {
    pub prepare_timeout_rounds: u32,
    pub precommit_timeout_rounds: u32,
    pub commit_timeout_rounds: u32,
    pub max_timeout_retries: u32,
    pub escalation_factor: u32,
}

impl LogicalLivenessParameters {
    pub const fn constitutional() -> Self {
        Self {
            prepare_timeout_rounds: 3,
            precommit_timeout_rounds: 3,
            commit_timeout_rounds: 3,
            max_timeout_retries: 10,
            escalation_factor: 2,
        }
    }

    pub fn timeout_rounds_for_retry(&self, retry: u32) -> u32 {
        let factor = self.escalation_factor.saturating_pow(retry);
        self.prepare_timeout_rounds.saturating_mul(factor)
    }

    pub fn should_timeout(&self, started_at: u64, current: u64, retry: u32) -> bool {
        let rounds = self.timeout_rounds_for_retry(retry) as u64;
        current >= started_at.saturating_add(rounds)
    }
}

#[derive(Clone, Debug)]
pub struct TimeoutCertificate {
    pub requested_at_round: u64,
    pub retry: u32,
    pub fired_at_round: u64,
}

impl TimeoutCertificate {
    pub fn verify(&self, params: &LogicalLivenessParameters) -> Result<(), &'static str> {
        let expected = params.timeout_rounds_for_retry(self.retry) as u64;
        let actual = self.fired_at_round.saturating_sub(self.requested_at_round);
        if actual >= expected {
            Ok(())
        } else {
            Err("Timeout fired too early")
        }
    }
}
