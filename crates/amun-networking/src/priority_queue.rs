use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    BestEffort = 4,
}

pub struct PriorityWrite {
    pub data: Arc<[u8]>,
    pub offset: usize,
    pub priority: MessagePriority,
}

pub struct PriorityQueue {
    queues: Vec<VecDeque<PriorityWrite>>,
    total_bytes: usize,
    total_count: usize,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self {
            queues: vec![
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            total_bytes: 0,
            total_count: 0,
        }
    }

    pub fn push(&mut self, data: Arc<[u8]>, priority: MessagePriority) {
        let idx = priority as usize;
        self.total_bytes += data.len();
        self.total_count += 1;
        self.queues[idx].push_back(PriorityWrite {
            data,
            offset: 0,
            priority,
        });
    }

    pub fn pop_front(&mut self) -> Option<PriorityWrite> {
        for queue in self.queues.iter_mut() {
            if let Some(write) = queue.pop_front() {
                self.total_bytes = self.total_bytes.saturating_sub(write.data.len());
                self.total_count = self.total_count.saturating_sub(1);
                return Some(write);
            }
        }
        None
    }

    pub fn front_mut(&mut self) -> Option<&mut PriorityWrite> {
        for queue in self.queues.iter_mut() {
            if let Some(write) = queue.front_mut() {
                return Some(write);
            }
        }
        None
    }

    pub fn drop_lowest_priority(&mut self) -> bool {
        for i in (0..self.queues.len()).rev() {
            if let Some(write) = self.queues[i].pop_back() {
                self.total_bytes = self.total_bytes.saturating_sub(write.data.len());
                self.total_count = self.total_count.saturating_sub(1);
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.total_count
    }

    pub fn bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn is_empty(&self) -> bool {
        self.total_count == 0
    }
}
