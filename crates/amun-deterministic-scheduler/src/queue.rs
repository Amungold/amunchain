use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueEntry {
    pub position: u64,
    pub priority: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DeterministicQueue {
    entries: VecDeque<QueueEntry>,
    max_capacity: usize,
}

impl DeterministicQueue {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::with_capacity(1024),
            max_capacity: 10000,
        }
    }

    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_capacity.min(1024)),
            max_capacity,
        }
    }

    pub fn push(
        &mut self,
        position: u64,
        priority: u8,
        payload: Vec<u8>,
    ) -> Result<(), &'static str> {
        if self.entries.len() >= self.max_capacity {
            return Err("queue full");
        }
        // Binary insertion: find position by (priority ASC, position ASC)
        // Lower priority number = higher scheduling priority (executed first)
        let entry = QueueEntry {
            position,
            priority,
            payload,
        };
        let idx = self
            .entries
            .binary_search_by(|e| {
                // Primary: priority (lower = higher priority)
                // Secondary: position (lower = earlier)
                entry
                    .priority
                    .cmp(&e.priority)
                    .then(entry.position.cmp(&e.position))
            })
            .unwrap_or_else(|i| i);

        self.entries.insert(idx, entry);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<QueueEntry> {
        self.entries.pop_front()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }
}

impl Default for DeterministicQueue {
    fn default() -> Self {
        Self::new()
    }
}
