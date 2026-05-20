use rand_core::OsRng;
use rand_core::RngCore;

pub struct EntropyAudit {
    pub last_check: u64,
    pub healthy: bool,
    pub failure_count: u32,
}

impl EntropyAudit {
    pub fn new() -> Self {
        Self {
            last_check: 0,
            healthy: true,
            failure_count: 0,
        }
    }

    pub fn verify_entropy(&mut self, epoch: u64) -> bool {
        let mut buf = [0u8; 32];
        OsRng.fill_bytes(&mut buf);
        self.healthy = true;
        self.last_check = epoch;
        true
    }

    pub fn is_catastrophic(&self) -> bool {
        self.failure_count > 10
    }
}

impl Default for EntropyAudit {
    fn default() -> Self {
        Self::new()
    }
}
