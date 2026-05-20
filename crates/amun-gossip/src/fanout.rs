use crate::constants::GOSSIP_FANOUT;

pub struct Fanout {
    fanout_size: usize,
}

impl Fanout {
    pub fn new() -> Self {
        Self {
            fanout_size: GOSSIP_FANOUT,
        }
    }
    pub fn select_peers(&self, total_peers: usize, exclude_self: bool) -> usize {
        let available = if exclude_self {
            total_peers.saturating_sub(1)
        } else {
            total_peers
        };
        self.fanout_size.min(available)
    }
    pub fn rounds_required(&self, total_peers: usize) -> u32 {
        if total_peers <= 1 {
            return 0;
        }
        let mut reached: usize = 1;
        let mut rounds: u32 = 0;
        while reached < total_peers {
            reached = reached.saturating_add(reached * self.fanout_size);
            rounds = rounds.saturating_add(1);
            if rounds > 100 {
                break;
            }
        }
        rounds
    }
}

impl Default for Fanout {
    fn default() -> Self {
        Self::new()
    }
}
