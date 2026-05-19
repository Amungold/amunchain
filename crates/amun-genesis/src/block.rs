use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct GenesisBlock {
    pub chain_id: u64,
    pub timestamp: u64,
    pub initial_state_root: [u8; 32],
    pub constitution_hash: [u8; 32],
    pub validator_set_hash: [u8; 32],
    pub genesis_hash: [u8; 32],
}

impl GenesisBlock {
    pub fn new(
        chain_id: u64,
        timestamp: u64,
        initial_state_root: [u8; 32],
        constitution_hash: [u8; 32],
        validator_set_hash: [u8; 32],
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_GENESIS_V1");
        hasher.update(&chain_id.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&initial_state_root);
        hasher.update(&constitution_hash);
        hasher.update(&validator_set_hash);
        let mut genesis_hash = [0u8; 32];
        genesis_hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

        Self {
            chain_id,
            timestamp,
            initial_state_root,
            constitution_hash,
            validator_set_hash,
            genesis_hash,
        }
    }

    pub fn verify_integrity(&self) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_GENESIS_V1");
        hasher.update(&self.chain_id.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.initial_state_root);
        hasher.update(&self.constitution_hash);
        hasher.update(&self.validator_set_hash);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        computed == self.genesis_hash
    }
}
