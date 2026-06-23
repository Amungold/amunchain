use crate::commitment::ConstitutionalCommitment;
use crate::roots::{commitment_root, compute_constitutional_root};
use crate::Hash32;

/// All constitutional roots for a block, computed by the CCA layer.
/// This is the canonical output of the EndBlock pipeline.
pub struct ConstitutionalRoots {
    pub state_root: Hash32,
    pub commitment_root: Hash32,
    pub economic_root: Hash32,
    pub identity_root: Hash32,
    pub governance_root: Hash32,
    pub constitutional_root: Hash32,
}

impl ConstitutionalRoots {
    pub fn from_commitment(
        raw_state_root: Hash32,
        economic_root: Hash32,
        identity_root: Hash32,
        governance_root: Hash32,
        commitment: &ConstitutionalCommitment,
    ) -> Self {
        let commitment_root_val = commitment_root(commitment);
        let constitutional_root_val =
            compute_constitutional_root(identity_root, [0u8; 32], governance_root, economic_root);

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CCA_STATE_ROOT_V1");
        hasher.update(&raw_state_root);
        hasher.update(&commitment_root_val);
        let state_root_val = *hasher.finalize().as_bytes();

        Self {
            state_root: state_root_val,
            commitment_root: commitment_root_val,
            economic_root,
            identity_root,
            governance_root,
            constitutional_root: constitutional_root_val,
        }
    }
}
