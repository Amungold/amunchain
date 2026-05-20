use crate::equivocation::EquivocationProof;

pub struct SlashingEngine {
    pub base_penalty_bps: u16,
    pub penalty_multiplier: u16,
}

impl SlashingEngine {
    pub fn new() -> Self {
        Self {
            base_penalty_bps: 500,
            penalty_multiplier: 2,
        }
    }

    pub fn process(
        &self,
        proof: &EquivocationProof,
        stake: u64,
        offense_count: u32,
        chain_id: u64,
    ) -> Result<u64, &'static str> {
        if !proof.verify(chain_id) {
            return Err("invalid equivocation proof");
        }

        // First offense: base penalty only
        // Subsequent offenses: multiplier applies
        let multiplier = if offense_count <= 1 {
            1
        } else {
            (offense_count as u64)
                .saturating_mul(self.penalty_multiplier as u64)
                .min(10)
        };

        let penalty_bps = (self.base_penalty_bps as u64)
            .saturating_mul(multiplier)
            .min(10000);

        Ok(stake.saturating_mul(penalty_bps) / 10000)
    }
}

impl Default for SlashingEngine {
    fn default() -> Self {
        Self::new()
    }
}
