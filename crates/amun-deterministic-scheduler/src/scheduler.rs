use crate::budget::ResourceBudget;
use crate::queue::DeterministicQueue;

/// Deterministic scheduler with weighted fairness.
/// I/O gets a configurable share, regular tasks get the remainder.
#[derive(Debug, Clone)]
pub struct DeterministicScheduler {
    pending: DeterministicQueue,
    io_pending: DeterministicQueue,
    budget: ResourceBudget,
    round: u64,
    io_quota: u64,
    io_consumed_this_round: u64,
}

impl DeterministicScheduler {
    pub fn new(budget: ResourceBudget) -> Self {
        Self {
            pending: DeterministicQueue::new(),
            io_pending: DeterministicQueue::new(),
            budget,
            round: 0,
            io_quota: 3,
            io_consumed_this_round: 0,
        }
    }

    pub fn with_io_quota(budget: ResourceBudget, io_quota: u64) -> Self {
        Self {
            pending: DeterministicQueue::new(),
            io_pending: DeterministicQueue::new(),
            budget,
            round: 0,
            io_quota,
            io_consumed_this_round: 0,
        }
    }

    pub fn enqueue(
        &mut self,
        position: u64,
        priority: u8,
        payload: Vec<u8>,
    ) -> Result<(), &'static str> {
        self.pending.push(position, priority, payload)
    }

    pub fn enqueue_io(
        &mut self,
        position: u64,
        priority: u8,
        payload: Vec<u8>,
    ) -> Result<(), &'static str> {
        self.io_pending.push(position, priority, payload)
    }

    /// Execute batch with weighted fairness.
    /// I/O gets up to io_quota tasks per round, then regular tasks execute.
    pub fn execute_batch(&mut self, max_tasks: usize) -> Vec<ExecutedTask> {
        let mut executed = Vec::new();
        let mut remaining_budget = self.budget.clone();
        self.io_consumed_this_round = 0;

        // Phase 1: I/O up to quota
        while executed.len() < max_tasks && self.io_consumed_this_round < self.io_quota {
            if let Some(task) = self.io_pending.pop() {
                let cost = (task.payload.len() as u64).min(1000) + (task.priority as u64 * 50);
                if remaining_budget.consume(cost).is_ok() {
                    self.io_consumed_this_round += 1;
                    executed.push(ExecutedTask {
                        position: task.position,
                        priority: task.priority,
                        payload_len: task.payload.len(),
                        round: self.round,
                        is_io: true,
                    });
                } else {
                    self.io_pending
                        .push(task.position, task.priority, task.payload)
                        .ok();
                    break;
                }
            } else {
                break;
            }
        }

        // Phase 2: Regular tasks with remaining budget
        while executed.len() < max_tasks {
            if let Some(task) = self.pending.pop() {
                let cost = (task.payload.len() as u64) + (task.priority as u64 * 100);
                if remaining_budget.consume(cost).is_ok() {
                    executed.push(ExecutedTask {
                        position: task.position,
                        priority: task.priority,
                        payload_len: task.payload.len(),
                        round: self.round,
                        is_io: false,
                    });
                } else {
                    self.pending
                        .push(task.position, task.priority, task.payload)
                        .ok();
                    break;
                }
            } else {
                break;
            }
        }

        // Phase 3: If I/O quota not exhausted and regular queue empty, drain remaining I/O
        while executed.len() < max_tasks && self.io_consumed_this_round < self.io_quota {
            if let Some(task) = self.io_pending.pop() {
                let cost = (task.payload.len() as u64).min(1000) + (task.priority as u64 * 50);
                if remaining_budget.consume(cost).is_ok() {
                    self.io_consumed_this_round += 1;
                    executed.push(ExecutedTask {
                        position: task.position,
                        priority: task.priority,
                        payload_len: task.payload.len(),
                        round: self.round,
                        is_io: true,
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.round += 1;
        self.budget.reset();
        executed
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn io_pending_count(&self) -> usize {
        self.io_pending.len()
    }
    pub fn round(&self) -> u64 {
        self.round
    }
    pub fn io_quota_remaining(&self) -> u64 {
        self.io_quota.saturating_sub(self.io_consumed_this_round)
    }
}

#[derive(Debug, Clone)]
pub struct ExecutedTask {
    pub position: u64,
    pub priority: u8,
    pub payload_len: usize,
    pub round: u64,
    pub is_io: bool,
}
