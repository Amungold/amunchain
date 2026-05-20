use blake3::Hasher;

pub struct FinalityLaw;

impl FinalityLaw {
    pub fn is_finalized(precommit_votes: u64, total_stake: u64) -> bool {
        let threshold = (total_stake * 2 / 3) + 1;
        precommit_votes >= threshold
    }

    pub fn finality_proof_hash(
        height: u64,
        block_hash: [u8; 32],
        precommit_signatures: &[[u8; 64]],
    ) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_FINALITY_PROOF_V1");
        hasher.update(&height.to_le_bytes());
        hasher.update(&block_hash);
        for sig in precommit_signatures {
            hasher.update(sig);
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        out
    }

    pub fn finality_depth_required() -> u64 {
        2
    }
}
