use crate::message::NetworkMessage;
use std::collections::VecDeque;

/// Deterministic network scheduler.
/// Controls message delivery order with configurable delays.
#[derive(Debug, Clone)]
pub struct NetworkScheduler {
    /// Messages waiting to be delivered: (deliver_at_round, message)
    queue: VecDeque<(u64, NetworkMessage)>,
    current_round: u64,
    seed: u64,
}

impl NetworkScheduler {
    pub fn new(seed: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            current_round: 0,
            seed,
        }
    }

    /// Enqueue a message for delivery at a specific round.
    pub fn send(&mut self, message: NetworkMessage, delay_rounds: u64) {
        let deliver_at = self.current_round + delay_rounds.max(1);
        // Binary insert to maintain ordering
        let idx = self.queue.binary_search_by(|(r, _)| r.cmp(&deliver_at))
            .unwrap_or_else(|i| i);
        self.queue.insert(idx, (deliver_at, message));
    }

    /// Advance one round. Returns all messages due for delivery.
    pub fn tick(&mut self) -> Vec<NetworkMessage> {
        self.current_round += 1;
        let mut delivered = Vec::new();

        while let Some(&(round, _)) = self.queue.front() {
            if round <= self.current_round {
                if let Some((_, msg)) = self.queue.pop_front() {
                    delivered.push(msg);
                }
            } else {
                break;
            }
        }

        delivered
    }

    pub fn round(&self) -> u64 { self.current_round }
    pub fn pending(&self) -> usize { self.queue.len() }

    /// Deterministic pseudo-random delay based on seed and message content.
    pub fn compute_delay(&self, msg_hash: &[u8; 32]) -> u64 {
        let mut state = self.seed;
        for byte in msg_hash.iter() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(*byte as u64);
        }
        1 + (state % 5) // 1-5 round delay
    }
}
