use crate::deterministic::{DeterministicExecutor, ExecutionResult, ExecutionTrace};
use crate::errors::ReplayFailure;
use crate::state::ReplayState;
use amun_constitutional::{ConstitutionalHash, TranscriptEntry};

#[derive(Debug, Clone)]
pub struct EquivalenceProof {
    pub trace: ExecutionTrace,
    pub is_equivalent: bool,
    pub expected_result_hash: ConstitutionalHash,
    pub computed_result_hash: ConstitutionalHash,
}

#[derive(Debug, Clone)]
pub struct ConstitutionalReplayResult {
    pub replay_root: ConstitutionalHash,
    pub continuity_proof: ContinuityProof,
    pub checkpoint_result: CheckpointResult,
    pub equivalence_proof: EquivalenceProof,
    pub authority_proof: AuthorityProof,
    pub deterministic_transcript_hash: ConstitutionalHash,
}

#[derive(Debug, Clone)]
pub struct ContinuityProof {
    pub is_continuous: bool,
    pub chain_hash: ConstitutionalHash,
}

#[derive(Debug, Clone)]
pub struct CheckpointResult {
    pub checkpoint_hash: ConstitutionalHash,
    pub is_valid: bool,
}

#[derive(Debug, Clone)]
pub struct AuthorityProof {
    pub authority_root: ConstitutionalHash,
    pub is_authoritative: bool,
}

impl ConstitutionalReplayResult {
    pub fn verify(&self) -> bool {
        self.equivalence_proof.is_equivalent
            && self.continuity_proof.is_continuous
            && self.checkpoint_result.is_valid
            && self.authority_proof.is_authoritative
    }
}

pub struct EquivalenceProver;

impl EquivalenceProver {
    pub fn prove_over_result(
        result: &ExecutionResult,
        expected_state: &ReplayState,
    ) -> Result<EquivalenceProof, ReplayFailure> {
        let is_equivalent = result.final_state == expected_state.state_root;
        Ok(EquivalenceProof {
            trace: result.trace.clone(),
            is_equivalent,
            expected_result_hash: expected_state.state_root,
            computed_result_hash: result.final_state,
        })
    }

    pub fn execute_and_self_verify(
        initial_state: &ReplayState,
        entries: &[TranscriptEntry],
        start_sequence: u64,
    ) -> Result<EquivalenceProof, ReplayFailure> {
        // Execute trace from the initial state root
        let trace = DeterministicExecutor::execute_with_trace(
            entries,
            initial_state.state_root,
            start_sequence,
        )?;

        // The final hash from the trace SHOULD match the state root
        // if we had applied the same entries to the ReplayState
        let final_state = trace.final_state_hash();

        let result = ExecutionResult {
            trace: trace.clone(),
            start_sequence,
            end_sequence: start_sequence + entries.len() as u64 - 1,
            steps: trace.steps.clone(),
            final_state,
        };

        // For self-verification: replay the entries through ReplayState
        // to get the expected root, then compare
        let mut state = initial_state.clone();
        for entry in entries {
            state = state.apply_entry(entry)?;
        }

        Self::prove_over_result(&result, &state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_constitutional::ReplayDomain;

    fn mk_entry(seq: u64, hash: [u8; 32]) -> TranscriptEntry {
        TranscriptEntry {
            entry_hash: hash,
            sequence: seq,
            domain: ReplayDomain::Canonical,
        }
    }

    #[test]
    fn execute_and_self_verify_produces_valid_proof() {
        let state = ReplayState::new([0; 32]);
        let entries = vec![mk_entry(1, [0x01; 32])];
        let proof = EquivalenceProver::execute_and_self_verify(&state, &entries, 1).unwrap();
        // The proof should be valid - it compares trace output vs state application
        assert!(proof.is_equivalent);
    }

    #[test]
    fn prove_over_result_accepts_valid_execution() {
        let state = ReplayState::new([0; 32]);
        let entries = vec![mk_entry(1, [0x01; 32])];

        // Apply entries to state to get expected root
        let mut expected_state = state.clone();
        for entry in &entries {
            expected_state = expected_state.apply_entry(entry).unwrap();
        }

        // Execute trace from initial root
        let trace =
            DeterministicExecutor::execute_with_trace(&entries, state.state_root, 1).unwrap();
        let final_state = trace.final_state_hash();

        let result = ExecutionResult {
            trace,
            start_sequence: 1,
            end_sequence: 1,
            steps: vec![],
            final_state,
        };

        let proof = EquivalenceProver::prove_over_result(&result, &expected_state).unwrap();
        assert!(proof.is_equivalent);
    }
}
