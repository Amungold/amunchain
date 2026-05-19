use amun_quorum_certificate::QuorumCertificate;

#[derive(Debug, Clone)]
pub struct ValidatorLock {
    pub locked_round: u64,
    pub locked_value: Option<[u8; 32]>,
    pub locked_qc: Option<QuorumCertificate>,
}

impl ValidatorLock {
    pub fn new() -> Self {
        Self { locked_round: 0, locked_value: None, locked_qc: None }
    }

    pub fn try_lock(&mut self, qc: &QuorumCertificate) -> Result<(), &'static str> {
        if qc.round < self.locked_round {
            return Err("cannot lock at lower round");
        }
        if qc.round == self.locked_round {
            if let Some(locked_val) = self.locked_value {
                if locked_val != qc.block_hash {
                    return Err("conflicting lock at same round");
                }
            }
        }
        self.locked_round = qc.round;
        self.locked_value = Some(qc.block_hash);
        self.locked_qc = Some(qc.clone());
        Ok(())
    }

    pub fn can_prevote_for(&self, round: u64, value: [u8; 32]) -> bool {
        if round < self.locked_round { return false; }
        if round == self.locked_round {
            if let Some(locked_val) = self.locked_value {
                return locked_val == value;
            }
        }
        true
    }

    pub fn can_precommit_for(&self, round: u64, value: [u8; 32]) -> bool {
        if let Some(locked_val) = self.locked_value {
            if round >= self.locked_round && value != locked_val {
                return false;
            }
        }
        true
    }
}

impl Default for ValidatorLock {
    fn default() -> Self { Self::new() }
}
