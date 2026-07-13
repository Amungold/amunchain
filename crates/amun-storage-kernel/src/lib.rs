#![allow(
    clippy::too_many_arguments,
    clippy::ptr_arg,
    clippy::suspicious_open_options
)]
pub mod canonical;
pub mod crash;
pub mod lru;
pub mod persistence;
pub mod smt;

pub use persistence::{NodeStore, ValueKey, ValueStore};
pub use smt::{MerkleProof, NodeHash, ProofStep, ProofVerifier, SparseMerkleTree};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key256(pub [u8; 32]);
impl Key256 {
    pub fn bit(&self, pos: usize) -> u8 {
        (self.0[pos / 8] >> (7 - (pos % 8))) & 1
    }
}

#[derive(Debug, Clone)]
pub struct ValueBlob {
    pub data: Vec<u8>,
}
impl ValueBlob {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    pub fn hash(&self) -> [u8; 32] {
        blake3::hash(&self.data).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateRoot(pub [u8; 32]);
impl StateRoot {
    pub const EMPTY: Self = Self([0u8; 32]);
}

// ============================================================
// CONSTITUTIONAL TIME & LINEAGE
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GenerationId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EpochNumber(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateLineage {
    pub epoch: EpochNumber,
    pub generation: GenerationId,
    pub state_root: StateRoot,
    pub prev_state_root: StateRoot,
    pub wal_sequence: u64,
    pub lineage_hash: [u8; 32],
}

impl StateLineage {
    pub fn new(
        epoch: EpochNumber,
        generation: GenerationId,
        state_root: StateRoot,
        prev_state_root: StateRoot,
        wal_sequence: u64,
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_LINEAGE_V1");
        hasher.update(&epoch.0.to_le_bytes());
        hasher.update(&generation.0.to_le_bytes());
        hasher.update(&state_root.0);
        hasher.update(&prev_state_root.0);
        hasher.update(&wal_sequence.to_le_bytes());
        let lineage_hash = hasher.finalize().into();
        Self {
            epoch,
            generation,
            state_root,
            prev_state_root,
            wal_sequence,
            lineage_hash,
        }
    }
}

// ============================================================
// CONSTITUTIONAL CONTINUITY VERIFIER
// ============================================================
pub struct ConstitutionalContinuity;

impl ConstitutionalContinuity {
    pub fn verify_lineage_continuity(
        prev: &StateLineage,
        next: &StateLineage,
    ) -> Result<bool, String> {
        // Epoch must be monotonic
        if next.epoch < prev.epoch {
            return Err(format!(
                "epoch regression: {} -> {}",
                prev.epoch.0, next.epoch.0
            ));
        }
        // Within same epoch, generation must strictly increase
        if next.epoch == prev.epoch && next.generation <= prev.generation {
            return Err(format!(
                "generation not monotonic: {} -> {} (epoch {})",
                prev.generation.0, next.generation.0, prev.epoch.0
            ));
        }
        // State root chain must be continuous
        if next.prev_state_root != prev.state_root {
            return Err(format!(
                "state root continuity broken: expected prev {:?}, got {:?}",
                &prev.state_root.0[..8],
                &next.prev_state_root.0[..8]
            ));
        }
        Ok(true)
    }

    pub fn verify_epoch_transition(
        prev: &StateLineage,
        next: &StateLineage,
    ) -> Result<bool, String> {
        if next.epoch > prev.epoch {
            // Epoch transition: generation resets to 0
            if next.generation.0 != 0 {
                return Err(format!(
                    "epoch transition must reset generation to 0, got {}",
                    next.generation.0
                ));
            }
        }
        Self::verify_lineage_continuity(prev, next)
    }
}

// ============================================================
// CONSTITUTIONAL VALIDITY LAYERS
// ============================================================
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidityLevel {
    CryptographicallyValid = 1,
    StateValid = 2,
    ConstitutionallyValid = 3,
    SemanticallyValid = 4,
    SovereignlyValid = 5,
}

impl ValidityLevel {
    pub fn is_at_least(&self, minimum: &ValidityLevel) -> bool {
        self >= minimum
    }
}
