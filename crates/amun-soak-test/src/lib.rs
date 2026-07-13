use amun_defi_amm::AmmEngine;
use amun_defi_lending_engine::LendingEngine;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct ValidatorSimulator {
    pub registry: Arc<Mutex<ResourceRegistry>>,
    pub amm: Arc<Mutex<AmmEngine>>,
    pub lending: Arc<Mutex<LendingEngine>>,
    pub height: Arc<Mutex<u64>>,
    pub is_alive: Arc<Mutex<bool>>,
}

impl ValidatorSimulator {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(ResourceRegistry::new(5000))),
            amm: Arc::new(Mutex::new(AmmEngine::new())),
            lending: Arc::new(Mutex::new(LendingEngine::new())),
            height: Arc::new(Mutex::new(0)),
            is_alive: Arc::new(Mutex::new(true)),
        }
    }

    pub fn run(&self, duration_secs: u64, events_enabled: bool) -> SoakResult {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let mut ops = 0u64;
        let mut failures = 0u64;

        // Initialize genesis state
        {
            let mut reg = self.registry.lock().unwrap();
            let col_id = ResourceId([1u8; 32]);
            reg.register_genesis(ResourceMetadata {
                resource_id: col_id,
                archetype: ResourceArchetype::NFTCollection,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(col_id),
                contract_id: [0u8; 32],
                owner: [0u8; 32],
            })
            .ok();
        }

        while start.elapsed().as_secs() < duration_secs {
            // Check if validator is alive
            if !*self.is_alive.lock().unwrap() {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            // Increment height
            *self.height.lock().unwrap() += 1;

            // Perform random operation
            match rng.gen_range(0..4) {
                0 => {
                    // Mint NFT
                    let mut reg = self.registry.lock().unwrap();
                    let col_id = ResourceId([1u8; 32]);
                    let token_id = ResourceId(rng.gen::<[u8; 32]>());
                    let parent_hash = reg.resource_hash(&col_id).unwrap_or([0u8; 32]);
                    let version = reg.get(&col_id).map(|m| m.lineage.version + 1).unwrap_or(1);
                    let meta = ResourceMetadata {
                        resource_id: token_id,
                        archetype: ResourceArchetype::NFTAsset,
                        state: ResourceState::Active,
                        lineage: ResourceLineage::single_ancestor(
                            token_id,
                            col_id,
                            parent_hash,
                            version,
                        ),
                        contract_id: [0u8; 32],
                        owner: rng.gen::<[u8; 32]>(),
                    };
                    match reg.derive_from_collection(&col_id, meta) {
                        Ok(_) => ops += 1,
                        Err(_) => failures += 1,
                    }
                }
                1 => {
                    // AMM swap
                    let mut amm = self.amm.lock().unwrap();
                    let token_a = rng.gen::<[u8; 32]>();
                    let token_b = rng.gen::<[u8; 32]>();
                    let pool_id = amun_defi_core::DefiPool::compute_pool_id(token_a, token_b);
                    if amm.pools.is_empty() {
                        let mut reg = self.registry.lock().unwrap();
                        amm.create_pool(&mut reg, token_a, token_b, [0u8; 32]).ok();
                        amm.add_liquidity(&pool_id, 1000, 1000);
                    } else {
                        let first_key = *amm.pools.keys().next().unwrap();
                        amm.swap(&first_key, rng.gen_range(1..100), true);
                    }
                    ops += 1;
                }
                2 => {
                    // Lending
                    let mut lending = self.lending.lock().unwrap();
                    let borrower = rng.gen::<[u8; 32]>();
                    lending
                        .create_loan(
                            &mut self.registry.lock().unwrap(),
                            borrower,
                            rng.gen_range(100..1000),
                            rng.gen_range(100..500),
                            rng.gen_range(500..2000),
                            [0u8; 32],
                            1,
                        )
                        .ok();
                    ops += 1;
                }
                _ => {
                    // State root computation
                    let _root = self.registry.lock().unwrap().compute_state_root();
                    ops += 1;
                }
            }

            // Random adversarial events if enabled
            if events_enabled && rng.gen_range(0..100) < 5 {
                match rng.gen_range(0..3) {
                    0 => {
                        // Kill validator temporarily
                        *self.is_alive.lock().unwrap() = false;
                        let kill_duration = rng.gen_range(100..2000);
                        thread::sleep(Duration::from_millis(kill_duration));
                        *self.is_alive.lock().unwrap() = true;
                    }
                    1 => {
                        // State root consistency check
                        let root1 = self.registry.lock().unwrap().compute_state_root();
                        let root2 = self.registry.lock().unwrap().compute_state_root();
                        if root1 != root2 {
                            failures += 1;
                        }
                    }
                    _ => {}
                }
            }

            // Small delay between operations
            thread::sleep(Duration::from_micros(rng.gen_range(100..5000)));
        }

        SoakResult {
            duration_secs,
            operations: ops,
            failures,
            final_height: *self.height.lock().unwrap(),
            state_root: self.registry.lock().unwrap().compute_state_root(),
        }
    }
}

pub struct SoakResult {
    pub duration_secs: u64,
    pub operations: u64,
    pub failures: u64,
    pub final_height: u64,
    pub state_root: [u8; 32],
}

impl SoakResult {
    pub fn passed(&self) -> bool {
        self.failures == 0 && self.operations > 0
    }
}

impl Default for ValidatorSimulator {
    fn default() -> Self {
        Self::new()
    }
}
