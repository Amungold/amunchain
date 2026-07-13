pub struct ConsensusClock {
    pub epoch: u64,
    pub slot: u64,
    pub max_drift_ms: u64,
    pub slots_per_epoch: u64,
}

impl Default for ConsensusClock {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsensusClock {
    pub fn new() -> Self {
        Self {
            epoch: 1,
            slot: 0,
            max_drift_ms: 500,
            slots_per_epoch: 100,
        }
    }

    pub fn advance_slot(&mut self) {
        self.slot = self.slot.saturating_add(1);
        if self.slot >= self.slots_per_epoch {
            self.epoch = self.epoch.saturating_add(1);
            self.slot = 0;
        }
    }

    pub fn current_tick(&self) -> u64 {
        self.epoch
            .saturating_mul(self.slots_per_epoch)
            .saturating_add(self.slot)
    }
}
