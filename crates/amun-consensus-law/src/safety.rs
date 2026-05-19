pub struct SafetyAxioms;

impl SafetyAxioms {
    pub fn no_conflicting_commits(
        commit_a_height: u64,
        commit_b_height: u64,
        commit_a_hash: [u8; 32],
        commit_b_hash: [u8; 32],
    ) -> bool {
        if commit_a_height != commit_b_height {
            return true;
        }
        commit_a_hash == commit_b_hash
    }

    pub fn quorum_intersection(
        quorum_a: u64,
        quorum_b: u64,
        total_validators: u64,
        threshold: u64,
    ) -> bool {
        quorum_a >= threshold && quorum_b >= threshold && quorum_a + quorum_b > total_validators
    }

    pub fn accountable_safety(
        votes_a: u64,
        votes_b: u64,
        total: u64,
        required: u64,
    ) -> bool {
        votes_a >= required && votes_b >= required && votes_a + votes_b > total + required
    }

    pub fn lock_monotonicity(locked_round_before: u64, locked_round_after: u64) -> bool {
        locked_round_after >= locked_round_before
    }
}
