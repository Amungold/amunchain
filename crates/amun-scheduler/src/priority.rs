#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Consensus = 0,
    Networking = 1,
    Execution = 2,
    Rpc = 3,
}

pub struct ConstitutionalScheduler {
    pub max_consensus_per_tick: u32,
    pub max_rpc_per_tick: u32,
    pub consensus_count: u32,
    pub rpc_count: u32,
}

impl ConstitutionalScheduler {
    pub fn new() -> Self {
        Self {
            max_consensus_per_tick: 10,
            max_rpc_per_tick: 100,
            consensus_count: 0,
            rpc_count: 0,
        }
    }

    pub fn allow_consensus(&mut self) -> bool {
        if self.consensus_count >= self.max_consensus_per_tick {
            return false;
        }
        self.consensus_count += 1;
        true
    }

    pub fn reset_tick(&mut self) {
        self.consensus_count = 0;
        self.rpc_count = 0;
    }
}
