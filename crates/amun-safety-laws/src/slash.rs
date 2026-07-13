use crate::equivocation::EquivocationProof;

/// Slashing condition: what penalty applies for equivocation.
#[derive(Debug, Clone)]
pub struct SlashingCondition {
    pub base_penalty_bps: u16,
    pub repeat_multiplier: u16,
}

impl SlashingCondition {
    pub fn new() -> Self {
        Self { base_penalty_bps: 500, repeat_multiplier: 2 }
    }

    pub fn calculate_penalty(
        &self,
        proof: &EquivocationProof,
        stake: u64,
        prior_offenses: u32,
    ) -> Result<u64, &'static str> {
        if !proof.verify() {
            return Err("invalid equivocation proof");
        }

        let multiplier = if prior_offenses == 0 {
            1u64
        } else {
            (prior_offenses as u64)
                .saturating_mul(self.repeat_multiplier as u64)
                .min(10)
        };

        let penalty_bps = (self.base_penalty_bps as u64)
            .saturating_mul(multiplier)
            .min(10000);

        Ok(stake.saturating_mul(penalty_bps) / 10000)
    }
}
