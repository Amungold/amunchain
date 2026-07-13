pub struct RewardDistributor {
    pub block_reward: u64,
}
impl RewardDistributor {
    pub fn new() -> Self {
        Self {
            block_reward: 1_000_000_000,
        }
    }
    pub fn distribute(&self, total: u64, val: u64) -> (u64, u64) {
        if total == 0 || val == 0 {
            (0, 0)
        } else {
            let v = self
                .block_reward
                .checked_mul(val)
                .unwrap_or(0)
                .checked_div(total)
                .unwrap_or(0);
            (v, self.block_reward.saturating_sub(v))
        }
    }
}

impl Default for RewardDistributor {
    fn default() -> Self {
        Self::new()
    }
}
