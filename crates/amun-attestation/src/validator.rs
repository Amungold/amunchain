use amun_crypto::Ed25519Signer;

pub struct ValidatorAttestation {
    pub public_key: [u8; 32],
    pub chain_id: u64,
    pub epoch: u64,
    pub signature: [u8; 64],
}

impl ValidatorAttestation {
    pub fn create(signer: &Ed25519Signer, chain_id: u64, epoch: u64) -> Option<Self> {
        let pk = signer.public_bytes();
        let sig = signer.sign(&pk, b"AMUN_VALIDATOR_ATTEST_V4", chain_id).ok()?;
        Some(Self {
            public_key: pk,
            chain_id,
            epoch,
            signature: sig,
        })
    }

    pub fn verify(&self) -> bool {
        Ed25519Signer::verify(
            &self.public_key,
            &self.public_key,
            &self.signature,
            b"AMUN_VALIDATOR_ATTEST_V4",
            self.chain_id,
        ).is_ok()
    }
}
