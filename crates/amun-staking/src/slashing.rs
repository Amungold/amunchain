use amun_economics::constants::SLASHING_PENALTY_BASIS_POINTS;
pub struct SlashingConditions {
    pub base_penalty_bps: u16,
    pub max_slash_count: u32,
}
impl SlashingConditions {
    pub fn new() -> Self {
        Self {
            base_penalty_bps: SLASHING_PENALTY_BASIS_POINTS,
            max_slash_count: 5,
        }
    }
    pub fn calculate_slash(&self, stake: u64, count: u32) -> u64 {
        let m = (count as u64).min(10);
        let bps = (self.base_penalty_bps as u64 * m).min(10000);
        stake
            .checked_mul(bps)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0)
    }
}

impl Default for SlashingConditions {
    fn default() -> Self {
        Self::new()
    }
}
