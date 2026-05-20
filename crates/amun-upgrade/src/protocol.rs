#[derive(Debug, Clone)]
pub struct UpgradeProposal {
    pub id: u64,
    pub approved: bool,
    pub activation_epoch: u64,
}

pub struct UpgradeProtocol {
    pub proposals: Vec<UpgradeProposal>,
    pub current_epoch: u64,
}

impl Default for UpgradeProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeProtocol {
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
            current_epoch: 0,
        }
    }

    pub fn propose(&mut self, proposal: UpgradeProposal) -> Result<(), &'static str> {
        if self.proposals.len() >= 16 {
            return Err("upgrade proposals full");
        }
        self.proposals.push(proposal);
        Ok(())
    }

    pub fn check_activation(&mut self) -> Option<u64> {
        for p in &self.proposals {
            if p.approved && self.current_epoch >= p.activation_epoch {
                return Some(p.id);
            }
        }
        None
    }

    pub fn advance_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
    }
}
