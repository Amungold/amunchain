use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DeterministicTimerWheel {
    timers: VecDeque<(u64, u64)>,
    current_round: u64,
    next_id: u64,
}

impl DeterministicTimerWheel {
    pub fn new() -> Self {
        Self {
            timers: VecDeque::new(),
            current_round: 0,
            next_id: 0,
        }
    }

    pub fn schedule(&mut self, round: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        // Binary insert: find position maintaining sorted order
        let idx = self
            .timers
            .binary_search_by(|(r, tid)| r.cmp(&round).then(tid.cmp(&id)))
            .unwrap_or_else(|i| i);
        self.timers.insert(idx, (round, id));
        id
    }

    pub fn cancel(&mut self, id: u64) {
        self.timers.retain(|(_, tid)| *tid != id);
    }

    pub fn advance(&mut self) -> Vec<u64> {
        self.current_round += 1;
        let mut fired = Vec::new();

        while let Some(&(round, id)) = self.timers.front() {
            if round <= self.current_round {
                self.timers.pop_front();
                fired.push(id);
            } else {
                break;
            }
        }

        fired
    }

    pub fn current_round(&self) -> u64 {
        self.current_round
    }
    pub fn pending_count(&self) -> usize {
        self.timers.len()
    }
}

impl Default for DeterministicTimerWheel {
    fn default() -> Self {
        Self::new()
    }
}
