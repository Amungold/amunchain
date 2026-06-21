use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry, RegistryError,
};
use amun_defi_core::DefiPool;
use std::collections::BTreeMap;

pub struct AmmEngine {
    pub pools: BTreeMap<[u8; 32], DefiPool>,
}

impl Default for AmmEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AmmEngine {
    pub fn new() -> Self {
        Self { pools: BTreeMap::new() }
    }

    pub fn create_pool(
        &mut self,
        registry: &mut ResourceRegistry,
        token_a: [u8; 32],
        token_b: [u8; 32],
        creator: [u8; 32],
    ) -> Result<ResourceId, RegistryError> {
        let pool_id_bytes = DefiPool::compute_pool_id(token_a, token_b);
        let pool_id = ResourceId(pool_id_bytes);
        let meta = ResourceMetadata {
            resource_id: pool_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(pool_id),
            contract_id: [0u8; 32],
            owner: creator,
        };
        registry.register_genesis(meta)?;
        self.pools.insert(pool_id_bytes, DefiPool::new(pool_id, ResourceId(token_a), ResourceId(token_b)));
        Ok(pool_id)
    }

    pub fn add_liquidity(
        &mut self,
        pool_id: &[u8; 32],
        amount_a: u64,
        amount_b: u64,
    ) -> Option<u64> {
        self.pools.get_mut(pool_id).map(|pool| pool.add_liquidity(amount_a, amount_b))
    }

    pub fn swap(
        &mut self,
        pool_id: &[u8; 32],
        amount_in: u64,
        swap_a_for_b: bool,
    ) -> Option<u64> {
        self.pools.get_mut(pool_id).map(|pool| {
            if swap_a_for_b { pool.swap_a_for_b(amount_in) }
            else { pool.swap_b_for_a(amount_in) }
        })
    }

    pub fn compute_evidence_root(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_AMM_EVIDENCE_V1");
        for (id, pool) in &self.pools {
            hasher.update(id);
            hasher.update(pool.reserve_a.to_le_bytes());
            hasher.update(pool.reserve_b.to_le_bytes());
            hasher.update(pool.total_liquidity.to_le_bytes());
        }
        hasher.finalize().into()
    }
}
