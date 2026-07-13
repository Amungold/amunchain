//! ExecutionTask — a schedulable unit of constitutional execution.
//!
//! A task describes WHAT to execute, not HOW to schedule it.
//! The task carries the minimum information needed for the runtime
//! to produce constitutional artifacts that the kernel can judge.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;

/// The type of execution a task represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    /// Execute a state transition within a boundary.
    StateTransition = 0x01,
    /// Verify an existing artifact against the constitutional kernel.
    VerifyArtifact = 0x02,
    /// Produce a witness for a target artifact.
    ProduceWitness = 0x03,
    /// Restore state from a snapshot.
    RestoreFromSnapshot = 0x04,
}

/// A schedulable unit of constitutional execution.
///
/// The task is RUNTIME-OWNED: it describes execution work.
/// Constitutional judgment happens AFTER the task produces artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTask {
    /// Unique task identifier.
    pub task_id: u64,

    /// What kind of execution this task performs.
    pub task_type: TaskType,

    /// The constitutional context this task executes within.
    pub context_hash: ConstitutionalHash,

    /// The boundary active during this task's execution.
    pub boundary_hash: ConstitutionalHash,

    /// Hash of the artifact this task operates on (varies by task_type).
    /// For StateTransition: the preceding journal entry.
    /// For VerifyArtifact: the artifact to verify.
    /// For ProduceWitness: the target artifact.
    /// For RestoreFromSnapshot: the snapshot hash.
    pub target_artifact_hash: ConstitutionalHash,

    /// Informational payload for the executor (CIR-001).
    pub execution_payload: Option<Vec<u8>>,

    /// Sequence number within the context for ordering.
    pub task_sequence: u64,
}

impl ExecutionTask {
    pub fn new(
        task_id: u64,
        task_type: TaskType,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        target_artifact_hash: ConstitutionalHash,
        task_sequence: u64,
    ) -> Self {
        Self {
            task_id, task_type, context_hash, boundary_hash,
            target_artifact_hash, execution_payload: None, task_sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = ExecutionTask::new(
            1,
            TaskType::StateTransition,
            [0xAB; 32],
            [0xBC; 32],
            [0xCD; 32],
            0,
        );
        assert_eq!(task.task_id, 1);
        assert_eq!(task.task_type, TaskType::StateTransition);
        assert_eq!(task.task_sequence, 0);
    }
}
