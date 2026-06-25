use amun_defi_amm::AmmEngine;
use amun_defi_lending_engine::LendingEngine;
use amun_defi_stablecoin::StablecoinEngine;
use amun_nft_collateral::NftCollateralEngine;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use rand::Rng;

pub struct DefiStressResult {
    pub total_operations: u64,
    pub successful: u64,
    pub failed: u64,
    pub invariants_broken: u64,
}

impl Default for DefiStressResult {
    fn default() -> Self {
        Self::new()
    }
}

impl DefiStressResult {
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            successful: 0,
            failed: 0,
            invariants_broken: 0,
        }
    }

    pub fn passed(&self) -> bool {
        self.failed == 0 && self.invariants_broken == 0
    }
}

pub fn stress_amm_swaps(iterations: u64) -> DefiStressResult {
    let mut result = DefiStressResult::new();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(1000);
        let mut amm = AmmEngine::new();
        let token_a = rng.gen::<[u8; 32]>();
        let token_b = rng.gen::<[u8; 32]>();
        if let Ok(pool_id) = amm.create_pool(&mut reg, token_a, token_b, [0u8; 32]) {
            let amount = rng.gen_range(1..1_000_000);
            amm.add_liquidity(&pool_id.0, amount, amount);
            let max_swap = std::cmp::max(2, amount / 10);
            let swap_amount = rng.gen_range(1..max_swap);
            let root_before = amm.compute_evidence_root();
            let out = amm.swap(&pool_id.0, swap_amount, rng.gen::<bool>());
            let root_after = amm.compute_evidence_root();
            result.total_operations += 1;
            if out.is_some() {
                result.successful += 1;
                if root_before == root_after {
                    result.invariants_broken += 1;
                }
            } else {
                result.failed += 1;
            }
        }
    }
    result
}

pub fn stress_lending_liquidations(iterations: u64) -> DefiStressResult {
    let mut result = DefiStressResult::new();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(1000);
        let mut engine = LendingEngine::new();
        let borrower = rng.gen::<[u8; 32]>();
        let principal = rng.gen_range(100..10_000);
        let rate = rng.gen_range(100..2000);
        let collateral = rng.gen_range(50..500);
        if let Ok((loan_id, _)) = engine.create_loan(
            &mut reg, borrower, principal, rate, collateral, [0u8; 32], 1,
        ) {
            engine.accrue_interest(&loan_id.0, rng.gen_range(1..5_000_000));
            let health = engine.get_health_factor(&loan_id.0, 5_000_000);
            result.total_operations += 1;
            if amun_defi_lending_core::InterestModel::is_liquidatable(health) {
                if engine
                    .liquidate(&loan_id.0, rng.gen::<[u8; 32]>(), 5_000_000)
                    .is_ok()
                {
                    result.successful += 1;
                } else {
                    result.failed += 1;
                }
            } else {
                result.successful += 1;
            }
        }
    }
    result
}

pub fn stress_stablecoin_mint_burn(iterations: u64) -> DefiStressResult {
    let mut result = DefiStressResult::new();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(100);
        let mut engine = StablecoinEngine::new();
        let owner = rng.gen::<[u8; 32]>();
        let collateral = rng.gen_range(300..10_000);
        let mint_amount = collateral * 2 / 3;
        if let Ok(pos_id) = engine.mint(&mut reg, owner, collateral, mint_amount) {
            let root_before = engine.compute_stablecoin_root();
            let burn_amount = rng.gen_range(1..=mint_amount);
            let _ = engine.burn(&pos_id, burn_amount);
            let root_after = engine.compute_stablecoin_root();
            result.total_operations += 1;
            result.successful += 1;
            if root_before == root_after {
                result.invariants_broken += 1;
            }
        }
    }
    result
}

pub fn stress_nft_collateral_flow(iterations: u64) -> DefiStressResult {
    let mut result = DefiStressResult::new();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(100);
        let token_id = ResourceId(rng.gen::<[u8; 32]>());
        let owner = rng.gen::<[u8; 32]>();
        reg.register_genesis(ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(token_id),
            contract_id: [0u8; 32],
            owner,
        })
        .unwrap();
        let mut engine = NftCollateralEngine::new();
        engine.lock_nft(&reg, token_id, &owner).unwrap();
        let loan_amount = rng.gen_range(100..1000);
        if let Ok(loan_id) =
            engine.borrow_against_nft(&mut reg, token_id, owner, loan_amount, 500, 1)
        {
            let repay = rng.gen_range(1..=loan_amount);
            let _ = engine.repay_and_unlock(&mut reg, &loan_id, token_id, repay);
            result.total_operations += 1;
            result.successful += 1;
        }
    }
    result
}
