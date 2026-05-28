// ============================================================================
// REPLAY STATE — IMMUTABLE-STYLE CONSTITUTIONAL STATE EVOLUTION
// ============================================================================
//
// ALIGNED with actual amun-constitutional ontology.
// TranscriptEntry is { entry_hash, sequence, domain }.
// We do NOT dispatch on semantic event types — that belongs to higher layers.
//
// evolve_state_root() provides CRYPTOGRAPHIC CONTINUITY.
// It does NOT interpret what the event MEANS.

extern crate alloc;
use alloc::vec::Vec;

use amun_constitutional::{ConstitutionalHash, TranscriptEntry};

use crate::canonical::CanonicalHasher;
use crate::errors::ReplayFailure;

#[derive(Debug, Clone)]
pub struct ReplayState {
    pub state_root: ConstitutionalHash,
    pub events_processed: u64,
    pub divergences: Vec<ReplayFailure>,
}

impl ReplayState {
    pub fn new(initial_root: ConstitutionalHash) -> Self {
        Self {
            state_root: initial_root,
            events_processed: 0,
            divergences: Vec::new(),
        }
    }

    pub fn is_divergent(&self) -> bool {
        !self.divergences.is_empty()
    }

    /// Apply a single transcript entry to the state.
    /// This is the ONLY mutation method — we do not dispatch on event type.
    /// The entry's hash is combined with the current state root to produce
    /// a new state root. This guarantees cryptographic continuity.
    pub fn apply_entry(&self, entry: &TranscriptEntry) -> Result<Self, ReplayFailure> {
        let mut hasher = CanonicalHasher::new();
        hasher.update(&self.state_root);
        hasher.update(&entry.entry_hash);
        hasher.update(&entry.sequence);
        hasher.update(&(entry.domain as u8));
        let new_root = hasher.finalize();

        Ok(Self {
            state_root: new_root,
            events_processed: self.events_processed + 1,
            divergences: self.divergences.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_constitutional::ReplayDomain;

    #[test]
    fn state_root_evolves_after_entry() {
        let state = ReplayState::new([0xAA; 32]);
        let root_before = state.state_root;

        let entry = TranscriptEntry {
            entry_hash: [0xBB; 32],
            sequence: 1,
            domain: ReplayDomain::Canonical,
        };

        let new_state = state.apply_entry(&entry).unwrap();
        assert_ne!(root_before, new_state.state_root);
        assert_eq!(new_state.events_processed, 1);
    }

    #[test]
    fn same_entry_same_result() {
        let state = ReplayState::new([0xAA; 32]);
        let entry = TranscriptEntry {
            entry_hash: [0xBB; 32],
            sequence: 1,
            domain: ReplayDomain::Canonical,
        };

        let r1 = state.apply_entry(&entry).unwrap();
        let r2 = state.apply_entry(&entry).unwrap();

        // Same input → same output (deterministic)
        assert_eq!(r1.state_root, r2.state_root);
    }

    #[test]
    fn different_entries_different_results() {
        let state = ReplayState::new([0xAA; 32]);
        let e1 = TranscriptEntry {
            entry_hash: [0x01; 32],
            sequence: 1,
            domain: ReplayDomain::Canonical,
        };
        let e2 = TranscriptEntry {
            entry_hash: [0x02; 32],
            sequence: 1,
            domain: ReplayDomain::Canonical,
        };

        let r1 = state.apply_entry(&e1).unwrap();
        let r2 = state.apply_entry(&e2).unwrap();

        assert_ne!(r1.state_root, r2.state_root);
    }
}
