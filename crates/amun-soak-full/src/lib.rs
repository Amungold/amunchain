use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_defi_amm::AmmEngine;
use amun_defi_lending_engine::LendingEngine;
use amun_defi_stablecoin::StablecoinEngine;
use amun_contract_registry::ContractRegistry;
use amun_bytecode::OpCode;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct FullSoakConfig {
    pub duration_secs: u64,
    pub validators: u64,
    pub nft_mint_rate: u64,
    pub amm_swap_rate: u64,
    pub contract_deploy_rate: u64,
    pub snapshot_interval: u64,
    pub adversarial_events: bool,
}

impl Default for FullSoakConfig {
    fn default() -> Self {
        Self {
            duration_secs: 60,
            validators: 4,
            nft_mint_rate: 100,
            amm_swap_rate: 50,
            contract_deploy_rate: 10,
            snapshot_interval: 30,
            adversarial_events: true,
        }
    }
}

pub struct SoakValidator {
    pub registry: ResourceRegistry,
    pub amm: AmmEngine,
    pub lending: LendingEngine,
    pub stablecoin: StablecoinEngine,
    pub contracts: ContractRegistry,
    pub height: u64,
    pub ops: u64,
    pub failures: u64,
    pub state_roots: Vec<[u8; 32]>,
    pub memory_samples: Vec<usize>,
    pub is_alive: bool,
}

impl SoakValidator {
    pub fn new() -> Self {
        let mut reg = ResourceRegistry::new(10000);
        let col_id = ResourceId([1u8; 32]);
        reg.register_genesis(ResourceMetadata {
            resource_id: col_id,
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(col_id),
            contract_id: [0u8; 32],
            owner: [0u8; 32],
        }).ok();

        Self {
            registry: reg,
            amm: AmmEngine::new(),
            lending: LendingEngine::new(),
            stablecoin: StablecoinEngine::new(),
            contracts: ContractRegistry::new(),
            height: 0,
            ops: 0,
            failures: 0,
            state_roots: Vec::new(),
            memory_samples: Vec::new(),
            is_alive: true,
        }
    }

    pub fn run(&mut self, config: &FullSoakConfig) {
        let start = Instant::now();
        let mut rng = rand::thread_rng();

        while start.elapsed().as_secs() < config.duration_secs {
            if !self.is_alive {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.height += 1;

            // NFT Mint
            if self.height % config.nft_mint_rate == 0 {
                let col_id = ResourceId([1u8; 32]);
                let token_id = ResourceId(rng.gen::<[u8; 32]>());
                let parent_hash = self.registry.resource_hash(&col_id).unwrap_or([0u8; 32]);
                let version = self.registry.get(&col_id).map(|m| m.lineage.version + 1).unwrap_or(1);
                match self.registry.derive_from_collection(&col_id, ResourceMetadata {
                    resource_id: token_id,
                    archetype: ResourceArchetype::NFTAsset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
                    contract_id: [0u8; 32],
                    owner: rng.gen::<[u8; 32]>(),
                }) {
                    Ok(_) => self.ops += 1,
                    Err(_) => self.failures += 1,
                }
            }

            // AMM Swap
            if self.height % config.amm_swap_rate == 0 {
                if self.amm.pools.is_empty() {
                    let token_a = rng.gen::<[u8; 32]>();
                    let token_b = rng.gen::<[u8; 32]>();
                    let pool_id_bytes = amun_defi_core::DefiPool::compute_pool_id(token_a, token_b);
                    self.amm.create_pool(&mut self.registry, token_a, token_b, [0u8; 32]).ok();
                    self.amm.add_liquidity(&pool_id_bytes, 10000, 10000);
                } else {
                    let first_key = self.amm.pools.keys().next().unwrap().clone();
                    self.amm.swap(&first_key, rng.gen_range(1..100), rng.gen::<bool>());
                }
                self.ops += 1;
            }

            // Contract deploy
            if self.height % config.contract_deploy_rate == 0 {
                let code = vec![OpCode::Push(rng.gen::<u64>()), OpCode::Halt];
                let cid = ResourceId(rng.gen::<[u8; 32]>());
                self.contracts.deploy(&mut self.registry, cid, rng.gen::<[u8; 32]>(), code, self.height).ok();
                self.ops += 1;
            }

            // Lending
            if self.height % 200 == 0 {
                self.lending.create_loan(
                    &mut self.registry,
                    rng.gen::<[u8; 32]>(),
                    rng.gen_range(100..1000),
                    rng.gen_range(100..500),
                    rng.gen_range(500..2000),
                    [0u8; 32],
                    self.height,
                ).ok();
                self.ops += 1;
            }

            // Stablecoin mint
            if self.height % 300 == 0 {
                self.stablecoin.mint(
                    &mut self.registry,
                    rng.gen::<[u8; 32]>(),
                    rng.gen_range(300..1000),
                    rng.gen_range(100..600),
                ).ok();
                self.ops += 1;
            }

            // State root snapshot
            if self.height % config.snapshot_interval == 0 {
                let root = self.registry.compute_state_root();
                self.state_roots.push(root);
                self.memory_samples.push(std::mem::size_of_val(&self.registry));
            }

            // Adversarial events
            if config.adversarial_events && rng.gen_range(0..100) < 3 {
                match rng.gen_range(0..3) {
                    0 => {
                        self.is_alive = false;
                        thread::sleep(Duration::from_millis(rng.gen_range(50..500)));
                        self.is_alive = true;
                    }
                    1 => {
                        let root1 = self.registry.compute_state_root();
                        let root2 = self.registry.compute_state_root();
                        if root1 != root2 { self.failures += 1; }
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_micros(rng.gen_range(100..2000)));
        }
    }
}

pub struct FullSoakResult {
    pub duration_secs: u64,
    pub total_ops: u64,
    pub total_failures: u64,
    pub state_roots_collected: u64,
    pub state_roots_consistent: bool,
    pub memory_samples: Vec<usize>,
    pub passed: bool,
}

pub fn run_full_soak(config: FullSoakConfig) -> FullSoakResult {
    let validator = Arc::new(Mutex::new(SoakValidator::new()));
    let mut handles = vec![];

    for _ in 0..config.validators {
        let v = validator.clone();
        let cfg = FullSoakConfig { ..config };
        handles.push(thread::spawn(move || {
            v.lock().unwrap().run(&cfg);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let v = validator.lock().unwrap();
    let roots_consistent = v.state_roots.windows(2).all(|w| w[0] != [0u8; 32]);
    FullSoakResult {
        duration_secs: config.duration_secs,
        total_ops: v.ops,
        total_failures: v.failures,
        state_roots_collected: v.state_roots.len() as u64,
        state_roots_consistent: roots_consistent,
        memory_samples: v.memory_samples.clone(),
        passed: v.failures == 0 && roots_consistent,
    }
}
