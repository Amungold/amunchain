// N109.7B: Consensus metrics — tracks proposal rejections for monitoring.
// state_root_mismatches increments when a validator's re-execution
// produces a different state root than the proposer claimed.
#[derive(Debug, Default, Clone)]
pub struct ConsensusMetrics {
    /// N109.7B: Count of proposals rejected due to state root mismatch.
    /// A non-zero value indicates either:
    ///   - The proposer executed incorrectly (bug or byzantine)
    ///   - The local validator executed incorrectly (bug or corrupt state)
    ///   - Non-determinism in the execution engine (critical bug)
    pub state_root_mismatches: u64,

    /// Proposals rejected due to failing basic validation (N109.6)
    pub basic_validation_failures: u64,

    /// Total proposals received
    pub proposals_received: u64,

    /// Proposals that passed both validation and re-execution
    pub proposals_accepted: u64,
}

impl ConsensusMetrics {
    pub fn summary(&self) -> String {
        format!(
            "proposals: received={} accepted={} rejected_basic={} rejected_root={}",
            self.proposals_received,
            self.proposals_accepted,
            self.basic_validation_failures,
            self.state_root_mismatches,
        )
    }
}
