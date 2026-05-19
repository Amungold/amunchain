pub struct FormalInvariants;

impl FormalInvariants {
    pub fn supply_conservation(minted: u64, burned: u64, initial: u64, current: u64) -> bool {
        initial.checked_add(minted).unwrap_or(0) == current.checked_add(burned).unwrap_or(0)
    }

    pub fn nonce_monotonicity(prev_nonce: u64, new_nonce: u64) -> bool {
        new_nonce > prev_nonce
    }

    pub fn stake_consistency(total_staked: u64, validator_stakes: &[u64]) -> bool {
        total_staked == validator_stakes.iter().sum()
    }

    pub fn quorum_safety(votes: u64, total: u64, threshold_bps: u16) -> bool {
        if total == 0 {
            return false;
        }
        votes.checked_mul(10000).unwrap_or(0)
            >= total.checked_mul(threshold_bps as u64).unwrap_or(0)
    }

    pub fn no_overflow_in_supply(a: u64, b: u64) -> bool {
        a.checked_add(b).is_some()
    }
}
