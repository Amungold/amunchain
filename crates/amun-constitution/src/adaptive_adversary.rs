// Adaptive corruption model for Byzantine adversary.
// Models corruption that can occur DURING protocol execution.

#[derive(Clone, Copy, Debug)]
pub struct AdaptiveCorruptionConstraints {
    pub max_corrupt_stake: u64,
    pub corruption_latency_rounds: u32,
    pub slashability_window_epochs: u64,
    pub unbonding_delay_epochs: u64,
}

impl AdaptiveCorruptionConstraints {
    pub const fn constitutional() -> Self {
        Self {
            max_corrupt_stake: 0,
            corruption_latency_rounds: 1,
            slashability_window_epochs: 2,
            unbonding_delay_epochs: 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidatorCorruptionState {
    Honest,
    Corrupted {
        corrupted_at_round: u64,
    },
    Slashed {
        slashed_at_round: u64,
        frozen_until_epoch: u64,
    },
}

impl ValidatorCorruptionState {
    pub fn is_honest(&self) -> bool {
        matches!(self, Self::Honest)
    }

    pub fn can_participate(&self) -> bool {
        match self {
            Self::Honest => true,
            Self::Corrupted { .. } => true,
            Self::Slashed { .. } => false,
        }
    }
}
