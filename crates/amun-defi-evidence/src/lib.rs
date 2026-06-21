use sha2::{Sha256, Digest};

pub struct DefiEvidence;

impl DefiEvidence {
    pub fn generate_swap_evidence(
        pool_id: [u8; 32],
        swapper: [u8; 32],
        amount_in: u64,
        amount_out: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_DEFI_SWAP_V1");
        hasher.update(pool_id);
        hasher.update(swapper);
        hasher.update(amount_in.to_le_bytes());
        hasher.update(amount_out.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn generate_liquidity_evidence(
        pool_id: [u8; 32],
        provider: [u8; 32],
        amount_a: u64,
        amount_b: u64,
        liquidity: u64,
        block_height: u64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_DEFI_LIQUIDITY_V1");
        hasher.update(pool_id);
        hasher.update(provider);
        hasher.update(amount_a.to_le_bytes());
        hasher.update(amount_b.to_le_bytes());
        hasher.update(liquidity.to_le_bytes());
        hasher.update(block_height.to_le_bytes());
        hasher.finalize().into()
    }
}
