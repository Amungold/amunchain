#![forbid(unsafe_code)]

use std::collections::VecDeque;
use sha2::{Sha256, Digest};
use crate::event::Event;
use crate::state::ConstitutionalState;
use crate::transition::TransitionEngine;

/// Deterministic event scheduler
pub struct DeterministicScheduler {
    queue: VecDeque<Event>,
    executed_hashes: Vec<[u8; 32]>,
}

impl DeterministicScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            executed_hashes: Vec::new(),
        }
    }
    
    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }
    
    pub fn push_many(&mut self, events: Vec<Event>) {
        for event in events {
            self.queue.push_back(event);
        }
    }
    
    pub fn execute_next(&mut self, state: &mut ConstitutionalState) -> Option<[u8; 32]> {
        let event = self.queue.pop_front()?;
        let pre_hash = state.hash();
        let result = TransitionEngine::apply(state, &event);
        
        if result.success {
            let post_hash = state.hash();
            self.executed_hashes.push(post_hash);
            self._log_execution(&event, pre_hash, post_hash, true);
            Some(post_hash)
        } else {
            self._log_execution(&event, pre_hash, state.hash(), false);
            None
        }
    }
    
    pub fn execute_all(&mut self, state: &mut ConstitutionalState) -> usize {
        let mut count = 0;
        while self.execute_next(state).is_some() {
            count += 1;
        }
        count
    }
    
    pub fn execution_trace_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for hash in &self.executed_hashes {
            hasher.update(hash);
        }
        hasher.finalize().into()
    }
    
    fn _log_execution(&self, _event: &Event, _pre_hash: [u8; 32], _post_hash: [u8; 32], _success: bool) {
        #[cfg(debug_assertions)]
        println!("[EXEC] event processed");
    }
    
    pub fn clear(&mut self) {
        self.queue.clear();
        self.executed_hashes.clear();
    }
    
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
    
    pub fn executed_count(&self) -> usize {
        self.executed_hashes.len()
    }
}

impl Default for DeterministicScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_consensus_math::Fixed;
    use crate::event::EventType;
    
    #[test]
    fn test_scheduler_deterministic() {
        let mut state1 = ConstitutionalState::new();
        state1.add_account(1, Fixed::from_int(100));
        state1.add_account(2, Fixed::from_int(0));
        
        let mut scheduler1 = DeterministicScheduler::new();
        scheduler1.push(Event::new(EventType::Transfer, 1, 2, Fixed::from_int(30).raw(), 0));
        scheduler1.execute_all(&mut state1);
        
        let mut state2 = ConstitutionalState::new();
        state2.add_account(1, Fixed::from_int(100));
        state2.add_account(2, Fixed::from_int(0));
        
        let mut scheduler2 = DeterministicScheduler::new();
        scheduler2.push(Event::new(EventType::Transfer, 1, 2, Fixed::from_int(30).raw(), 0));
        scheduler2.execute_all(&mut state2);
        
        assert_eq!(state1.hash(), state2.hash());
        assert_eq!(scheduler1.execution_trace_hash(), scheduler2.execution_trace_hash());
    }
}
