pub struct LivenessProof;

impl LivenessProof {
    pub fn eventual_progress(
        timeout_count: u32,
        max_timeouts: u32,
        round_advanced: bool,
    ) -> bool {
        if timeout_count >= max_timeouts {
            return false;
        }
        round_advanced
    }

    pub fn quorum_formation_possible(active: usize, threshold: usize) -> bool {
        active >= threshold
    }
}
