use crate::constants::*;

pub struct PresaleVesting {
    pub total_allocated: u64,
    pub total_claimed: u64,
    pub vesting_start_block: u64,
    pub vesting_duration_blocks: u64,
}

impl PresaleVesting {
    pub fn new(start_block: u64) -> Self {
        Self {
            total_allocated: NTR_TOTAL_SUPPLY
                .checked_mul(NTR_PRESALE_ALLOCATION_BPS as u64)
                .unwrap_or(0)
                .checked_div(10000)
                .unwrap_or(0),
            total_claimed: 0,
            vesting_start_block: start_block,
            vesting_duration_blocks: NTR_PRESALE_VESTING_BLOCKS,
        }
    }

    pub fn claimable_amount(&self, current_block: u64, allocated: u64) -> u64 {
        if current_block < self.vesting_start_block {
            return 0;
        }
        let elapsed = current_block.checked_sub(self.vesting_start_block).unwrap_or(0);
        if elapsed >= self.vesting_duration_blocks {
            return allocated;
        }
        allocated
            .checked_mul(elapsed)
            .unwrap_or(0)
            .checked_div(self.vesting_duration_blocks)
            .unwrap_or(0)
    }
}
