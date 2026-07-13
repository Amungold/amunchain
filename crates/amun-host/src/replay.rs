pub struct ReplayGuard {
    pub chain_id: u64,
    pub guard_enabled: bool,
}

impl ReplayGuard {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            guard_enabled: true,
        }
    }

    pub fn check_chain_id(&self, chain_id: u64) -> Result<(), &'static str> {
        if !self.guard_enabled {
            return Ok(());
        }
        if chain_id != self.chain_id {
            return Err("chain ID mismatch - cross-chain replay rejected");
        }
        Ok(())
    }
}
