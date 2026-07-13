#[derive(Clone, Debug, Default)]
pub struct EngineMetrics {
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
}

impl EngineMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_qc_formed(&mut self, _h: u64) {
        self.qcs_formed += 1;
    }

    pub fn record_block_finalized(&mut self, _h: u64) {
        self.blocks_finalized += 1;
    }

    pub fn record_vote(&mut self) {
        self.votes_received += 1;
    }

    pub fn summary(&self) -> String {
        format!(
            "qcs:{} final:{} votes:{}",
            self.qcs_formed, self.blocks_finalized, self.votes_received
        )
    }
}
