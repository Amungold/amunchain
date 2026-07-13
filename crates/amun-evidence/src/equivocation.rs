pub struct EquivocationProof {
    pub validator_pk: [u8; 32],
    pub round: u64,
    pub block_hash_a: [u8; 32],
    pub block_hash_b: [u8; 32],
    pub signature_a: [u8; 64],
    pub signature_b: [u8; 64],
}

impl EquivocationProof {
    pub fn verify(&self, chain_id: u64) -> bool {
        self.block_hash_a != self.block_hash_b
            && amun_crypto::Ed25519Signer::verify(
                &self.validator_pk,
                &self.block_hash_a,
                &self.signature_a,
                b"AMUN_VOTE_V4",
                chain_id,
            )
            .is_ok()
            && amun_crypto::Ed25519Signer::verify(
                &self.validator_pk,
                &self.block_hash_b,
                &self.signature_b,
                b"AMUN_VOTE_V4",
                chain_id,
            )
            .is_ok()
    }

    pub fn compute_id(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"EQUIVOCATION_V4");
        hasher.update(&self.validator_pk);
        hasher.update(&self.round.to_le_bytes());
        let h = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.as_bytes()[..32]);
        out
    }
}
