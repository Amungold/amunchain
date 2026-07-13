use amun_chain_position::ChainPosition;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ValidatorAttestation {
    pub validator_id: u64,
    pub public_key: [u8; 32],
    pub position: ChainPosition,
    pub state_root: [u8; 32],
    pub signature: [u8; 64],
    pub attestation_hash: [u8; 32],
}

impl ValidatorAttestation {
    pub fn new(
        validator_id: u64,
        public_key: [u8; 32],
        position: ChainPosition,
        state_root: [u8; 32],
        signature: [u8; 64],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_ATTESTATION_V1");
        h.update(&validator_id.to_le_bytes());
        h.update(&public_key);
        h.update(&position.hash());
        h.update(&state_root);
        h.update(&signature);
        let mut attestation_hash = [0u8; 32];
        attestation_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { validator_id, public_key, position, state_root, signature, attestation_hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_ATTESTATION_V1");
        h.update(&self.validator_id.to_le_bytes());
        h.update(&self.public_key);
        h.update(&self.position.hash());
        h.update(&self.state_root);
        h.update(&self.signature);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.attestation_hash
    }
}
