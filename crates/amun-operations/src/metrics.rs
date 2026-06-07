use serde::{Deserialize, Serialize};

/// Operational metrics for an AmunChain node.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub blocks_produced: u64,
    pub blocks_imported: u64,
    pub transactions_processed: u64,
    pub proofs_generated: u64,
    pub proofs_verified: u64,
    pub replays_performed: u64,
    pub state_syncs_completed: u64,
    pub peers_connected: u64,
    pub peers_disconnected: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
}

impl NodeMetrics {
    pub fn new() -> Self { Self::default() }

    pub fn record_block_produced(&mut self) { self.blocks_produced += 1; }
    pub fn record_block_imported(&mut self) { self.blocks_imported += 1; }
    pub fn record_transaction(&mut self) { self.transactions_processed += 1; }
    pub fn record_proof_generated(&mut self) { self.proofs_generated += 1; }
    pub fn record_proof_verified(&mut self) { self.proofs_verified += 1; }
    pub fn record_replay(&mut self) { self.replays_performed += 1; }
    pub fn record_state_sync(&mut self) { self.state_syncs_completed += 1; }

    /// Return a summary of all metrics as key-value pairs.
    pub fn summary(&self) -> Vec<(&str, u64)> {
        vec![
            ("blocks_produced", self.blocks_produced),
            ("blocks_imported", self.blocks_imported),
            ("transactions_processed", self.transactions_processed),
            ("proofs_generated", self.proofs_generated),
            ("proofs_verified", self.proofs_verified),
            ("replays_performed", self.replays_performed),
            ("state_syncs_completed", self.state_syncs_completed),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n62_metrics_record_and_summarize() {
        let mut metrics = NodeMetrics::new();
        metrics.record_block_produced();
        metrics.record_transaction();
        metrics.record_proof_generated();
        metrics.record_replay();
        assert_eq!(metrics.blocks_produced, 1);
        assert_eq!(metrics.transactions_processed, 1);
        assert_eq!(metrics.summary().len(), 7);
    }
}
