use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct ConsensusMetrics {
    pub votes_accepted: u64,
    pub duplicate_votes: u64,
    pub future_height_votes: u64,
    pub votes_rejected: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub rounds_started: u64,
    pub proposals_made: u64,
    pub last_proposed_at: Option<Instant>,
    pub last_qc_at: Option<Instant>,
    pub last_finalized_at: Option<Instant>,
    pub finality_timestamps: Vec<(u64, Instant)>,
    pub current_height: u64,
    pub highest_round_with_votes: u64,
}

impl ConsensusMetrics {
    pub fn new() -> Self { Self::default() }
    pub fn record_vote_accepted(&mut self) { self.votes_accepted += 1; }
    pub fn record_duplicate_vote(&mut self) { self.duplicate_votes += 1; self.votes_rejected += 1; }
    pub fn record_future_height_vote(&mut self) { self.future_height_votes += 1; self.votes_rejected += 1; }
    pub fn record_round_started(&mut self) { self.rounds_started += 1; }
    pub fn record_proposal(&mut self) { self.proposals_made += 1; self.last_proposed_at = Some(Instant::now()); }
    pub fn record_qc_formed(&mut self, _height: u64) {
        self.qcs_formed += 1;
        self.last_qc_at = Some(Instant::now());
    }
    pub fn record_block_finalized(&mut self, height: u64) {
        self.blocks_finalized += 1;
        self.current_height = height;
        let now = Instant::now();
        self.last_finalized_at = Some(now);
        self.finality_timestamps.push((height, now));
    }
    pub fn update_highest_round(&mut self, height: u64) {
        if height > self.highest_round_with_votes {
            self.highest_round_with_votes = height;
        }
    }
    pub fn finality_stats(&self) -> FinalityStats {
        if self.finality_timestamps.len() < 2 {
            return FinalityStats::default();
        }
        let first = self.finality_timestamps[0].1;
        let last = self.finality_timestamps[self.finality_timestamps.len() - 1].1;
        let elapsed = last.duration_since(first).as_secs_f64();
        let count = self.finality_timestamps.len() as f64;
        let avg_interval = if count > 1.0 { elapsed / (count - 1.0) } else { elapsed };
        FinalityStats {
            blocks_finalized: self.blocks_finalized,
            elapsed_secs: elapsed,
            blocks_per_second: if elapsed > 0.0 { count / elapsed } else { 0.0 },
            avg_block_time_ms: avg_interval * 1000.0,
            qcs_formed: self.qcs_formed,
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "H={} | votes: {}/{} (dup:{}, fut:{}) | rounds: {} | proposals: {} | QC: {} | Final: {}",
            self.current_height,
            self.votes_accepted, self.votes_rejected,
            self.duplicate_votes, self.future_height_votes,
            self.rounds_started, self.proposals_made,
            self.qcs_formed, self.blocks_finalized
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct FinalityStats {
    pub blocks_finalized: u64,
    pub elapsed_secs: f64,
    pub blocks_per_second: f64,
    pub avg_block_time_ms: f64,
    pub qcs_formed: u64,
}
