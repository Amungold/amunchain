/// Recovery strategy for a failed component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Restart the service immediately.
    Restart,
    /// Restart with exponential backoff.
    RestartWithBackoff {
        attempt: u32,
        base_delay_secs: u64,
        max_delay_secs: u64,
    },
    /// Do nothing — requires manual intervention.
    Manual,
    /// Rejoin the network from scratch.
    Rejoin,
}

impl RecoveryStrategy {
    /// Choose a strategy based on failure count.
    pub fn for_failure(failure_count: u32, max_restarts: u32) -> Self {
        if failure_count == 0 {
            return RecoveryStrategy::Restart;
        }
        if failure_count < max_restarts {
            return RecoveryStrategy::RestartWithBackoff {
                attempt: failure_count,
                base_delay_secs: 5,
                max_delay_secs: 300,
            };
        }
        RecoveryStrategy::Manual
    }

    /// Calculate the delay before retrying.
    pub fn delay_secs(&self) -> u64 {
        match self {
            RecoveryStrategy::Restart => 1,
            RecoveryStrategy::RestartWithBackoff {
                attempt,
                base_delay_secs,
                max_delay_secs,
            } => {
                let delay = base_delay_secs * 2u64.pow(*attempt);
                std::cmp::min(delay, *max_delay_secs)
            }
            RecoveryStrategy::Manual => 0,
            RecoveryStrategy::Rejoin => 10,
        }
    }
}

impl std::fmt::Display for RecoveryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryStrategy::Restart => write!(f, "restart"),
            RecoveryStrategy::RestartWithBackoff { attempt, .. } => {
                write!(f, "restart-with-backoff-attempt-{}", attempt)
            }
            RecoveryStrategy::Manual => write!(f, "manual"),
            RecoveryStrategy::Rejoin => write!(f, "rejoin"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_failure_restart() {
        assert_eq!(
            RecoveryStrategy::for_failure(0, 3),
            RecoveryStrategy::Restart
        );
    }

    #[test]
    fn test_repeated_failures_backoff() {
        assert!(matches!(
            RecoveryStrategy::for_failure(2, 5),
            RecoveryStrategy::RestartWithBackoff { .. }
        ));
    }

    #[test]
    fn test_max_failures_manual() {
        assert_eq!(
            RecoveryStrategy::for_failure(5, 3),
            RecoveryStrategy::Manual
        );
    }

    #[test]
    fn test_backoff_delay_increases() {
        let s1 = RecoveryStrategy::RestartWithBackoff {
            attempt: 1,
            base_delay_secs: 5,
            max_delay_secs: 300,
        };
        let s2 = RecoveryStrategy::RestartWithBackoff {
            attempt: 3,
            base_delay_secs: 5,
            max_delay_secs: 300,
        };
        assert!(s2.delay_secs() > s1.delay_secs());
    }
}
