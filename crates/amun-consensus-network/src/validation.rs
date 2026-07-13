// N109.6: Basic proposal validation
use crate::messages::N109BlockProposal;

impl N109BlockProposal {
    pub fn validate_basic(
        &self,
        current_height: u64,
        tip_parent: &[u8; 32],
        now_secs: u64,
    ) -> Result<(), String> {
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
        let computed = blake3::hash(&self.block_bytes);
        if computed.as_bytes() != &self.block_hash {
            return Err(format!(
                "HASH_INTEGRITY: stated={} computed={}",
                hex::encode(self.block_hash),
                hex::encode(computed.as_bytes())
            ));
        }
        Ok(())
    }
}
