/// Quorum threshold: 2/3 + 1 of total stake.
pub fn quorum_threshold(total_stake: u64) -> u64 {
    (total_stake * 2 / 3) + 1
}

/// Check if a vote weight meets quorum.
pub fn has_quorum(vote_weight: u64, total_stake: u64) -> bool {
    vote_weight >= quorum_threshold(total_stake)
}
