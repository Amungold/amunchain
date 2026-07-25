// N109.6: Basic proposal validation
use crate::messages::N109BlockProposal;
impl N109BlockProposal {
    /// Validate a block proposal against basic consensus rules.
    ///
    /// `hash_fn` computes the canonical block hash from `block_bytes`.
    /// This strategy injection avoids a circular dependency on `amun-block-builder`
    /// while keeping validation logic in the consensus layer.
    pub fn validate_basic<F>(
        &self,
        current_height: u64,
        tip_parent: &[u8; 32],
        now_secs: u64,
        hash_fn: F,
    ) -> Result<(), String>
    where
        F: FnOnce(&[u8]) -> Result<[u8; 32], String>,
    {
        if self.height != current_height + 1 {
            return Err(format!(
                "HEIGHT: expected {}, got {}",
                current_height + 1,
                self.height
            ));
        }
        if &self.parent_root != tip_parent {
            return Err("PARENT: parent_root != local tip".into());
        }
        if self.timestamp > now_secs + 10 {
            return Err("TIMESTAMP_FUTURE".into());
        }
        if self.timestamp < now_secs.saturating_sub(60) {
            return Err("TIMESTAMP_PAST".into());
        }
        // R2.2: Delegate canonical hash computation to the caller.
        // The caller (e.g. amun-live-cluster) knows the Block type
        // and can call Block::block_hash().
        let computed = hash_fn(&self.block_bytes)?;
        if computed != self.block_hash {
            return Err(format!(
                "HASH_INTEGRITY: stated={} computed={}",
                hex::encode(self.block_hash),
                hex::encode(computed)
            ));
        }
        Ok(())
    }
}
