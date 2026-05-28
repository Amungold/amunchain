//! ExecutionScheduler — orchestrates constitutional task ordering.
//!
//! The scheduler determines WHICH task executes NEXT.
//! It does NOT determine constitutional validity.
//!
//! INVARIANT: Scheduler decisions affect ordering, not validity.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::execution_task::ExecutionTask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleDecision {
    ExecuteNow,
    Queued { position: u64 },
    Blocked,
    Completed,
}

#[derive(Debug, Clone)]
pub struct ExecutionScheduler {
    tasks: Vec<ExecutionTask>,
    decisions: Vec<ScheduleDecision>,
    /// The context this scheduler operates within.
    /// Reserved for future context-aware scheduling.
    #[allow(dead_code)]
    context_hash: ConstitutionalHash,
}

impl ExecutionScheduler {
    pub fn new(context_hash: ConstitutionalHash) -> Self {
        Self { tasks: Vec::new(), decisions: Vec::new(), context_hash }
    }

    pub fn add_task(&mut self, task: ExecutionTask) {
        self.decisions.push(ScheduleDecision::Queued { position: self.tasks.len() as u64 });
        self.tasks.push(task);
    }

    /// Get the next task ready for execution.
    /// Promotes the first queued task to ExecuteNow, then returns it.
    pub fn next_task(&mut self) -> Option<&ExecutionTask> {
        // First, check if any task is already marked ExecuteNow
        for (i, decision) in self.decisions.iter().enumerate() {
            if matches!(decision, ScheduleDecision::ExecuteNow) {
                self.decisions[i] = ScheduleDecision::Completed;
                return Some(&self.tasks[i]);
            }
        }
        // Promote the first queued task to ExecuteNow and return it
        for (i, decision) in self.decisions.iter().enumerate() {
            if matches!(decision, ScheduleDecision::Queued { .. }) {
                self.decisions[i] = ScheduleDecision::Completed;
                return Some(&self.tasks[i]);
            }
        }
        None
    }

    pub fn block_task(&mut self, task_id: u64) {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.task_id == task_id {
                self.decisions[i] = ScheduleDecision::Blocked;
                return;
            }
        }
    }

    pub fn unblock_task(&mut self, task_id: u64) {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.task_id == task_id {
                self.decisions[i] = ScheduleDecision::Queued { position: self.tasks.len() as u64 };
                return;
            }
        }
    }

    pub fn task_count(&self) -> usize { self.tasks.len() }
    pub fn pending_count(&self) -> usize {
        self.decisions.iter().filter(|d| matches!(d, ScheduleDecision::Queued { .. } | ScheduleDecision::Blocked)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_task::{ExecutionTask, TaskType};

    fn make_task(id: u64, seq: u64) -> ExecutionTask {
        ExecutionTask::new(id, TaskType::StateTransition, [0xAB; 32], [0xBC; 32], [0xCD; 32], seq)
    }

    #[test]
    fn test_scheduler_fifo_order() {
        let mut s = ExecutionScheduler::new([0xAB; 32]);
        s.add_task(make_task(1, 0));
        s.add_task(make_task(2, 1));

        let first = s.next_task().unwrap();
        assert_eq!(first.task_id, 1);
        let second = s.next_task().unwrap();
        assert_eq!(second.task_id, 2);
        assert!(s.next_task().is_none());
    }

    #[test]
    fn test_block_and_unblock() {
        let mut s = ExecutionScheduler::new([0xAB; 32]);
        s.add_task(make_task(1, 0));
        s.block_task(1);
        assert!(s.next_task().is_none());
        s.unblock_task(1);
        let task = s.next_task().unwrap();
        assert_eq!(task.task_id, 1);
    }

    #[test]
    fn test_pending_count() {
        let mut s = ExecutionScheduler::new([0xAB; 32]);
        s.add_task(make_task(1, 0));
        s.add_task(make_task(2, 1));
        assert_eq!(s.pending_count(), 2);
        s.next_task();
        assert_eq!(s.pending_count(), 1);
    }
}
