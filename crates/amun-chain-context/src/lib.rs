use blake3::Hasher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainContext {
    pub chain_id: u64,
    pub genesis_hash: [u8; 32],
    pub constitution_hash: [u8; 32],
    pub invariant_kernel_hash: [u8; 32],
    pub domain_separator: [u8; 32],
}

impl ChainContext {
    pub fn new(
        chain_id: u64,
        genesis_hash: [u8; 32],
        constitution_hash: [u8; 32],
        invariant_kernel_hash: [u8; 32],
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CHAIN_DOMAIN_V1");
        hasher.update(&chain_id.to_le_bytes());
        hasher.update(&genesis_hash);
        hasher.update(&constitution_hash);
        hasher.update(&invariant_kernel_hash);
        let mut domain_separator = [0u8; 32];
        domain_separator.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

        Self {
            chain_id,
            genesis_hash,
            constitution_hash,
            invariant_kernel_hash,
            domain_separator,
        }
    }

    pub fn domain_tag(&self, purpose: &[u8]) -> Vec<u8> {
        let mut tag = Vec::with_capacity(64);
        tag.extend_from_slice(&self.domain_separator[..16]);
        tag.extend_from_slice(purpose);
        tag
    }

    pub fn verify_binding(&self) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CHAIN_DOMAIN_V1");
        hasher.update(&self.chain_id.to_le_bytes());
        hasher.update(&self.genesis_hash);
        hasher.update(&self.constitution_hash);
        hasher.update(&self.invariant_kernel_hash);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        computed == self.domain_separator
    }
}
