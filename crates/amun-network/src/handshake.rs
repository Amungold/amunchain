use crate::constants::HANDSHAKE_TIMEOUT_MS;
use amun_kernel_types::PublicKey;

pub struct Handshake {
    pub peer_id: PublicKey,
    pub protocol_version: u32,
    pub capabilities: u64,
    pub initiated_ms: u64,
    pub completed: bool,
}

impl Handshake {
    pub fn new(peer_id: PublicKey, protocol_version: u32, capabilities: u64, now_ms: u64) -> Self {
        Self {
            peer_id,
            protocol_version,
            capabilities,
            initiated_ms: now_ms,
            completed: false,
        }
    }
    pub fn is_timed_out(&self, now_ms: u64) -> bool {
        now_ms.saturating_sub(self.initiated_ms) > HANDSHAKE_TIMEOUT_MS
    }
    pub fn complete(&mut self) {
        self.completed = true;
    }
}
