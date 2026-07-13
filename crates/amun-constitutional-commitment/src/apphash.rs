use crate::commitment::ConstitutionalCommitment;
use crate::roots::commitment_root;
use crate::Hash32;

pub struct AppHashPipeline;

impl AppHashPipeline {
    pub fn compute_state_root(
        accounts_root: Hash32,
        staking_root: Hash32,
        governance_state_root: Hash32,
        commitment: &ConstitutionalCommitment,
    ) -> Hash32 {
        let commitment_root_val = commitment_root(commitment);

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_STATE_ROOT_V1");
        hasher.update(&accounts_root);
        hasher.update(&staking_root);
        hasher.update(&governance_state_root);
        hasher.update(&commitment_root_val);
        *hasher.finalize().as_bytes()
    }

    pub fn state_root_to_app_hash(state_root: Hash32) -> Hash32 {
        state_root
    }
}
