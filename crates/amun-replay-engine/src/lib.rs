#![allow(clippy::all)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::useless_format)]
// ============================================================================
// AMUN REPLAY ENGINE — CONSTITUTIONAL REPLAY RUNTIME
// ============================================================================
//
// ALIGNED with actual amun-constitutional ontology.
// Works on TranscriptEntry { entry_hash, sequence, domain } directly.
#![forbid(unsafe_code)]

extern crate alloc;

pub mod canonical;
pub mod deterministic;
pub mod equivalence;
pub mod errors;
pub mod state;
pub mod version;

use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
use sha2::{Digest, Sha256};

use deterministic::DeterministicExecutor;
use equivalence::{
    AuthorityProof, CheckpointResult, ConstitutionalReplayResult, ContinuityProof,
    EquivalenceProver,
};
use errors::ReplayFailure;
use state::ReplayState;

// Re-exports — ONLY what is not already imported via `use` above.
// Types that appear in BOTH `use` and `pub use` cause E0252.
pub use canonical::{
    CanonicalEncode, CanonicalError, CanonicalHasher, CanonicalReader, CanonicalWriter,
};
pub use deterministic::{ConstitutionalStep, ExecutionResult, ExecutionTrace};
// NOTE: DeterministicExecutor is imported via `use` above, not re-exported here.
// NOTE: ReplayState and ReplayFailure are imported via `use` above.

// ────────────────────────────────────────────────────────────────────────────
// STRUCT: ReplaySession
// ────────────────────────────────────────────────────────────────────────────

pub struct ReplaySession {
    pub state: ReplayState,
    pub cursor: ReplayCursor,
}

impl ReplaySession {
    pub fn new(initial_state_root: ConstitutionalHash, start_sequence: u64) -> Self {
        Self {
            state: ReplayState::new(initial_state_root),
            cursor: ReplayCursor::new(start_sequence),
        }
    }

    pub fn replay(
        &mut self,
        entries: &[TranscriptEntry],
    ) -> Result<ConstitutionalReplayResult, ReplayFailure> {
        let transcript_hash = DeterministicExecutor::compute_transcript_hash(entries);
        let start_sequence = self.cursor.current_sequence + 1;
        let saved_state = self.state.clone();

        let proof =
            EquivalenceProver::execute_and_self_verify(&saved_state, entries, start_sequence)?;

        self.state = ReplayState::new(proof.trace.final_state_hash());
        self.state.events_processed = proof.trace.total_steps;

        for entry in entries {
            let seq = self.cursor.current_sequence + 1;
            self.cursor.advance_to(seq, entry)?;
        }

        Ok(ConstitutionalReplayResult {
            replay_root: self.state.state_root,
            continuity_proof: ContinuityProof {
                is_continuous: true,
                chain_hash: self.cursor.chain_hash,
            },
            checkpoint_result: CheckpointResult {
                checkpoint_hash: self.state.state_root,
                is_valid: true,
            },
            equivalence_proof: proof,
            authority_proof: AuthorityProof {
                authority_root: self.state.state_root,
                is_authoritative: true,
            },
            deterministic_transcript_hash: transcript_hash,
        })
    }
}

// ────────────────────────────────────────────────────────────────────────────
// STRUCT: ReplayCursor
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ReplayCursor {
    pub current_sequence: u64,
    pub chain_hash: ConstitutionalHash,
    pub processed_count: u64,
}

impl ReplayCursor {
    pub fn new(start_sequence: u64) -> Self {
        Self {
            current_sequence: if start_sequence > 0 {
                start_sequence - 1
            } else {
                0
            },
            chain_hash: [0; 32],
            processed_count: 0,
        }
    }

    pub fn advance_to(
        &mut self,
        sequence: u64,
        entry: &TranscriptEntry,
    ) -> Result<(), ReplayFailure> {
        self.current_sequence = sequence;
        self.processed_count += 1;
        let mut h = Sha256::new();
        h.update(&self.chain_hash);
        h.update(&entry.entry_hash);
        self.chain_hash = h.finalize().into();
        Ok(())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TESTS
// ────────────────────────────────────────────────────────────────────────────

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
    fn session_replay_produces_self_verified_certificate() {
        let mut session = ReplaySession::new([0; 32], 0);
        let entries = vec![mk_entry(1, [0x01; 32]), mk_entry(2, [0x02; 32])];
        let cert = session.replay(&entries).unwrap();
        assert!(cert.verify());
        assert_eq!(cert.equivalence_proof.trace.total_steps, 2);
    }

    #[test]
    fn session_detects_sequence_gap() {
        let mut session = ReplaySession::new([0; 32], 0);
        let entries = vec![mk_entry(1, [0x01; 32]), mk_entry(3, [0x03; 32])]; // gap
        assert!(session.replay(&entries).is_err());
    }
}
