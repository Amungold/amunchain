#![allow(clippy::explicit_counter_loop)]

use crate::errors::ReplayFailure;
use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct ConstitutionalStep {
    pub sequence: u64,
    pub entry_hash: [u8; 32],
    pub state_hash: ConstitutionalHash,
    pub step_hash: ConstitutionalHash,
}

#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    pub steps: Vec<ConstitutionalStep>,
    pub total_steps: u64,
}

impl Default for ExecutionTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionTrace {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            total_steps: 0,
        }
    }
    pub fn add_step(&mut self, step: ConstitutionalStep) {
        self.steps.push(step);
        self.total_steps += 1;
    }
    pub fn final_state_hash(&self) -> ConstitutionalHash {
        self.steps.last().map(|s| s.state_hash).unwrap_or([0u8; 32])
    }
    pub fn is_fully_constitutional(&self) -> bool {
        self.total_steps > 0
    }
    pub fn is_transformative(&self) -> bool {
        self.total_steps > 1
    }
}

pub struct DeterministicExecutor;

impl DeterministicExecutor {
    pub fn execute_step(
        entry: &TranscriptEntry,
        current_hash: ConstitutionalHash,
        expected_sequence: u64,
    ) -> Result<ConstitutionalStep, ReplayFailure> {
        if entry.sequence != expected_sequence {
            return Err(ReplayFailure::OrderingViolation {
                expected_sequence,
                actual_sequence: entry.sequence,
            });
        }
        let mut hasher = Sha256::new();
        hasher.update(&current_hash);
        hasher.update(&entry.entry_hash);
        hasher.update(&entry.sequence.to_be_bytes());
        hasher.update(&(entry.domain as u8).to_be_bytes());
        let state_hash: [u8; 32] = hasher.finalize().into();
        Ok(ConstitutionalStep {
            sequence: entry.sequence,
            entry_hash: entry.entry_hash,
            state_hash,
            step_hash: [0u8; 32],
        })
    }

    pub fn execute_with_trace(
        entries: &[TranscriptEntry],
        initial_hash: ConstitutionalHash,
        start_sequence: u64,
    ) -> Result<ExecutionTrace, ReplayFailure> {
        let mut trace = ExecutionTrace::new();
        let mut current_hash = initial_hash;
        let mut expected = start_sequence;
        for entry in entries {
            let step = Self::execute_step(entry, current_hash, expected)?;
            current_hash = step.state_hash;
            trace.add_step(step);
            expected += 1;
        }
        Ok(trace)
    }

    pub fn compute_transcript_hash(entries: &[TranscriptEntry]) -> ConstitutionalHash {
        let mut hasher = Sha256::new();
        for entry in entries {
            hasher.update(&entry.entry_hash);
        }
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub trace: ExecutionTrace,
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub steps: Vec<ConstitutionalStep>,
    pub final_state: ConstitutionalHash,
}

impl ExecutionResult {
    pub fn verify_integrity(&self) -> bool {
        !self.steps.is_empty()
    }
}
