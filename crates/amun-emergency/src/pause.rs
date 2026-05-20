pub struct EmergencyPause {
    pub paused: bool,
    pub pause_epoch: u64,
    pub required_validators: u8,
    pub approvals: heapless::Vec<[u8; 32], 64>,
    pub max_pause_duration_epochs: u64,
}

impl EmergencyPause {
    pub fn new(required_validators: u8) -> Self {
        Self {
            paused: false,
            pause_epoch: 0,
            required_validators,
            approvals: heapless::Vec::new(),
            max_pause_duration_epochs: 100,
        }
    }

    pub fn approve(
        &mut self,
        validator_pk: &[u8; 32],
        current_epoch: u64,
    ) -> Result<(), &'static str> {
        if self.paused {
            return Err("already paused");
        }
        if self.approvals.iter().any(|pk| pk == validator_pk) {
            return Err("duplicate approval");
        }
        self.approvals
            .push(*validator_pk)
            .map_err(|_| "approvals full")?;
        if self.approvals.len() >= self.required_validators as usize {
            self.paused = true;
            self.pause_epoch = current_epoch;
        }
        Ok(())
    }

    pub fn check_expiry(&mut self, current_epoch: u64) {
        if self.paused
            && current_epoch
                > self
                    .pause_epoch
                    .saturating_add(self.max_pause_duration_epochs)
        {
            self.reset();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn reset(&mut self) {
        self.paused = false;
        self.approvals.clear();
    }
}
