use serde::{Deserialize, Serialize};
use crate::state::ConsensusStep;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FixedMultiplier {
    pub numerator: u64,
    pub denominator: u64,
}

impl FixedMultiplier {
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self { numerator, denominator }
    }

    /// Apply the multiplier to a value using checked arithmetic.
    /// Returns None on overflow.
    pub fn apply(&self, value: u64) -> Option<u64> {
        let result = (value as u128)
            .checked_mul(self.numerator as u128)?
            .checked_div(self.denominator as u128)?;
        u64::try_from(result).ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacemakerConfig {
    pub base_propose_timeout_ms: u64,
    pub base_prevote_timeout_ms: u64,
    pub base_precommit_timeout_ms: u64,
    pub timeout_multiplier: FixedMultiplier,
    pub max_timeout_rounds: u64,
}

impl Default for PacemakerConfig {
    fn default() -> Self {
        Self {
            base_propose_timeout_ms: 3000,
            base_prevote_timeout_ms: 1000,
            base_precommit_timeout_ms: 1000,
            timeout_multiplier: FixedMultiplier::new(3, 2),
            max_timeout_rounds: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pacemaker {
    config: PacemakerConfig,
}

impl Pacemaker {
    pub fn new(config: PacemakerConfig) -> Self {
        Self { config }
    }

    /// Calculate timeout in milliseconds. Returns None on overflow.
    pub fn timeout_ms(&self, round: u64, step: ConsensusStep) -> Option<u64> {
        let base = match step {
            ConsensusStep::Propose => self.config.base_propose_timeout_ms,
            ConsensusStep::Prevote => self.config.base_prevote_timeout_ms,
            ConsensusStep::Precommit => self.config.base_precommit_timeout_ms,
            ConsensusStep::Commit => return Some(0),
        };

        let capped_round = round.min(self.config.max_timeout_rounds);
        let mut value = base;
        for _ in 0..capped_round {
            value = self.config.timeout_multiplier.apply(value)?;
        }
        Some(value)
    }
}
