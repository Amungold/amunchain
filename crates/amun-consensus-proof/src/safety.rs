pub struct SafetyProof;

impl SafetyProof {
    pub fn no_two_conflicting_blocks(
        height_a: u64,
        height_b: u64,
        hash_a: &[u8; 32],
        hash_b: &[u8; 32],
        finalized_a: bool,
        finalized_b: bool,
    ) -> bool {
        if height_a != height_b {
            return true;
        }
        if hash_a == hash_b {
            return true;
        }
        if finalized_a && finalized_b {
            return false;
        }
        true
    }

    pub fn quorum_intersection(
        quorum_a: u64,
        quorum_b: u64,
        total: u64,
        threshold: u64,
    ) -> bool {
        quorum_a >= threshold && quorum_b >= threshold && quorum_a + quorum_b > total
    }
}
