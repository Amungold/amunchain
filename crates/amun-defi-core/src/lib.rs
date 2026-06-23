use amun_resource_core::ResourceId;
use sha2::{Digest, Sha256};

pub struct DefiPool {
    pub pool_id: ResourceId,
    pub token_a: ResourceId,
    pub token_b: ResourceId,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_liquidity: u64,
}

pub struct LiquidityToken {
    pub token_id: ResourceId,
    pub pool_id: ResourceId,
    pub owner: [u8; 32],
    pub amount: u64,
}

impl DefiPool {
    pub fn new(pool_id: ResourceId, token_a: ResourceId, token_b: ResourceId) -> Self {
        Self {
            pool_id,
            token_a,
            token_b,
            reserve_a: 0,
            reserve_b: 0,
            total_liquidity: 0,
        }
    }

    pub fn add_liquidity(&mut self, amount_a: u64, amount_b: u64) -> u64 {
        let liquidity = if self.total_liquidity == 0 {
            ((amount_a as u128 * amount_b as u128) as f64).sqrt() as u64
        } else {
            let share_a =
                (amount_a as u128 * self.total_liquidity as u128) / self.reserve_a as u128;
            let share_b =
                (amount_b as u128 * self.total_liquidity as u128) / self.reserve_b as u128;
            std::cmp::min(share_a, share_b) as u64
        };
        self.reserve_a += amount_a;
        self.reserve_b += amount_b;
        self.total_liquidity += liquidity;
        liquidity
    }

    pub fn remove_liquidity(&mut self, liquidity: u64) -> (u64, u64) {
        let amount_a = (liquidity as u128 * self.reserve_a as u128) / self.total_liquidity as u128;
        let amount_b = (liquidity as u128 * self.reserve_b as u128) / self.total_liquidity as u128;
        self.reserve_a -= amount_a as u64;
        self.reserve_b -= amount_b as u64;
        self.total_liquidity -= liquidity;
        (amount_a as u64, amount_b as u64)
    }

    pub fn swap_a_for_b(&mut self, amount_a_in: u64) -> u64 {
        let amount_a_in_with_fee = (amount_a_in as u128 * 997) / 1000;
        let numerator = amount_a_in_with_fee * self.reserve_b as u128;
        let denominator = self.reserve_a as u128 + amount_a_in_with_fee;
        let amount_b_out = (numerator / denominator) as u64;
        self.reserve_a += amount_a_in;
        self.reserve_b -= amount_b_out;
        amount_b_out
    }

    pub fn swap_b_for_a(&mut self, amount_b_in: u64) -> u64 {
        let amount_b_in_with_fee = (amount_b_in as u128 * 997) / 1000;
        let numerator = amount_b_in_with_fee * self.reserve_a as u128;
        let denominator = self.reserve_b as u128 + amount_b_in_with_fee;
        let amount_a_out = (numerator / denominator) as u64;
        self.reserve_b += amount_b_in;
        self.reserve_a -= amount_a_out;
        amount_a_out
    }

    pub fn compute_pool_id(token_a: [u8; 32], token_b: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_DEFI_POOL_V1");
        hasher.update(token_a);
        hasher.update(token_b);
        hasher.finalize().into()
    }
}
