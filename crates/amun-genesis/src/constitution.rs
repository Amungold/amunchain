use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct GenesisConstitution {
    pub version: u64,
    pub invariant_kernel_hash: [u8; 32],
    pub complexity_budget_json: String,
    pub constitution_hash: [u8; 32],
}

impl GenesisConstitution {
    pub fn new(invariant_kernel_hash: [u8; 32], complexity_budget_json: String) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CONSTITUTION_V1");
        hasher.update(&1u64.to_le_bytes());
        hasher.update(&invariant_kernel_hash);
        hasher.update(complexity_budget_json.as_bytes());
        let mut constitution_hash = [0u8; 32];
        constitution_hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

        Self {
            version: 1,
            invariant_kernel_hash,
            complexity_budget_json,
            constitution_hash,
        }
    }
}
