# ResourceRegistry

## Definition
crates/amun-bench/tests/n161_state_root_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n161_state_root_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-bench/tests/n162_snapshot_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n162_snapshot_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-benchmarks/benches/sync_bench.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-benchmarks/benches/sync_bench.rs:5:fn create_test_registry(size: u64) -> ResourceRegistry {
crates/amun-benchmarks/benches/sync_bench.rs:6:    let mut reg = ResourceRegistry::new(size as usize * 2);
crates/amun-byzantine-tests/tests/attack_suite.rs:107:    let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:11:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-byzantine-tests/tests/attack_suite.rs:126:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:137:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:154:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:182:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:193:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:210:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:224:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:245:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:55:    let mut fresh_reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:82:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:94:    let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
crates/amun-consensus-integration/src/consensus_integrator.rs:114:        let mut registry = ResourceRegistry::new(10000);
crates/amun-consensus-integration/src/consensus_integrator.rs:140:        let mut registry = ResourceRegistry::new(10000);
crates/amun-consensus-integration/src/consensus_integrator.rs:167:        let mut registry = ResourceRegistry::new(10000);
crates/amun-consensus-integration/src/consensus_integrator.rs:17:        registry: &mut ResourceRegistry,
crates/amun-consensus-integration/src/consensus_integrator.rs:192:        let mut registry = ResourceRegistry::new(10000);
crates/amun-consensus-integration/src/consensus_integrator.rs:219:        let mut registry = ResourceRegistry::new(10000);
crates/amun-consensus-integration/src/consensus_integrator.rs:5:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-constitutional-runtime/src/block_validator.rs:102:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/block_validator.rs:138:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/block_validator.rs:183:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/block_validator.rs:31:        registry: &mut ResourceRegistry,
crates/amun-constitutional-runtime/src/block_validator.rs:5:use amun_resource_core::ResourceRegistry;
crates/amun-constitutional-runtime/src/finality_certificate.rs:140:    use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-constitutional-runtime/src/finality_certificate.rs:151:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:240:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:282:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:316:        let mut registry = ResourceRegistry::new(1000);
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:40:        registry: &mut ResourceRegistry,
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:8:use amun_resource_core::{ResourceMetadata, ResourceRegistry};
crates/amun-contract-fuzzing/src/lib.rs:133:        let mut reg = ResourceRegistry::new(100);
crates/amun-contract-fuzzing/src/lib.rs:4:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-fuzzing/src/lib.rs:61:        let mut reg = ResourceRegistry::new(100);
crates/amun-contract-fuzzing/src/lib.rs:89:        let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/src/lib.rs:14:        registry: &mut ResourceRegistry,
crates/amun-contract-integration/src/lib.rs:45:        registry: &mut ResourceRegistry,
crates/amun-contract-integration/src/lib.rs:6:    ResourceRegistry, ResourceState,
crates/amun-contract-integration/src/lib.rs:93:    pub fn compute_contract_evidence_root(registry: &ResourceRegistry) -> [u8; 32] {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:39:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:40:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:53:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:54:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:74:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:75:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:33:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-registry/src/lib.rs:40:        registry: &mut ResourceRegistry,
crates/amun-contract-registry/src/lib.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-contract-sdk/src/lib.rs:14:            registry: ResourceRegistry::new(1000),
crates/amun-contract-sdk/src/lib.rs:4:use amun_resource_core::{RegistryError, ResourceArchetype, ResourceId, ResourceRegistry};
crates/amun-contract-sdk/src/lib.rs:7:    pub registry: ResourceRegistry,
crates/amun-contract-security/src/lib.rs:104:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:105:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:123:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:135:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:13:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:49:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:4:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-security/src/lib.rs:82:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-upgrade/src/lib.rs:16:        _registry: &mut ResourceRegistry,
crates/amun-contract-upgrade/src/lib.rs:3:use amun_resource_core::{RegistryError, ResourceArchetype, ResourceId, ResourceRegistry};
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:5:use amun_resource_core::ResourceRegistry;
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:9:    let mut reg = ResourceRegistry::new(100);
crates/amun-core-optimization/src/lib.rs:10:    pub registry: ResourceRegistry,
crates/amun-core-optimization/src/lib.rs:18:            registry: ResourceRegistry::new(max_lineage_depth),
crates/amun-core-optimization/src/lib.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-core-optimization/tests/n161_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(20000);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-defi-amm/src/lib.rs:27:        registry: &mut ResourceRegistry,
crates/amun-defi-amm/src/lib.rs:4:    ResourceRegistry, ResourceState,
crates/amun-defi-amm/tests/n153_amm_tests.rs:20:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:21:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-amm/tests/n153_amm_tests.rs:37:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:6:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-lending-engine/src/lib.rs:2:use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry};
crates/amun-defi-lending-engine/src/lib.rs:38:        _registry: &mut ResourceRegistry,
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:10:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:30:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:49:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:68:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:88:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:99:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-stablecoin/src/lib.rs:1:use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry};
crates/amun-defi-stablecoin/src/lib.rs:34:        _registry: &mut ResourceRegistry,
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:17:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:25:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:26:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:6:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stress/src/lib.rs:106:        let mut reg = ResourceRegistry::new(100);
crates/amun-defi-stress/src/lib.rs:130:        let mut reg = ResourceRegistry::new(100);
crates/amun-defi-stress/src/lib.rs:43:        let mut reg = ResourceRegistry::new(1000);
crates/amun-defi-stress/src/lib.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-defi-stress/src/lib.rs:73:        let mut reg = ResourceRegistry::new(1000);
crates/amun-dual-verification/src/dual_verifier.rs:17:        registry: &mut ResourceRegistry,
crates/amun-dual-verification/src/dual_verifier.rs:8:use amun_resource_core::ResourceRegistry;
crates/amun-evidence-finality/src/evidence_finality.rs:148:        registry: &mut ResourceRegistry,
crates/amun-evidence-finality/src/evidence_finality.rs:183:            let mut fresh_reg = ResourceRegistry::new(10000);
crates/amun-evidence-finality/src/evidence_finality.rs:286:        let mut registry = ResourceRegistry::new(10000);
crates/amun-evidence-finality/src/evidence_finality.rs:313:        let mut registry = ResourceRegistry::new(10000);
crates/amun-evidence-finality/src/evidence_finality.rs:344:        let mut registry = ResourceRegistry::new(10000);
crates/amun-evidence-finality/src/evidence_finality.rs:8:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-experimental-framework/src/main.rs:213:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:310:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:339:            let mut reg = ResourceRegistry::new((n * 10) as usize);
crates/amun-experimental-framework/src/main.rs:370:                    let mut fresh = ResourceRegistry::new(10000);
crates/amun-experimental-framework/src/main.rs:398:            let tip_hash = ResourceRegistry::hash_resource(tip_meta);
crates/amun-experimental-framework/src/main.rs:65:fn build_registry(size: u64) -> ResourceRegistry {
crates/amun-experimental-framework/src/main.rs:66:    let mut reg = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:81:fn build_deep_chain(depth: u64) -> (ResourceRegistry, ResourceId) {
crates/amun-experimental-framework/src/main.rs:82:    let mut reg = ResourceRegistry::new((depth * 2) as usize);
crates/amun-experimental-framework/src/main.rs:8:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-experimental-framework/src/main.rs:96:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:103:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:124:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:58:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:84:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:123:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:17:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:64:    let mut reg = ResourceRegistry::new(2000);
crates/amun-nft-collateral/src/lib.rs:2:use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry};
crates/amun-nft-collateral/src/lib.rs:34:        registry: &ResourceRegistry,
crates/amun-nft-collateral/src/lib.rs:67:        registry: &mut ResourceRegistry,
crates/amun-nft-collateral/src/lib.rs:95:        _registry: &mut ResourceRegistry,
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:109:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:110:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:35:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:81:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-evidence/src/lib.rs:114:        registry: &ResourceRegistry,
crates/amun-nft-evidence/src/lib.rs:131:        registry: &ResourceRegistry,
crates/amun-nft-evidence/src/lib.rs:2:use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry, ResourceState};
crates/amun-nft-evidence/src/lib.rs:32:        registry: &ResourceRegistry,
crates/amun-nft-evidence/src/lib.rs:47:        registry: &ResourceRegistry,
crates/amun-nft-evidence/src/lib.rs:9:    pub registry: &'a ResourceRegistry,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:131:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:57:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:78:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/src/lib.rs:113:pub fn start_explorer_server(registry: Arc<Mutex<ResourceRegistry>>, bind_addr: &str) {
crates/amun-nft-explorer/src/lib.rs:145:fn handle_request(path: &str, registry: &ResourceRegistry) -> String {
crates/amun-nft-explorer/src/lib.rs:2:use amun_resource_core::{ResourceArchetype, ResourceId, ResourceRegistry};
crates/amun-nft-explorer/src/lib.rs:44:    pub fn get_collections(registry: &ResourceRegistry) -> Vec<ExplorerCollection> {
crates/amun-nft-explorer/src/lib.rs:59:    pub fn get_nft(registry: &ResourceRegistry, token_id: &ResourceId) -> Option<ExplorerNft> {
crates/amun-nft-explorer/src/lib.rs:70:    pub fn get_owner_nfts(registry: &ResourceRegistry, owner: &[u8; 32]) -> ExplorerOwner {
crates/amun-nft-explorer/src/lib.rs:86:        registry: &ResourceRegistry,
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:28:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:46:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-fuzz/src/lib.rs:109:        let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-fuzz/src/lib.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-fuzz/src/lib.rs:35:        let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-marketplace/src/lib.rs:133:        registry: &mut ResourceRegistry,
crates/amun-nft-marketplace/src/lib.rs:215:        registry: &ResourceRegistry,
crates/amun-nft-marketplace/src/lib.rs:281:        registry: &mut ResourceRegistry,
crates/amun-nft-marketplace/src/lib.rs:3:    ResourceRegistry, ResourceState,
crates/amun-nft-marketplace/src/lib.rs:93:        registry: &ResourceRegistry,
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:118:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:141:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:165:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:38:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:62:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:91:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/src/lib.rs:4:    ResourceRegistry, ResourceState,
crates/amun-nft-mining/src/lib.rs:63:    registry: &mut ResourceRegistry,
crates/amun-nft-mining/tests/n133_mining_tests.rs:18:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-mining/tests/n133_mining_tests.rs:55:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:17:    ResourceRegistry,
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:22:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:5:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-sdk/src/lib.rs:13:    pub registry: Arc<Mutex<ResourceRegistry>>,
crates/amun-nft-sdk/src/lib.rs:29:            registry: Arc::new(Mutex::new(ResourceRegistry::new(1000))),
crates/amun-nft-sdk/src/lib.rs:7:    ResourceRegistry, ResourceState,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:18:    ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:31:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:16:    registry: &mut ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:56:    registry: &mut ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:107:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:111:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:18:    let mut reg = ResourceRegistry::new(10000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:77:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:78:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-operations/src/backup_recovery.rs:118:        let mut reg = ResourceRegistry::new(100);
crates/amun-operations/src/backup_recovery.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-operations/src/backup_recovery.rs:57:    pub fn restore(&self) -> Result<ResourceRegistry, String> {
crates/amun-operations/src/backup_recovery.rs:61:                let mut registry = ResourceRegistry::new(self.sync_package.total_resources() * 2);
crates/amun-operations/src/backup_recovery.rs:80:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-operations/src/backup_recovery.rs:92:        let mut reg = ResourceRegistry::new(100);
crates/amun-pccv/src/lib.rs:111:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/lib.rs:15:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-pccv/src/lib.rs:163:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/lib.rs:215:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/lib.rs:227:        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-pccv/src/lib.rs:49:        let reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/lib.rs:56:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
crates/amun-pccv/src/pccv_verifier.rs:3:    ResourceArchetype, ResourceRegistry, ResourceState, TransformationMatrix,
crates/amun-pccv/src/transition_proof_engine.rs:107:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-pccv/src/transition_proof_engine.rs:120:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/transition_proof_engine.rs:133:        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-pccv/src/transition_proof_engine.rs:14:        registry: &ResourceRegistry,
crates/amun-pccv/src/transition_proof_engine.rs:171:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/transition_proof_engine.rs:184:        let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
crates/amun-pccv/src/transition_proof_engine.rs:240:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/transition_proof_engine.rs:2:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-pccv/src/transition_proof_engine.rs:74:        registry: &ResourceRegistry,
crates/amun-pccv/src/witness_builder.rs:105:    fn build_lineage_proof(registry: &ResourceRegistry, resource_id: &ResourceId) -> LineageProof {
crates/amun-pccv/src/witness_builder.rs:146:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/witness_builder.rs:167:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/witness_builder.rs:182:        let root_hash = ResourceRegistry::hash_resource(reg.get(&root).unwrap());
crates/amun-pccv/src/witness_builder.rs:196:        let child_hash = ResourceRegistry::hash_resource(reg.get(&child).unwrap());
crates/amun-pccv/src/witness_builder.rs:220:        let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/src/witness_builder.rs:2:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-pccv/src/witness_builder.rs:34:        registry: &ResourceRegistry,
crates/amun-pccv/src/witness_builder.rs:47:        registry: &ResourceRegistry,
crates/amun-pccv/src/witness_builder.rs:62:                    .map(|meta| (ResourceRegistry::hash_resource(meta), *id))
crates/amun-pccv/src/witness_builder.rs:8:        registry: &ResourceRegistry,
crates/amun-pccv/tests/replay_equivalence.rs:108:    let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:119:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:17:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:18:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:37:    let parent_hash = ResourceRegistry::hash_resource(reg1.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-persistent-node/src/persistent_store.rs:112:    pub fn registry(&self) -> &ResourceRegistry {
crates/amun-persistent-node/src/persistent_store.rs:115:    pub fn registry_mut(&mut self) -> &mut ResourceRegistry {
crates/amun-persistent-node/src/persistent_store.rs:25:                registry: ResourceRegistry::new(1_000_000),
crates/amun-persistent-node/src/persistent_store.rs:2:use amun_resource_core::{ResourceMetadata, ResourceRegistry};
crates/amun-persistent-node/src/persistent_store.rs:9:    registry: ResourceRegistry,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:167:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:193:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:219:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:23:        registry: &mut ResourceRegistry,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:247:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:274:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:55:            let mut fresh_reg = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:6:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-replay-verifier/src/replay_verifier.rs:132:        let mut r1 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:133:        let mut r2 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:162:        let mut r = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:184:        let mut r2 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:205:        let mut r = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:215:            let mut f = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:34:        registry: &mut ResourceRegistry,
crates/amun-replay-verifier/src/replay_verifier.rs:7:    ResourceArchetype, ResourceLineage, ResourceMetadata, ResourceRegistry, ResourceState,
crates/amun-resource-core/src/resource_registry.rs:366:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:376:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:387:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:399:        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-resource-core/src/resource_registry.rs:415:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:420:        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-resource-core/src/resource_registry.rs:434:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:439:        let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
crates/amun-resource-core/src/resource_registry.rs:449:        let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
crates/amun-resource-core/src/resource_registry.rs:463:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:470:            let hash = ResourceRegistry::hash_resource(reg.get(&current_parent).unwrap());
crates/amun-resource-core/src/resource_registry.rs:487:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:501:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:510:        let mut reg2 = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:519:        let mut reg = ResourceRegistry::new(1000);
crates/amun-resource-core/src/resource_registry.rs:66:pub struct ResourceRegistry {
crates/amun-resource-core/src/resource_registry.rs:73:impl ResourceRegistry {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:115:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:116:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:119:    let setup = |reg: &mut ResourceRegistry| {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:3:    ResourceRegistry, ResourceState,
crates/amun-resource-core/tests/n130_constitutional_nft.rs:45:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:74:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:8:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/stress_tests.rs:107:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:133:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:149:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:174:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:186:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:211:        let parent_hash = ResourceRegistry::hash_resource(parent);
crates/amun-resource-core/tests/stress_tests.rs:222:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:234:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:254:            ResourceRegistry::hash_resource(tip),
crates/amun-resource-core/tests/stress_tests.rs:26:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-resource-core/tests/stress_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-resource-core/tests/stress_tests.rs:45:    let mut reg = ResourceRegistry::new(10_000);
crates/amun-resource-core/tests/stress_tests.rs:58:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:89:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-snapshot-optimization/src/lib.rs:1:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-snapshot-optimization/src/lib.rs:23:    pub fn apply_to(&self, registry: &mut ResourceRegistry) {
crates/amun-snapshot-optimization/src/lib.rs:34:pub fn compress_snapshot(registry: &ResourceRegistry) -> (Vec<ResourceMetadata>, [u8; 32]) {
crates/amun-snapshot-optimization/src/lib.rs:44:pub fn restore_from_compressed(compressed: &[ResourceMetadata]) -> ResourceRegistry {
crates/amun-snapshot-optimization/src/lib.rs:45:    let mut reg = ResourceRegistry::new(compressed.len() * 2);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(10000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:69:    let mut reg = ResourceRegistry::new(1000);
crates/amun-soak-full/src/lib.rs:40:    pub registry: ResourceRegistry,
crates/amun-soak-full/src/lib.rs:55:        let mut reg = ResourceRegistry::new(10000);
crates/amun-soak-full/src/lib.rs:7:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-soak-test/src/lib.rs:13:    pub registry: Arc<Mutex<ResourceRegistry>>,
crates/amun-soak-test/src/lib.rs:23:            registry: Arc::new(Mutex::new(ResourceRegistry::new(5000))),
crates/amun-soak-test/src/lib.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-state-pruning/src/lib.rs:13:            registry: ResourceRegistry::new(max_lineage_depth),
crates/amun-state-pruning/src/lib.rs:1:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-state-pruning/src/lib.rs:5:    pub registry: ResourceRegistry,
crates/amun-state-sync/src/lib.rs:132:        let mut reg = ResourceRegistry::new(1000);
crates/amun-state-sync/src/lib.rs:204:        let mut reg = ResourceRegistry::new(100);
crates/amun-state-sync/src/lib.rs:238:        let mut reg = ResourceRegistry::new(100);
crates/amun-state-sync/src/lib.rs:46:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-state-sync/src/stateless_verifier.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-state-sync/src/stateless_verifier.rs:87:        let mut temp_reg = ResourceRegistry::new(all_resources.len() * 2);
crates/amun-state-sync/src/sync_protocol.rs:100:    ) -> Result<ResourceRegistry, String> {
crates/amun-state-sync/src/sync_protocol.rs:147:                let mut registry = ResourceRegistry::new(snapshot.total_resources as usize * 2);
crates/amun-state-sync/src/sync_protocol.rs:165:        registry: &ResourceRegistry,
crates/amun-state-sync/src/sync_protocol.rs:187:        registry: &mut ResourceRegistry,
crates/amun-state-sync/src/sync_protocol.rs:205:        registry: &ResourceRegistry,
crates/amun-state-sync/src/sync_protocol.rs:257:    fn create_test_registry(n: u64) -> ResourceRegistry {
crates/amun-state-sync/src/sync_protocol.rs:258:        let mut r = ResourceRegistry::new(n as usize * 2);
crates/amun-state-sync/src/sync_protocol.rs:5:use amun_resource_core::{ResourceMetadata, ResourceRegistry};
crates/amun-state-sync/src/sync_protocol.rs:60:        registry: &ResourceRegistry,
crates/amun-testnet-sim/tests/adversarial_tests.rs:22:fn build_registry(count: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:23:    let mut reg = ResourceRegistry::new((count * 2) as usize);
crates/amun-testnet-sim/tests/adversarial_tests.rs:9:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-transition-proof/src/enhanced_proof.rs:15:        let leaf_hash = ResourceRegistry::hash_resource(metadata);
crates/amun-transition-proof/src/enhanced_proof.rs:2:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-transition-proof/src/proof_builder.rs:119:        registry: &ResourceRegistry,
crates/amun-transition-proof/src/proof_builder.rs:131:    fn compute_siblings(registry: &ResourceRegistry, rid: &ResourceId) -> Vec<([u8; 32], bool)> {
crates/amun-transition-proof/src/proof_builder.rs:142:                    .map(|m| (ResourceRegistry::hash_resource(m), *id))
crates/amun-transition-proof/src/proof_builder.rs:178:    fn build_lineage_proof(registry: &ResourceRegistry, rid: &ResourceId) -> LineageProof {
crates/amun-transition-proof/src/proof_builder.rs:4:use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
crates/amun-transition-proof/src/proof_builder.rs:56:        registry: &ResourceRegistry,
crates/amun-validator-networking/src/lib.rs:48:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-validator-networking/src/lib.rs:58:    fn build_registry(count: u8) -> ResourceRegistry {
crates/amun-validator-networking/src/lib.rs:59:        let mut reg = ResourceRegistry::new(10000);
crates/amun-validator-networking/src/rejoin_protocol.rs:27:        peer_registry: &ResourceRegistry,
crates/amun-validator-networking/src/rejoin_protocol.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-validator-networking/src/sync_transport.rs:11:        registry: &ResourceRegistry,
crates/amun-validator-networking/src/sync_transport.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-validator-networking/src/sync_transport.rs:45:    ) -> Result<ResourceRegistry, String> {
crates/amun-validator-networking/src/sync_transport.rs:52:                let mut registry = ResourceRegistry::new(package.total_resources() * 2);
crates/amun-vm-kernel/src/vm_kernel.rs:108:        registry: &mut ResourceRegistry,
crates/amun-vm-kernel/src/vm_kernel.rs:1:use amun_resource_core::{RegistryError, ResourceMetadata, ResourceRegistry, TransformationMatrix};
crates/amun-vm-kernel/src/vm_kernel.rs:30:    pub fn verify(buffer: &mut PendingBuffer, registry: &ResourceRegistry) -> bool {
crates/amun-vm-kernel/src/vm_kernel.rs:77:                    let actual_hash = amun_resource_core::ResourceRegistry::hash_resource(parent);
docs/audit/SECURITY_INVARIANTS.md:12:| R6 | Version monotonicity | ResourceRegistry::consume_and_derive | WitnessBuilder |
docs/audit/SECURITY_INVARIANTS.md:7:| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
docs/audit/SECURITY_INVARIANTS.md:9:| R3 | Child requires consumed parent | ResourceRegistry::consume_and_derive | WitnessBuilder |

## Related Functions
crates/amun-accounts/src/lib.rs:135:        let snapshot = self.build_economic_snapshot_with_ledger(ledger);
crates/amun-accounts/src/lib.rs:80:    pub fn build_economic_snapshot(&self) -> EconomicSnapshot {
crates/amun-accounts/src/lib.rs:82:        self.build_economic_snapshot_with_ledger(&ledger)
crates/amun-accounts/src/lib.rs:86:    pub fn build_economic_snapshot_with_ledger(&self, ledger: &EconomicLedger) -> EconomicSnapshot {
crates/amun-amendments/src/proposal.rs:140:        amendment.amendment_id = amendment.compute_id();
crates/amun-amendments/src/proposal.rs:144:    fn compute_id(&self) -> [u8; 32] {
crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-authority-registry/src/executor.rs:130:    fn n107_7c_execute_add_authority() {
crates/amun-authority-registry/src/executor.rs:143:        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
crates/amun-authority-registry/src/executor.rs:149:    fn n107_7c_execute_transition() {
crates/amun-authority-registry/src/executor.rs:164:        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
crates/amun-authority-registry/src/executor.rs:169:    fn n107_7c_execute_retirement() {
crates/amun-authority-registry/src/executor.rs:181:        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
crates/amun-authority-registry/src/executor.rs:197:        let result = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
crates/amun-authority-registry/src/executor.rs:214:        let result = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
crates/amun-authority-registry/src/executor.rs:231:        execute_governance(&proposal, &votes, 4, &mut reg, &mut journal).unwrap();
crates/amun-authority-registry/src/executor.rs:232:        let second = execute_governance(&proposal, &votes, 4, &mut reg, &mut journal);
crates/amun-authority-registry/src/executor.rs:38:pub fn execute_governance(
crates/amun-authority-registry/src/recovery.rs:135:        let (snapshot, snap_height, _snap_id) = build_base_snapshot();
crates/amun-authority-registry/src/recovery.rs:170:        let (snapshot, snap_height, _snap_id) = build_base_snapshot();
crates/amun-authority-registry/src/recovery.rs:211:        let (snapshot, snap_height, _snap_id) = build_base_snapshot();
crates/amun-authority-registry/src/recovery.rs:43:    fn build_base_snapshot() -> (Vec<u8>, u64, [u8; 32]) {
crates/amun-authority-registry/src/recovery.rs:57:        let (snapshot, snap_height, _snap_id) = build_base_snapshot();
crates/amun-authority-registry/src/recovery.rs:90:        let (snapshot, snap_height, snap_id) = build_base_snapshot();
crates/amun-authority-registry/src/registry.rs:116:    pub fn verify_certificate_at(&self, cert: &ValidatorCertificate, height: u64) -> bool {
crates/amun-authority-registry/src/registry.rs:148:    pub fn can_issue_at(&self, authority_version: u64, height: u64) -> bool {
crates/amun-authority-registry/src/registry.rs:322:        let cert = ValidatorCertificate::issue_v2(
crates/amun-authority-registry/src/registry.rs:332:            reg.verify_certificate_at(&cert, 550),
crates/amun-authority-registry/src/registry.rs:362:        let cert = ValidatorCertificate::issue_v2(
crates/amun-authority-registry/src/registry.rs:372:            !reg.verify_certificate_at(&cert, 650),
crates/amun-authority-registry/src/registry.rs:401:        let cert = ValidatorCertificate::issue_v2(
crates/amun-authority-registry/src/registry.rs:411:            reg.verify_certificate_at(&cert, 650),
crates/amun-authority-registry/src/registry.rs:430:        let cert = ValidatorCertificate::issue_v2(
crates/amun-authority-registry/src/registry.rs:440:            !reg.verify_certificate_at(&cert, 600),
crates/amun-authority-registry/src/registry.rs:481:    fn n107_6_old_authority_cannot_issue_after_activation() {
crates/amun-authority-registry/src/registry.rs:493:        assert!(!reg.can_issue_at(1, 150));
crates/amun-authority-registry/src/registry.rs:494:        assert!(reg.can_issue_at(2, 150));
crates/amun-authority-registry/src/transaction.rs:65:                    crate::executor::execute_governance(
crates/amun-bench/tests/n161_state_root_bench.rs:41:        let _root = reg.compute_state_root();
crates/amun-bench/tests/n162_snapshot_bench.rs:9:fn n162_bench_snapshot_build_and_restore() {
crates/amun-benchmarks/benches/sync_bench.rs:47:            black_box(imported.compute_state_root());
crates/amun-benchmarks/benches/sync_bench.rs:5:fn create_test_registry(size: u64) -> ResourceRegistry {
crates/amun-block-builder/src/lib.rs:107:        self.build_block_with_certificates(
crates/amun-block-builder/src/lib.rs:119:    pub fn build_block_with_certificates(
crates/amun-block-builder/src/lib.rs:130:        let receipts = self.engine.execute_block(&transactions);
crates/amun-block-builder/src/lib.rs:195:    fn n27_build_block_with_transactions() {
crates/amun-block-builder/src/lib.rs:216:        let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/src/lib.rs:238:        let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/src/lib.rs:254:        let block1 = b1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/src/lib.rs:255:        let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/src/lib.rs:265:        let block = builder.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/src/lib.rs:29:    pub fn verify_slashing_certificates(&self) -> Result<(), String> {
crates/amun-block-builder/src/lib.rs:41:            let recomputed = cert.compute_hash();
crates/amun-block-builder/src/lib.rs:52:    pub fn verify_slashing_root(&self, expected_root: &[u8; 32]) -> Result<(), String> {
crates/amun-block-builder/src/lib.rs:98:    pub fn build_block(
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:105:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:106:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:118:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:119:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:132:    let block = build_test_block(vec![]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:134:        block.verify_slashing_certificates().is_ok(),
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:29:fn build_test_block(certs: Vec<SlashingCertificate>) -> Block {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:32:    builder.build_block_with_certificates(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000, certs)
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:42:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:44:        block.verify_slashing_certificates().is_ok(),
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:55:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:56:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:73:    let block = build_test_block(certs);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:74:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:38:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:70:    let block1 = builder1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:71:    let block2 = builder2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:100:        block.verify_slashing_root(&[0u8; 32]).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:104:        block.verify_slashing_root(&[0x01; 32]).is_err(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:15:    let block1 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:16:    let mut block2 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:29:    let block1 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:30:    let block2 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:42:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:54:    let block_a = build_block_with_root(root_a);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:55:    let block_b = build_block_with_root(root_b);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:5:fn build_block_with_root(root: [u8; 32]) -> Block {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:65:    let block_zero = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:66:    let block_nonzero = build_block_with_root([0xAB; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:78:    let block = build_block_with_root(root);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:80:        block.verify_slashing_root(&root).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:87:    let block = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:88:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:8:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:98:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:119:    let block1 = b1.build_block(1, [0u8; 32], &mut mp1, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:121:    let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:42:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:71:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:110:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:30:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block-builder/tests/n28_first_economic_block.rs:81:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:100:    let block1 = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:123:    let block2 = builder.build_block(
crates/amun-block-builder/tests/n32_certified_block.rs:173:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:52:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block/src/body.rs:43:    pub(crate) fn compute_encoded_size(&self) -> Result<usize, FailureContext> {
crates/amun-block/src/tests.rs:85:    assert_eq!(blk.compute_id(), blk.compute_id());
crates/amun-bls/src/verify.rs:15:pub fn verify_aggregate(_message: &[u8], signature: &[u8; BLS_SIGNATURE_SIZE], public_keys: &[PublicKey]) -> AmunResult<bool> {
crates/amun-bytecode/src/lib.rs:31:    fn w6_execute_simple_program() {
crates/amun-bytecode/src/lib.rs:56:    fn w6_execute_out_of_gas() {
crates/amun-bytecode/src/program.rs:29:        program.program_hash = program.compute_hash();
crates/amun-bytecode/src/program.rs:33:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-bytecode/src/program.rs:50:        self.program_hash == self.compute_hash()
crates/amun-byzantine-tests/tests/attack_suite.rs:276:    assert!(proof.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:279:    assert!(!tampered.verify_integrity());
crates/amun-canonical-collections/src/lib.rs:20:    fn verify_root(&self, expected: &[u8; 32]) -> bool { self.canonical_root() == *expected }
crates/amun-canonical/src/float_ban.rs:18:    pub fn verify_type<T: 'static>() -> Result<(), &'static str> {
crates/amun-canonical/src/float_ban.rs:4:    pub fn verify_no_floats(code: &str) -> Result<(), Vec<&'static str>> {
crates/amun-certificate-network/src/distribution.rs:102:    pub fn build_from_runtime(
crates/amun-certificate-network/src/distribution.rs:111:            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash)?;
crates/amun-certificate-network/src/distribution.rs:136:            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-certificate-network/src/distribution.rs:249:        let bundle = BundleBuilder::build_from_runtime(block, &rt, [0u8; 32]).unwrap();
crates/amun-certificate-network/src/distribution.rs:67:        amun_constitutional_block::verify_light_client_proof(
crates/amun-certificate-network/src/gossip.rs:173:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/src/bootstrap.rs:102:        let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/bootstrap.rs:19:        verify_checkpoint_sequence(bundles, &self.trusted_checkpoint_root)
crates/amun-chain-checkpoint/src/bootstrap.rs:1:use crate::inclusion::{verify_checkpoint_sequence, CheckpointBundle};
crates/amun-chain-checkpoint/src/bootstrap.rs:22:    pub fn verify_complete_chain(&self, expected_state_root: &str) -> Result<(), String> {
crates/amun-chain-checkpoint/src/bootstrap.rs:23:        self.node.verify_chain()?;
crates/amun-chain-checkpoint/src/bootstrap.rs:43:    use crate::inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion};
crates/amun-chain-checkpoint/src/bootstrap.rs:56:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/src/bootstrap.rs:91:        let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/chain.rs:1:use crate::inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle};
crates/amun-chain-checkpoint/src/chain.rs:37:            let proof = prove_checkpoint_inclusion(checkpoints, &c.checkpoint_hash_bytes())?;
crates/amun-chain-checkpoint/src/chain.rs:61:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:115:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:165:        let proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:174:        let mut proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:182:        let mut proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:190:        let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:194:    fn n12d_light_verify_sequence() {
crates/amun-chain-checkpoint/src/inclusion.rs:199:        let p1 = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:200:        let p2 = prove_checkpoint_inclusion(&certs, &c2.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:202:        assert!(verify_checkpoint_sequence(&bundles, &root).is_ok());
crates/amun-chain-checkpoint/src/inclusion.rs:205:    fn n12d_light_verify_wrong_root() {
crates/amun-chain-checkpoint/src/inclusion.rs:208:        let p1 = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/src/inclusion.rs:209:        assert!(verify_checkpoint_sequence(&[CheckpointBundle::new(c1, p1)], &[0x11; 32]).is_err());
crates/amun-chain-checkpoint/src/inclusion.rs:24:pub fn prove_checkpoint_inclusion(
crates/amun-chain-checkpoint/src/inclusion.rs:82:pub fn verify_checkpoint_sequence(
crates/amun-chain-checkpoint/src/lib.rs:129:        cert.checkpoint_hash = cert.compute_hash();
crates/amun-chain-checkpoint/src/lib.rs:138:    pub fn compute_hash(&self) -> String {
crates/amun-chain-checkpoint/src/lib.rs:158:        let recomputed = self.compute_hash();
crates/amun-chain-checkpoint/src/lib.rs:245:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:100:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:127:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:55:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:56:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:5:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:103:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:105:    let (_, mut bundle2, _) = build_checkpoint_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:10:fn build_checkpoint_bundle(
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:121:    let (cp, bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:124:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:51:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:57:fn build_bundle(start: u64, end: u64) -> (CheckpointBundle, [u8; 32]) {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:58:    let (_, bundle, root) = build_checkpoint_bundle(start, end);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:64:    let (cp1, _, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:65:    let (bundle2, _) = build_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:68:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:77:    let (_, mut bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:86:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:87:    let (bundle2, _) = build_bundle(4, 6);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:95:    let (bundle, _) = build_bundle(0, 4);
crates/amun-chain-context/src/lib.rs:44:    pub fn verify_binding(&self) -> bool {
crates/amun-chain-store/examples/snapshot_verification_test.rs:2:use amun_chain_store::snapshot::{create_snapshot, verify_snapshot};
crates/amun-chain-store/examples/snapshot_verification_test.rs:38:    let manifest = verify_snapshot(&snapshot_dir).unwrap();
crates/amun-chain-store/examples/snapshot_verification_test.rs:46:    match verify_snapshot(&snapshot_dir) {
crates/amun-chain-store/examples/snapshot_verification_test.rs:61:    match verify_snapshot(&snapshot_dir) {
crates/amun-chain-store/src/snapshot.rs:124:    let manifest = verify_snapshot(snapshot_dir)?;
crates/amun-chain-store/src/snapshot.rs:70:pub fn verify_snapshot(snapshot_dir: &Path) -> Result<SnapshotManifest, String> {
crates/amun-chunked-snapshot/src/lib.rs:7:pub use verifier::verify_chunk;
crates/amun-chunked-snapshot/src/verifier.rs:5:pub fn verify_chunk(
crates/amun-consensus-execution/src/fork_choice.rs:160:    pub fn verify_lock_monotonicity(&self) -> bool {
crates/amun-consensus-execution/src/persistent_state.rs:112:        let auth = wal.validate_authority_chain()?;
crates/amun-consensus-execution/src/persistent_state.rs:128:    pub fn validate_snapshot_binding(&self, snapshot: &SnapshotCheckpoint) -> Result<(), String> {
crates/amun-consensus-execution/src/persistent_state.rs:149:    pub fn validate_authority(&self) -> Result<AuthorityValidation, String> {
crates/amun-consensus-execution/src/persistent_state.rs:150:        self.wal.validate_authority_chain()
crates/amun-consensus-integration/src/consensus_integrator.rs:113:    fn w17_execute_block_with_proofs() {
crates/amun-consensus-integration/src/consensus_integrator.rs:122:            pre_state_root: registry.compute_state_root(),
crates/amun-consensus-integration/src/consensus_integrator.rs:125:        let block = ConsensusIntegrator::execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:134:        assert!(block.verify_all_proofs());
crates/amun-consensus-integration/src/consensus_integrator.rs:148:            pre_state_root: registry.compute_state_root(),
crates/amun-consensus-integration/src/consensus_integrator.rs:151:        let block = ConsensusIntegrator::execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:15:    pub fn execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:175:            pre_state_root: registry.compute_state_root(),
crates/amun-consensus-integration/src/consensus_integrator.rs:178:        let block = ConsensusIntegrator::execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:200:            pre_state_root: registry.compute_state_root(),
crates/amun-consensus-integration/src/consensus_integrator.rs:203:        let block = ConsensusIntegrator::execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:227:            pre_state_root: registry.compute_state_root(),
crates/amun-consensus-integration/src/consensus_integrator.rs:230:        let block = ConsensusIntegrator::execute_block(
crates/amun-consensus-integration/src/consensus_integrator.rs:238:        assert_eq!(block.state_root, registry.compute_state_root());
crates/amun-consensus-integration/src/consensus_integrator.rs:239:        assert_eq!(block.proof_root, block.compute_proof_root());
crates/amun-consensus-integration/src/consensus_integrator.rs:50:        let state_root = registry.compute_state_root();
crates/amun-consensus-integration/src/consensus_integrator.rs:61:        block.proof_root = block.compute_proof_root();
crates/amun-consensus-integration/src/consensus_integrator.rs:83:        if !block.verify_all_proofs() {
crates/amun-consensus-integration/src/consensus_types.rs:101:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-consensus-integration/src/consensus_types.rs:116:        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
crates/amun-consensus-integration/src/consensus_types.rs:18:    pub fn compute_proof_root(&self) -> [u8; 32] {
crates/amun-consensus-integration/src/consensus_types.rs:30:    pub fn verify_all_proofs(&self) -> bool {
crates/amun-consensus-integration/src/consensus_types.rs:35:            if !proof.verify_integrity() {
crates/amun-consensus-integration/src/consensus_types.rs:39:        self.proof_root == self.compute_proof_root()
crates/amun-consensus-integration/src/consensus_types.rs:97:        cert.certificate_hash = cert.compute_hash();
crates/amun-consensus-network/src/certificate_evidence_validation.rs:113:        let result = validate_certificate_evidence(&cert, &store);
crates/amun-consensus-network/src/certificate_evidence_validation.rs:139:        let result = validate_certificate_evidence(&cert, &store);
crates/amun-consensus-network/src/certificate_evidence_validation.rs:153:        let result = validate_certificate_evidence(&cert, &store);
crates/amun-consensus-network/src/certificate_evidence_validation.rs:165:    fn n111_6_build_missing_evidence_request() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:166:        let req = build_missing_evidence_request([0xAA; 32], vec![[0xB1; 32], [0xB2; 32]]);
crates/amun-consensus-network/src/certificate_evidence_validation.rs:31:pub fn validate_certificate_evidence(
crates/amun-consensus-network/src/certificate_evidence_validation.rs:53:pub fn build_missing_evidence_request(
crates/amun-consensus-network/src/engine.rs:164:        if let Err(e) = qc.verify_strict(validator_powers) {
crates/amun-consensus-network/src/engine.rs:351:            if !amun_validator_identity::verify_ed25519(pk, &payload, &vote.signature) {
crates/amun-consensus-network/src/engine.rs:358:                crate::vote_binding::verify_vote_binding(&vote)?;
crates/amun-consensus-network/src/engine.rs:425:                let evidence_id = crate::evidence_store::EvidenceRecord::compute_evidence_id(
crates/amun-consensus-network/src/engine.rs:507:        if let Err(e) = ConsensusEngine::verify_qc_signatures(&validator_keys, qc) {
crates/amun-consensus-network/src/engine.rs:550:    fn verify_qc_signatures(
crates/amun-consensus-network/src/engine.rs:574:            if !amun_validator_identity::verify_ed25519(pk, &payload, &vote.signature) {
crates/amun-consensus-network/src/evidence_gossip.rs:178:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_ok());
crates/amun-consensus-network/src/evidence_gossip.rs:190:        let err = EvidenceGossip::verify_announcement(&ann, 50).unwrap_err();
crates/amun-consensus-network/src/evidence_gossip.rs:203:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/evidence_gossip.rs:215:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/evidence_gossip.rs:79:    pub fn verify_announcement(
crates/amun-consensus-network/src/evidence_store.rs:49:    pub fn compute_evidence_id(
crates/amun-consensus-network/src/evidence_store.rs:81:            Self::compute_evidence_id(&validator_id, height, &evidence_type, &payload);
crates/amun-consensus-network/src/execution_commitment.rs:109:        let recomputed = Self::compute_execution_root(
crates/amun-consensus-network/src/execution_commitment.rs:142:    fn n109_8_compute_execution_root_is_deterministic() {
crates/amun-consensus-network/src/execution_commitment.rs:148:        let r1 = ExecutionCommitment::compute_execution_root(&pk, h, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:149:        let r2 = ExecutionCommitment::compute_execution_root(&pk, h, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:159:        let r1 = ExecutionCommitment::compute_execution_root(&[1u8; 32], h, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:160:        let r2 = ExecutionCommitment::compute_execution_root(&[2u8; 32], h, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:170:        let r1 = ExecutionCommitment::compute_execution_root(&pk, 1, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:171:        let r2 = ExecutionCommitment::compute_execution_root(&pk, 2, &bh, &sr);
crates/amun-consensus-network/src/execution_commitment.rs:60:    pub fn compute_execution_root(
crates/amun-consensus-network/src/execution_commitment.rs:84:            Self::compute_execution_root(&validator_id, height, &block_hash, &state_root);
crates/amun-consensus-network/src/finality_gate.rs:19:pub fn execute_if_finalized<F, T>(
crates/amun-consensus-network/src/finality_gate.rs:72:        let result = execute_if_finalized(&cert, 100, || Ok("executed"));
crates/amun-consensus-network/src/finality_gate.rs:81:        let result = execute_if_finalized(&cert, 99, || Ok("executed"));
crates/amun-consensus-network/src/integrated_slashing.rs:65:    /// This is the main entry point. Every time verify_vote_binding fails,
crates/amun-consensus-network/src/lib.rs:41:    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
crates/amun-consensus-network/src/lib.rs:49:pub use finality_gate::{execute_if_finalized, is_certificate_finalized};
crates/amun-consensus-network/src/lib.rs:60:pub use re_executor::verify_block_execution;
crates/amun-consensus-network/src/lib.rs:65:pub use slashing_inclusion_proof::{build_inclusion_proof, SlashingInclusionProof};
crates/amun-consensus-network/src/lib.rs:70:pub use vote_binding::verify_vote_binding;
crates/amun-consensus-network/src/messages.rs:102:        if !self.verify_consistency() {
crates/amun-consensus-network/src/messages.rs:210:        assert!(proof.verify_standalone().is_ok());
crates/amun-consensus-network/src/messages.rs:231:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:257:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:270:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:283:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:296:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:311:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:324:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:362:        assert!(!qc.verify_quorum());
crates/amun-consensus-network/src/messages.rs:380:        assert!(!qc.verify_consistency());
crates/amun-consensus-network/src/messages.rs:45:    pub fn verify_standalone(&self) -> Result<(), String> {
crates/amun-consensus-network/src/messages.rs:81:    pub fn verify_quorum(&self) -> bool {
crates/amun-consensus-network/src/messages.rs:85:    pub fn verify_consistency(&self) -> bool {
crates/amun-consensus-network/src/messages.rs:92:        self.verify_quorum() && self.verify_consistency()
crates/amun-consensus-network/src/messages.rs:94:    pub fn verify_strict(&self, validator_powers: &HashMap<[u8; 32], u64>) -> Result<(), String> {
crates/amun-consensus-network/src/misbehavior.rs:70:        proof.verify_standalone()?;
crates/amun-consensus-network/src/misbehavior_registry.rs:222:    pub fn rebuild_from_evidence(
crates/amun-consensus-network/src/misbehavior_registry.rs:380:            MisbehaviorRegistry::rebuild_from_evidence(MisbehaviorThresholds::default(), &store);
crates/amun-consensus-network/src/multi_signer_certificate.rs:158:    fn n116_verify_approvals_counts_correctly() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:165:        let valid = ms.verify_approvals().unwrap();
crates/amun-consensus-network/src/multi_signer_certificate.rs:176:        let valid = ms.verify_approvals().unwrap();
crates/amun-consensus-network/src/multi_signer_certificate.rs:47:    pub fn verify_approvals(&self) -> Result<usize, String> {
crates/amun-consensus-network/src/multi_signer_certificate.rs:70:        let valid = self.verify_approvals()?;
crates/amun-consensus-network/src/re_executor.rs:7:pub fn verify_block_execution<F>(
crates/amun-consensus-network/src/slashing_certificate.rs:112:    pub fn verify_signature(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_certificate.rs:136:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_certificate.rs:199:        cert.certificate_hash = cert.compute_hash();
crates/amun-consensus-network/src/slashing_certificate.rs:205:        let recomputed = self.compute_hash();
crates/amun-consensus-network/src/slashing_certificate.rs:217:            self.verify_signature()?;
crates/amun-consensus-network/src/slashing_certificate.rs:284:        assert_eq!(cert1.compute_hash(), cert1.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:285:        assert_eq!(cert2.compute_hash(), cert2.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:318:    fn n110_2_verify_rejects_tampered_amount() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:53:        proof.proof_id = proof.compute_proof_id();
crates/amun-consensus-network/src/slashing_fraud_proof.rs:58:    fn compute_proof_id(&self) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:72:        let recomputed_id = self.compute_proof_id();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:179:        let proof = build_inclusion_proof(&slashes, 0).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:193:        let proof = build_inclusion_proof(&slashes, 2).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:204:        let mut proof = build_inclusion_proof(&slashes, 1).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:218:        assert!(build_inclusion_proof(&slashes, 5).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:223:        assert!(build_inclusion_proof(&[], 0).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:232:            let proof = build_inclusion_proof(&slashes, i).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:94:pub fn build_inclusion_proof(
crates/amun-consensus-network/src/slashing_ledger.rs:63:    pub fn execute<F, T>(&mut self, cert: &SlashingCertificate, execute_fn: F) -> Result<T, String>
crates/amun-consensus-network/src/slashing_ledger.rs:78:        let result = execute_fn()?;
crates/amun-consensus-network/src/slashing_state.rs:117:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:150:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:30:        execute_fn: F,
crates/amun-consensus-network/src/slashing_state.rs:35:        let result = self.ledger.execute(cert, execute_fn)?;
crates/amun-consensus-network/src/slashing_state.rs:42:    pub fn verify_consistency(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_state.rs:98:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/staking_adapter.rs:102:        cert.verify_signature()
crates/amun-consensus-network/src/staking_adapter.rs:107:            Some(ValidatorAction::Slash) => Ok(Some(self.execute_slash(validator_id))),
crates/amun-consensus-network/src/staking_adapter.rs:113:    fn execute_slash(&mut self, validator_id: &[u8; 32]) -> SlashResult {
crates/amun-consensus-network/src/staking_adapter.rs:545:        let r = adapter.execute_after_finality(&vid, &cert, 99);
crates/amun-consensus-network/src/staking_adapter.rs:586:        let r = adapter.execute_after_finality(&vid, &cert, 100);
crates/amun-consensus-network/src/staking_adapter.rs:60:            Some(ValidatorAction::Slash) => Some(self.execute_slash(validator_id)),
crates/amun-consensus-network/src/staking_adapter.rs:67:    pub fn execute_after_finality(
crates/amun-consensus-network/src/staking_adapter.rs:80:            .verify_signature()
crates/amun-consensus-network/src/staking_adapter.rs:86:            Some(ValidatorAction::Slash) => Ok(Some(self.execute_slash(validator_id))),
crates/amun-consensus-network/src/validation.rs:5:    pub fn validate_basic(
crates/amun-consensus-network/src/validator_identity.rs:101:        if !identity.verify_binding() {
crates/amun-consensus-network/src/validator_identity.rs:169:        assert!(id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:176:        assert!(!id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:212:    fn n113_1_binding_must_verify_before_registration() {
crates/amun-consensus-network/src/validator_identity.rs:274:        assert!(decoded.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:48:        let binding = Self::compute_binding(&validator_id, public_key);
crates/amun-consensus-network/src/validator_identity.rs:59:    pub fn compute_binding(validator_id: &[u8; 32], public_key: &PublicKey) -> [u8; 32] {
crates/amun-consensus-network/src/validator_identity.rs:68:    pub fn verify_binding(&self) -> bool {
crates/amun-consensus-network/src/validator_identity.rs:69:        let recomputed = Self::compute_binding(&self.validator_id, &self.to_public_key());
crates/amun-consensus-network/src/validator_identity.rs:76:        let binding = Self::compute_binding(&validator_id, &pk);
crates/amun-consensus-network/src/vote_binding.rs:135:        assert!(verify_vote_binding(&vote).is_ok());
crates/amun-consensus-network/src/vote_binding.rs:143:        let err = verify_vote_binding(&vote).unwrap_err();
crates/amun-consensus-network/src/vote_binding.rs:152:        let err = verify_vote_binding(&vote).unwrap_err();
crates/amun-consensus-network/src/vote_binding.rs:161:        let err = verify_vote_binding(&vote).unwrap_err();
crates/amun-consensus-network/src/vote_binding.rs:170:        let err = verify_vote_binding(&vote).unwrap_err();
crates/amun-consensus-network/src/vote_binding.rs:183:        let err = verify_vote_binding(&vote).unwrap_err();
crates/amun-consensus-network/src/vote_binding.rs:18:pub fn verify_vote_binding(vote: &ConsensusVote) -> Result<(), String> {
crates/amun-consensus-network/src/vote_binding.rs:200:            verify_vote_binding(&vote).is_ok(),
crates/amun-consensus-network/src/vote_binding.rs:217:            verify_vote_binding(&v1).is_ok(),
crates/amun-consensus-network/src/vote_binding.rs:221:            verify_vote_binding(&v2).is_ok(),
crates/amun-consensus-network/src/vote_binding.rs:225:            verify_vote_binding(&v3).is_ok(),
crates/amun-consensus-network/src/vote_binding.rs:47:    let recomputed = ExecutionCommitment::compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:162:    let recomputed = compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:185:        compute_execution_root(&pk, commit.height, &commit.block_hash, &commit.state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:258:    let recomputed = compute_execution_root(&voter, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:52:fn compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:74:    let execution_root = compute_execution_root(&validator_id, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_block_propagation.rs:257:fn n109_validate_basic_accepts_valid_proposal() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:260:    let result = validate_basic_testable(&p, 0, &parent, 1050);
crates/amun-consensus-network/tests/n109_block_propagation.rs:272:fn n109_validate_basic_rejects_wrong_height() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:275:    let result = validate_basic_testable(&p, 0, &parent, 5000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:287:fn n109_validate_basic_rejects_parent_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:289:    let result = validate_basic_testable(&p, 0, &[0xBB; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:304:fn n109_validate_basic_rejects_hash_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:307:    let result = validate_basic_testable(&p, 0, &[0u8; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:73:fn validate_basic_testable(
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:108:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 42));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:137:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 99));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:181:        let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, seed));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:248:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:277:    let result = verify_block_execution(&proposal, |_bytes| {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:317:    let result = verify_block_execution(cached, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:51:fn verify_block_execution<F>(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:107:    let result_b = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:121:    let request = build_missing_evidence_request([0xBB; 32], missing_ids);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:12:    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:153:    let result_b_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:192:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:203:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:93:    let result_a = validate_certificate_evidence(&cert, &store_a);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:103:    let result_before = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:11:    process_incoming_evidence_push, validate_certificate_evidence, EvidenceCount, EvidenceGossip,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:130:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:205:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:119:    assert!(cert1.verify_signature().is_ok());
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:120:    assert!(cert2.verify_signature().is_ok());
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:138:        decoded.verify_signature().is_ok(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:42:        cert.verify_signature().is_ok(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:59:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:77:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:95:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:100:    assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:131:        state.verify_consistency().is_err(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:46:        state.verify_consistency().is_ok(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:82:    assert!(restored.verify_consistency().is_ok());
crates/amun-consensus-reactor/src/reactor.rs:72:        let _batch = self.scheduler.execute_batch(10);
crates/amun-consensus/src/action.rs:26:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-consensus/src/action.rs:70:        record.hash = record.compute_hash();
crates/amun-consensus/src/action.rs:91:            if self.records[i].hash != self.records[i].compute_hash() {
crates/amun-consensus/src/validator.rs:16:    /// Must be rebuilt after deserialization via rebuild_index().
crates/amun-consensus/src/validator.rs:41:    pub fn rebuild_index(&mut self) {
crates/amun-constitution-builder/src/hashing.rs:17:pub fn compute_specification_hash(root: &Path) -> Result<String, String> {
crates/amun-constitution-builder/src/hashing.rs:18:    let hashes = compute_all_hashes(root)?;
crates/amun-constitution-builder/src/hashing.rs:23:pub fn compute_all_hashes(root: &Path) -> Result<SpecificationHashes, String> {
crates/amun-constitution-builder/src/hashing.rs:27:    let semantic_hash = compute_hash_for_extensions(root, &semantic_exts)?;
crates/amun-constitution-builder/src/hashing.rs:28:    let documentary_hash = compute_hash_for_extensions(root, &documentary_exts)?;
crates/amun-constitution-builder/src/hashing.rs:32:    let combined_hash = compute_hash_for_extensions(root, &all_exts)?;
crates/amun-constitution-builder/src/hashing.rs:41:fn compute_hash_for_extensions(root: &Path, extensions: &[&str]) -> Result<String, String> {
crates/amun-constitution-builder/src/lib.rs:22:pub use hashing::{compute_all_hashes, compute_specification_hash, SpecificationHashes};
crates/amun-constitution-builder/src/verify.rs:11:    pub fn verify_replay<T: CanonicalSerialize + PartialEq>(
crates/amun-constitution-builder/tests/determinism_tests.rs:12:    VerificationEngine::verify_replay(&m1, &m2).expect("Manifests must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:40:    VerificationEngine::verify_replay(&f1, &f2).expect("Federation artifacts must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:58:    VerificationEngine::verify_replay(&t1, &t2).expect("Treaties must be identical");
crates/amun-constitution-core/src/ordering.rs:16:pub fn validate_ordering(ids: &[u64]) -> Result<(), &'static str> {
crates/amun-constitution/src/canonical_form.rs:24:    pub fn compute_hash(&self) -> Hash<ConstitutionDomain> {
crates/amun-constitution/src/capacity.rs:25:    pub fn verify_compatible(&self, other: &Self) -> Result<(), &'static str> {
crates/amun-constitution/src/execution_semantics.rs:51:    fn execute_transaction(&self, ctx: &ExecutionContext, tx: &Transaction) -> ExecutionResult;
crates/amun-constitution/src/execution_semantics.rs:53:    fn execute_block(
crates/amun-constitution/src/freeze_validator.rs:28:    pub fn validate_change(
crates/amun-constitution/src/operational_semantics.rs:25:pub fn execute_semantics(_initial: Configuration) -> Configuration {
crates/amun-constitution/src/quorum_transition.rs:19:    pub fn verify_safety(&self) -> Result<(), &'static str> {
crates/amun-constitution/src/stake_quorum.rs:51:    pub fn verify_weighted_transition(&self, new_set: &Self) -> Result<(), &'static str> {
crates/amun-constitution/src/tests.rs:10:    assert!(c1.verify_compatible(&c2).is_ok());
crates/amun-constitution/src/tests.rs:20:    assert!(params.verify_safety().is_ok());
crates/amun-constitution/src/tests.rs:30:    assert!(params.verify_safety().is_err());
crates/amun-constitutional-authority-semantics/src/capability.rs:42:        let id = tmp.compute_id();
crates/amun-constitutional-authority-semantics/src/capability.rs:52:    fn compute_id(&self) -> String {
crates/amun-constitutional-authority/src/certificate.rs:114:    fn compute_id(&self) -> String {
crates/amun-constitutional-authority/src/certificate.rs:61:        let id = tmp.compute_id();
crates/amun-constitutional-authority/src/certificate.rs:87:        let id = tmp.compute_id();
crates/amun-constitutional-authority/src/rotation.rs:17:    pub fn validate_rotation(
crates/amun-constitutional-authority/tests/authority_tests.rs:112:    assert!(KeyRotationLaw::validate_rotation(&old.verifying_key_hex(), &signed, &old).is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:118:    let root = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:17:fn build_child(
crates/amun-constitutional-authority/tests/authority_tests.rs:35:    let a = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:36:    let b = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:45:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:46:    let child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:60:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:61:    let child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:72:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:73:    let mut child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:7:fn build_root(key: &ConstitutionalKeyPair) -> ConstitutionalCertificate {
crates/amun-constitutional-authority/tests/authority_tests.rs:86:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:87:    let child = build_child(&root, &child_key);
crates/amun-constitutional-block/src/block.rs:49:        let hash = block.compute_hash();
crates/amun-constitutional-block/src/block.rs:54:    pub fn compute_hash(&self) -> String {
crates/amun-constitutional-block/src/chain.rs:47:            let recomputed = block.compute_hash();
crates/amun-constitutional-block/src/chain.rs:61:    pub fn verify_block_evidence(
crates/amun-constitutional-block/src/chain.rs:81:    pub fn verify_chain_evidence(
crates/amun-constitutional-block/src/chain.rs:93:            self.verify_block_evidence(i as u64, log)?;
crates/amun-constitutional-block/src/lib.rs:116:pub fn verify_light_client_proof(
crates/amun-constitutional-block/src/lib.rs:48:pub fn verify_block_provenance(
crates/amun-constitutional-block/src/lib.rs:84:///   - verify_block_provenance() for block↔certificate binding
crates/amun-constitutional-block/src/lib.rs:86:pub fn verify_full_replay(
crates/amun-constitutional-block/src/lib.rs:92:    verify_block_provenance(block, cert)?;
crates/amun-constitutional-block/tests/block_tests.rs:168:fn test_verify_block_provenance_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:169:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:192:    assert!(verify_block_provenance(&block, &cert).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:196:fn test_verify_block_provenance_tampered_state_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:197:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:220:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:224:fn test_verify_block_provenance_wrong_certificate_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:225:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:248:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:253:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:277:    assert!(verify_full_replay(&block, &cert, rt.journal()).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:282:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:309:    assert!(verify_full_replay(&block, &cert, &tampered).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:314:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:339:    assert!(verify_full_replay(&block, &cert, &rt.journal()[..1]).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:344:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:354:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:369:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:374:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:384:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:402:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:407:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:422:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash1).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:438:    assert!(verify_light_client_proof(&block, &cert2, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:443:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:452:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:468:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/replay_tests.rs:22:    chain.verify_block_evidence(0, &log).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:43:    assert!(chain.verify_block_evidence(0, &tampered).is_err());
crates/amun-constitutional-block/tests/replay_tests.rs:64:    chain.verify_chain_evidence(&logs).unwrap();
crates/amun-constitutional-commitment/src/apphash.rs:8:    pub fn compute_state_root(
crates/amun-constitutional-commitment/src/economic_tree.rs:24:fn build_merkle_root(mut hashes: Vec<Hash32>) -> Hash32 {
crates/amun-constitutional-commitment/src/economic_tree.rs:69:        Ok(build_merkle_root(leaves))
crates/amun-constitutional-commitment/src/endblock.rs:17:        let constitutional_root = compute_constitutional_root(
crates/amun-constitutional-commitment/src/endblock.rs:4:use crate::roots::compute_constitutional_root;
crates/amun-constitutional-commitment/src/lib.rs:20:pub use roots::{commitment_root, compute_constitutional_root};
crates/amun-constitutional-commitment/src/roots.rs:8:pub fn compute_constitutional_root(
crates/amun-constitutional-commitment/src/roots_bundle.rs:26:            compute_constitutional_root(identity_root, [0u8; 32], governance_root, economic_root);
crates/amun-constitutional-commitment/src/roots_bundle.rs:2:use crate::roots::{commitment_root, compute_constitutional_root};
crates/amun-constitutional-commitment/src/verify.rs:26:        let recomputed_constitutional = compute_constitutional_root(
crates/amun-constitutional-commitment/src/verify.rs:2:use crate::roots::{commitment_root, compute_constitutional_root};
crates/amun-constitutional-commitment/tests/determinism.rs:16:    let root_a = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:17:    let root_b = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:28:    let root_a = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:2:    commitment_root, compute_constitutional_root, ConstitutionalCommitment, Hash32,
crates/amun-constitutional-commitment/tests/determinism.rs:32:    let root_b = compute_constitutional_root(id2, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:42:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:71:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/economic_tree.rs:2:    compute_constitutional_root, EconomicError, EconomicSnapshot, EconomicTree, Hash32,
crates/amun-constitutional-commitment/tests/economic_tree.rs:65:    let const_root_a = compute_constitutional_root(
crates/amun-constitutional-commitment/tests/economic_tree.rs:76:    let const_root_b = compute_constitutional_root(
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:107:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:108:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:54:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:55:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:82:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:83:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/rpc.rs:34:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitment/tests/rpc.rs:69:    let sr_a = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_a);
crates/amun-constitutional-commitment/tests/rpc.rs:70:    let sr_b = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_b);
crates/amun-constitutional-commitment/tests/rpc.rs:95:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitment/tests/state.rs:24:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:2:    commitment_root, compute_constitutional_root, ConstitutionalCommitment, ConstitutionalState,
crates/amun-constitutional-commitment/tests/verify.rs:110:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitments/src/smt.rs:146:    fn prove_inner(
crates/amun-constitutional-commitments/src/smt.rs:161:            self.prove_inner(depth + 1, &[], target, siblings, directions);
crates/amun-constitutional-commitments/src/smt.rs:172:            self.prove_inner(depth + 1, &keys[..split], target, siblings, directions);
crates/amun-constitutional-commitments/src/smt.rs:177:            self.prove_inner(depth + 1, &keys[split..], target, siblings, directions);
crates/amun-constitutional-commitments/src/smt.rs:73:        self.prove_inner(0, &self.sorted_keys, &kh, &mut siblings, &mut directions);
crates/amun-constitutional-commitments/tests/commitment_tests.rs:17:    let (s1, g1, e1) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:18:    let (s2, g2, e2) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:26:    let (s, g, e) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:3:fn build_domain_roots() -> ([u8; 32], [u8; 32], [u8; 32]) {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:68:    pub fn from_quorum_certificate(block_hash: &[u8; 32], qc_verify_quorum: bool) -> QcEvidence {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:71:            finality_supermajority: qc_verify_quorum,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:81:    qc_verify_quorum: bool,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:88:        qc_verify_quorum,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:93:    let qc = QcEvidenceProvider::from_quorum_certificate(block_hash, qc_verify_quorum);
crates/amun-constitutional-enforcement/src/evidence_records.rs:152:        record.evidence_hash = record.compute_hash();
crates/amun-constitutional-enforcement/src/evidence_records.rs:156:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-constitutional-enforcement/src/evidence_records.rs:169:        self.compute_hash() == self.evidence_hash
crates/amun-constitutional-enforcement/src/proof_engine.rs:113:        if !Self::verify_state_root_integrity(block_state_root, execution_state_root) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:126:        if !Self::verify_chain_continuity(block_parent, tip_hash) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:135:        if !Self::verify_signatures(transactions) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:144:        if !Self::verify_no_double_spend(sender_nonce_pairs) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:14:    pub fn verify_state_root_integrity(
crates/amun-constitutional-enforcement/src/proof_engine.rs:153:        if !Self::verify_slashing_evidence_binding(evidence_ids, evidence_available) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:162:        if !Self::verify_validator_governance(governance_approved) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:171:        if !Self::verify_replay_determinism(original_root, replay_root) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:180:        if !Self::verify_finality_supermajority(approval_power, total_power) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:192:        if !Self::verify_state_transition(pre_state_root, post_state_root, transition_valid) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:201:        if !Self::verify_evidence_validity(evidence_hash_valid) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:22:    pub fn verify_chain_continuity(block_parent: &[u8; 32], tip_hash: &[u8; 32]) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:29:    pub fn verify_signatures(transactions: &[([u8; 32], Vec<u8>, [u8; 64])]) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:31:            if !amun_validator_identity::verify_ed25519(&sender, payload, signature) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:41:    pub fn verify_no_double_spend(sender_nonce_pairs: &[([u8; 32], u64)]) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:52:    pub fn verify_slashing_evidence_binding(
crates/amun-constitutional-enforcement/src/proof_engine.rs:60:    pub fn verify_validator_governance(governance_approved: bool) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:65:    pub fn verify_replay_determinism(original_root: &[u8; 32], replay_root: &[u8; 32]) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:70:    pub fn verify_finality_supermajority(approval_power: u64, total_power: u64) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:75:    pub fn verify_state_transition(
crates/amun-constitutional-enforcement/src/proof_engine.rs:84:    pub fn verify_evidence_validity(evidence_hash_valid: bool) -> bool {
crates/amun-constitutional-enforcement/src/state_transition.rs:107:    let post_state_root = execute_fn();
crates/amun-constitutional-enforcement/src/state_transition.rs:131:        let verdict = verify_state_transition(
crates/amun-constitutional-enforcement/src/state_transition.rs:147:        let verdict = verify_state_transition(
crates/amun-constitutional-enforcement/src/state_transition.rs:167:        let verdict = verify_state_transition(
crates/amun-constitutional-enforcement/src/state_transition.rs:92:/// `execute_fn` is the actual execution function that produces post_state_root.
crates/amun-constitutional-enforcement/src/state_transition.rs:95:pub fn verify_state_transition<F, R, S>(
crates/amun-constitutional-enforcement/src/state_transition.rs:98:    execute_fn: F,
crates/amun-constitutional-geometry/src/flow_dynamics.rs:57:    pub fn compute_total_force(&mut self) {
crates/amun-constitutional-geometry/src/flow_dynamics.rs:94:        self.compute_total_force();
crates/amun-constitutional-geometry/src/metric_tensor.rs:54:            self.recompute_determinant();
crates/amun-constitutional-geometry/src/metric_tensor.rs:58:    fn recompute_determinant(&mut self) {
crates/amun-constitutional-governance/src/capability.rs:53:        let id = tmp.compute_id();
crates/amun-constitutional-governance/src/capability.rs:64:    fn compute_id(&self) -> String {
crates/amun-constitutional-governance/src/delegation.rs:10:pub fn verify_delegation_chain(
crates/amun-constitutional-governance/src/voting.rs:45:        let id = tmp.compute_id();
crates/amun-constitutional-governance/src/voting.rs:56:    fn compute_id(&self) -> String {
crates/amun-constitutional-governance/tests/governance_tests.rs:59:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_ok());
crates/amun-constitutional-governance/tests/governance_tests.rs:77:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_err());
crates/amun-constitutional-integration/src/lib.rs:101:            "forall fc in FinalityCertificate : verify_signatures(fc) = true",
crates/amun-constitutional-integration/src/lib.rs:13:    pub fn build_obligation_registry() -> ObligationRegistry {
crates/amun-constitutional-integration/src/lib.rs:346:        let registry = Self::build_obligation_registry();
crates/amun-constitutional-integration/src/lib.rs:449:    fn n47_7_build_obligation_registry() {
crates/amun-constitutional-integration/src/lib.rs:450:        let reg = ConstitutionalBridge::build_obligation_registry();
crates/amun-constitutional-kernel/src/receipt.rs:42:        let id = tmp.compute_id();
crates/amun-constitutional-kernel/src/receipt.rs:53:    fn compute_id(&self) -> String {
crates/amun-constitutional-kernel/tests/kernel_tests.rs:104:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:12:fn build_test_context() -> (
crates/amun-constitutional-kernel/tests/kernel_tests.rs:137:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:52:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:86:    let (ctx, _capabilities, _) = build_test_context();
crates/amun-constitutional-proof/src/article_i_certificate.rs:75:        let derived_ok = graph.validate_derived_terminate_in_primary(&kinds).is_ok();
crates/amun-constitutional-proof/src/constitutional_verdict.rs:72:        verdict.verdict_hash = verdict.compute_hash();
crates/amun-constitutional-proof/src/constitutional_verdict.rs:78:        self.verdict_hash == self.compute_hash()
crates/amun-constitutional-proof/src/constitutional_verdict.rs:88:    fn compute_hash(&self) -> String {
crates/amun-constitutional-proof/src/dependency_graph.rs:79:    pub fn validate_derived_terminate_in_primary(
crates/amun-constitutional-proof/src/lib.rs:1114:    fn n47_3_s1_verify_and_archive() {
crates/amun-constitutional-proof/src/lib.rs:325:        assert!(graph.validate_derived_terminate_in_primary(&kinds).is_ok());
crates/amun-constitutional-proof/src/lib.rs:340:        let result = graph.validate_derived_terminate_in_primary(&kinds);
crates/amun-constitutional-proof/src/lib.rs:469:    fn n47_1_cert_issue_success() {
crates/amun-constitutional-proof/src/lib.rs:676:    fn n47_2_s1_compute_verdict_hash() {
crates/amun-constitutional-proof/src/obligation_registry.rs:116:    fn build_graph_from_registry(&self) -> DependencyGraph {
crates/amun-constitutional-proof/src/obligation_registry.rs:50:        self.graph.validate_derived_terminate_in_primary(&kinds)?;
crates/amun-constitutional-proof/src/obligation_registry.rs:53:            self.graph = self.build_graph_from_registry();
crates/amun-constitutional-proof/src/publication_package.rs:104:    fn compute_package_hash(&self) -> String {
crates/amun-constitutional-proof/src/publication_package.rs:78:        pkg.package_hash = pkg.compute_package_hash();
crates/amun-constitutional-proof/src/publication_package.rs:89:        self.package_hash == self.compute_package_hash()
crates/amun-constitutional-proof/src/verdict_evaluator.rs:32:        let overall = Self::compute_overall_result(obligations, &results);
crates/amun-constitutional-proof/src/verdict_evaluator.rs:47:    pub fn compute_overall_result(
crates/amun-constitutional-quarantine/src/pipeline.rs:34:    pub fn verify_physics(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:47:    pub fn verify_replay(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:61:    pub fn verify_lineage(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-runtime/src/block_validator.rs:117:                    pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/block_validator.rs:125:            ConstitutionalBlockValidator::validate_block(&programs, &mut registry, &[]).unwrap();
crates/amun-constitutional-runtime/src/block_validator.rs:166:                    pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/block_validator.rs:174:            ConstitutionalBlockValidator::validate_block(&programs, &mut registry, &[]).unwrap();
crates/amun-constitutional-runtime/src/block_validator.rs:186:        let _root_before = registry.compute_state_root();
crates/amun-constitutional-runtime/src/block_validator.rs:200:                    pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/block_validator.rs:208:            ConstitutionalBlockValidator::validate_block(&programs, &mut registry, &[]).unwrap();
crates/amun-constitutional-runtime/src/block_validator.rs:212:        assert_eq!(result.state_root, registry.compute_state_root());
crates/amun-constitutional-runtime/src/block_validator.rs:29:    pub fn validate_block(
crates/amun-constitutional-runtime/src/block_validator.rs:71:        let state_root = registry.compute_state_root();
crates/amun-constitutional-runtime/src/certificate_chain.rs:147:        cert.certificate_hash = cert.compute_hash();
crates/amun-constitutional-runtime/src/certificate_chain.rs:158:        assert!(chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:186:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:210:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:35:        let chain_root = Self::compute_chain_root(&certificates);
crates/amun-constitutional-runtime/src/certificate_chain.rs:44:    pub fn verify_chain(&self) -> bool {
crates/amun-constitutional-runtime/src/certificate_chain.rs:60:        Self::compute_chain_root(&self.certificates) == self.chain_root
crates/amun-constitutional-runtime/src/certificate_chain.rs:64:        self.verify_chain()
crates/amun-constitutional-runtime/src/certificate_chain.rs:79:    fn compute_chain_root(
crates/amun-constitutional-runtime/src/finality_certificate.rs:109:    pub fn compute_evidence_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:114:        Self::compute_merkle_root(ids)
crates/amun-constitutional-runtime/src/finality_certificate.rs:117:    pub fn compute_pccv_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:130:        Self::compute_merkle_root(hashes)
crates/amun-constitutional-runtime/src/finality_certificate.rs:150:    fn n52_issue_finality_certificate() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:165:                    pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/finality_certificate.rs:172:            ConstitutionalBlockValidator::validate_block(&programs, &mut registry, &[]).unwrap();
crates/amun-constitutional-runtime/src/finality_certificate.rs:32:            Self::compute_merkle_root(transitions.iter().map(|p| p.proof_hash).collect());
crates/amun-constitutional-runtime/src/finality_certificate.rs:33:        let evidence_root = Self::compute_evidence_root(&transitions);
crates/amun-constitutional-runtime/src/finality_certificate.rs:34:        let pccv_root = Self::compute_pccv_root(&transitions);
crates/amun-constitutional-runtime/src/finality_certificate.rs:54:        cert.certificate_hash = cert.compute_hash();
crates/amun-constitutional-runtime/src/finality_certificate.rs:59:        self.certificate_hash == self.compute_hash() && self.all_verified
crates/amun-constitutional-runtime/src/finality_certificate.rs:66:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:83:    pub fn compute_merkle_root(hashes: Vec<[u8; 32]>) -> [u8; 32] {
crates/amun-constitutional-runtime/src/history_root.rs:139:        cert.certificate_hash = cert.compute_hash();
crates/amun-constitutional-runtime/src/history_root.rs:144:    fn n54_compute_history_root() {
crates/amun-constitutional-runtime/src/history_root.rs:174:    fn n54_verify_chain_against_root() {
crates/amun-constitutional-runtime/src/history_root.rs:181:        assert!(root.verify_chain(&chain));
crates/amun-constitutional-runtime/src/history_root.rs:196:        assert!(!root.verify_chain(&tampered_chain));
crates/amun-constitutional-runtime/src/history_root.rs:38:        let history_root = Self::compute_history_root(chain);
crates/amun-constitutional-runtime/src/history_root.rs:52:    pub fn verify_chain(&self, chain: &CertificateChain) -> bool {
crates/amun-constitutional-runtime/src/history_root.rs:53:        if !chain.verify_chain() {
crates/amun-constitutional-runtime/src/history_root.rs:65:        Self::compute_history_root(chain) == self.history_root
crates/amun-constitutional-runtime/src/history_root.rs:68:    fn compute_history_root(chain: &CertificateChain) -> [u8; 32] {
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:164:        let enhanced_proof = PCCVEngine::build_proof(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:239:    fn n50_execute_with_pccv_integration() {
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:252:            pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:294:            pre_state_root: registry.compute_state_root(),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:332:        let root_before = registry.compute_state_root();
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:358:            registry.compute_state_root(),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:46:        let pre_state_root = registry.compute_state_root();
crates/amun-constitutional-semantics/src/lib.rs:114:    #[test] fn test_commitment_verification() { let e = vec![mk(1,[0x01;32],[0x00;32]), mk(2,[0x02;32],[0x01;32])]; let c = TranscriptCommitment::new_sequential(&e, [0xBB;32]); assert!(c.verify_events(&e)); }
crates/amun-constitutional-semantics/src/lib.rs:115:    #[test] fn test_commitment_detects_tamper() { let e1 = mk(1,[0x01;32],[0x00;32]); let e2 = mk(2,[0x02;32],[0x01;32]); let e3 = mk(3,[0x03;32],[0x02;32]); let c = TranscriptCommitment::new_sequential(&vec![e1.clone(), e2], [0xBB;32]); assert!(!c.verify_events(&vec![e1, e3])); }
crates/amun-constitutional-semantics/src/lib.rs:121:    #[test] fn test_authority_binding() { let b = AuthorityBinding { authority: EventAuthority::Authoritative, authority_set_root: [0xAA;32], authority_epoch: [0xBB;32], authority_proof: AuthorityProof::SingleSignature { validator_id: 1, signature: [0;64] } }; assert!(b.verify_binding(EventAuthority::Authoritative)); assert!(!b.verify_binding(EventAuthority::Derived)); }
crates/amun-constitutional-semantics/src/lib.rs:22:    pub fn verify_events(&self, events: &[TranscriptEntry]) -> bool { let r = Self::new_sequential(events, self.epoch_id); r.root == self.root && r.event_count == self.event_count }
crates/amun-constitutional-semantics/src/lib.rs:23:    pub fn verify_boundary(&self, boundary: &ReplayBoundary) -> bool { self.start_sequence >= boundary.finalized_sequence }
crates/amun-constitutional-semantics/src/lib.rs:43:    pub fn verify_binding(&self, event_authority: EventAuthority) -> bool { self.authority == event_authority }
crates/amun-constitutional-semantics/src/lib.rs:44:    pub fn verify_epoch(&self, current_epoch: &[u8; 32]) -> bool { self.authority_epoch == *current_epoch }
crates/amun-constitutional-semantics/src/lib.rs:58:    pub fn verify_normalization(&self, witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> bool { Self::normalize(witnesses).normalization_root == self.normalization_root }
crates/amun-constitutional-signing/tests/signing_tests.rs:19:fn test_sign_and_verify_federation() {
crates/amun-constitutional-signing/tests/signing_tests.rs:36:fn test_sign_and_verify_treaty() {
crates/amun-constitutional-signing/tests/signing_tests.rs:7:fn test_sign_and_verify_manifest() {
crates/amun-constitutional-state/src/lib.rs:417:    pub fn prove_certificate_inclusion(
crates/amun-constitutional-state/src/lib.rs:470:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-state/src/lib.rs:489:            ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-state/src/lib.rs:503:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &unknown_hash);
crates/amun-constitutional-verifier/src/verifier.rs:145:        assert!(verify_qc(&qc, &set));
crates/amun-constitutional-verifier/src/verifier.rs:146:        assert!(verify_vote_uniqueness(&qc));
crates/amun-constitutional-verifier/src/verifier.rs:36:pub fn verify_vote(vote: &ConsensusVote, validator_set: &ValidatorSet) -> bool {
crates/amun-constitutional-verifier/src/verifier.rs:59:pub fn verify_vote_uniqueness(qc: &QuorumCertificate) -> bool {
crates/amun-constitutional-verifier/src/verifier.rs:6:pub fn verify_qc(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
crates/amun-constitutional-verifier/src/verifier.rs:99:    fn test_verify_qc() {
crates/amun-constitutional/src/artifact_graph.rs:132:    pub fn verify_all_edges(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/artifact_graph.rs:155:        assert!(g.verify_all_edges().is_ok());
crates/amun-constitutional/src/causal_edge.rs:110:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causal_edge.rs:188:        assert!(e.verify_structure().is_err());
crates/amun-constitutional/src/causal_edge.rs:75:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causal_edge.rs:97:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causality_chain.rs:102:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causality_chain.rs:115:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causality_chain.rs:128:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/causality_chain.rs:231:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/causality_chain.rs:254:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/causality_chain.rs:77:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/certificate_scope.rs:156:    pub fn verify_against_parent(
crates/amun-constitutional/src/certificate_scope.rs:250:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/certificate_scope.rs:253:        assert!(child.verify_against_parent(&parent).is_ok());
crates/amun-constitutional/src/certificate_scope.rs:257:    fn test_verify_against_parent_divergent() {
crates/amun-constitutional/src/certificate_scope.rs:261:        assert!(child.verify_against_parent(&parent).is_err());
crates/amun-constitutional/src/constitutional_failure.rs:111:    pub fn verify_structure(&self) -> Result<(), Self> {
crates/amun-constitutional/src/constitutional_failure.rs:132:    pub fn verify_semantics(&self) -> Result<(), Self> {
crates/amun-constitutional/src/constitutional_failure.rs:144:    pub fn verify_provenance(&self) -> Result<(), Self> {
crates/amun-constitutional/src/constitutional_failure.rs:159:    pub fn verify_admissibility_graph(&self) -> Result<(), Self> {
crates/amun-constitutional/src/constitutional_failure.rs:174:    pub fn verify_constitutional(&self) -> Result<(), Self> {
crates/amun-constitutional/src/constitutional_failure.rs:175:        self.verify_admissibility_graph()?;
crates/amun-constitutional/src/constitutional_failure.rs:179:        self.verify_structure()?;
crates/amun-constitutional/src/constitutional_failure.rs:180:        self.verify_semantics()?;
crates/amun-constitutional/src/constitutional_failure.rs:181:        self.verify_provenance()?;
crates/amun-constitutional/src/constitutional_failure.rs:182:        self.verify_constitutional()?;
crates/amun-constitutional/src/constitutional_object.rs:12:        self.verify_structure()?;
crates/amun-constitutional/src/constitutional_object.rs:13:        self.verify_semantics()?;
crates/amun-constitutional/src/constitutional_object.rs:14:        self.verify_provenance()?;
crates/amun-constitutional/src/constitutional_object.rs:15:        self.verify_constitutional()?;
crates/amun-constitutional/src/constitutional_object.rs:5:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure>;
crates/amun-constitutional/src/constitutional_object.rs:6:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure>;
crates/amun-constitutional/src/constitutional_object.rs:7:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure>;
crates/amun-constitutional/src/constitutional_object.rs:8:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/constitutional_witness.rs:125:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/constitutional_witness.rs:147:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/constitutional_witness.rs:160:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/constitutional_witness.rs:173:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/constitutional_witness.rs:297:        assert!(w.verify_structure().is_err());
crates/amun-constitutional/src/constitutional_witness.rs:306:        assert!(w.verify_constitutional().is_err());
crates/amun-constitutional/src/continuation_chain.rs:103:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/continuation_chain.rs:116:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/continuation_chain.rs:162:    pub fn verify_continuation(
crates/amun-constitutional/src/continuation_chain.rs:220:    fn test_verify_continuation_ok() {
crates/amun-constitutional/src/continuation_chain.rs:225:        assert!(chain.verify_continuation(&rp).is_ok());
crates/amun-constitutional/src/continuation_chain.rs:233:        assert!(chain.verify_continuation(&rp).is_err());
crates/amun-constitutional/src/continuation_chain.rs:90:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_point.rs:106:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_point.rs:119:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_point.rs:93:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_resolution.rs:100:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_resolution.rs:113:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/divergence_resolution.rs:126:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_boundary.rs:107:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_boundary.rs:63:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_boundary.rs:95:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_context.rs:50:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_context.rs:62:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_context.rs:74:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:174:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:186:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:198:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:54:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:66:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_journal.rs:78:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_limits.rs:114:    pub fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_limits.rs:126:    pub fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_limits.rs:138:    pub fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_limits.rs:150:    pub fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_limits.rs:181:        self.verify_structure()?;
crates/amun-constitutional/src/execution_limits.rs:182:        self.verify_semantics()?;
crates/amun-constitutional/src/execution_limits.rs:183:        self.verify_provenance()?;
crates/amun-constitutional/src/execution_limits.rs:184:        self.verify_constitutional()?;
crates/amun-constitutional/src/execution_receipt.rs:120:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_receipt.rs:133:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_receipt.rs:146:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/execution_receipt.rs:159:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:101:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:114:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:177:    pub fn verify_scope_against_parent(
crates/amun-constitutional/src/replay_certificate.rs:181:        match self.scope.verify_against_parent(&parent.scope) {
crates/amun-constitutional/src/replay_certificate.rs:240:        assert!(c.verify_scope_against_parent(&p).is_ok());
crates/amun-constitutional/src/replay_certificate.rs:260:        assert!(c.verify_scope_against_parent(&p).is_err());
crates/amun-constitutional/src/replay_certificate.rs:267:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/replay_certificate.rs:274:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/replay_certificate.rs:66:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:88:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/restoration_point.rs:103:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/restoration_point.rs:116:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/restoration_point.rs:90:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/schema_registry.rs:125:    pub const fn verify_range() -> bool {
crates/amun-constitutional/src/schema_registry.rs:163:        assert!(core_schemas::verify_uniqueness());
crates/amun-constitutional/src/schema_registry.rs:168:        assert!(core_schemas::verify_range());
crates/amun-constitutional/src/schema_registry.rs:89:    pub const fn verify_uniqueness() -> bool {
crates/amun-constitutional/src/snapshot.rs:104:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/snapshot.rs:126:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/snapshot.rs:139:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/snapshot.rs:152:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/snapshot.rs:225:    pub fn verify_against_parent(
crates/amun-constitutional/src/snapshot.rs:229:        match self.scope.verify_against_parent(&parent.scope) {
crates/amun-constitutional/src/snapshot.rs:320:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot.rs:338:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/snapshot.rs:345:        assert!(s.verify_constitutional().is_err());
crates/amun-constitutional/src/snapshot_scope.rs:153:    pub fn verify_against_parent(
crates/amun-constitutional/src/snapshot_scope.rs:214:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/snapshot_scope.rs:217:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot_scope.rs:220:    fn test_verify_against_parent_divergent_rejected() {
crates/amun-constitutional/src/snapshot_scope.rs:224:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor.rs:120:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/state_anchor.rs:133:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/state_anchor.rs:146:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/state_anchor.rs:208:    pub fn verify_against_parent(
crates/amun-constitutional/src/state_anchor.rs:212:        match self.scope.verify_against_parent(&parent.scope) {
crates/amun-constitutional/src/state_anchor.rs:270:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:276:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:294:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor.rs:300:        assert!(a.verify_structure().is_err());
crates/amun-constitutional/src/state_anchor.rs:307:        assert!(a.verify_constitutional().is_err());
crates/amun-constitutional/src/state_anchor.rs:98:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/state_anchor_scope.rs:123:    pub fn verify_against_parent(
crates/amun-constitutional/src/state_anchor_scope.rs:190:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/state_anchor_scope.rs:193:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:196:    fn test_verify_against_parent_state_transition_ok() {
crates/amun-constitutional/src/state_anchor_scope.rs:199:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:202:    fn test_verify_against_parent_divergent_rejected() {
crates/amun-constitutional/src/state_anchor_scope.rs:206:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/transition_commitment.rs:52:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/transition_commitment.rs:64:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/transition_commitment.rs:76:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/transition_evidence.rs:55:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/transition_evidence.rs:67:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/transition_evidence.rs:79:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-contract-events/src/lib.rs:67:    pub fn compute_events_root(&self) -> [u8; 32] {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:31:        storage1.compute_events_root(),
crates/amun-contract-events/tests/n173_events_storage_tests.rs:32:        storage2.compute_events_root()
crates/amun-contract-fuzzing/src/lib.rs:117:        let root1 = cr.compute_registry_root();
crates/amun-contract-fuzzing/src/lib.rs:118:        let root2 = cr.compute_registry_root();
crates/amun-contract-fuzzing/src/lib.rs:166:        let root1 = cr.compute_registry_root();
crates/amun-contract-fuzzing/src/lib.rs:167:        let root2 = cr.compute_registry_root();
crates/amun-contract-fuzzing/src/lib.rs:73:        let root1 = cr.compute_registry_root();
crates/amun-contract-fuzzing/src/lib.rs:74:        let root2 = cr.compute_registry_root();
crates/amun-contract-integration/src/lib.rs:66:        let (result, _evidence) = GasEngine::execute_with_gas(
crates/amun-contract-integration/src/lib.rs:78:                    pre_state_root: registry.compute_state_root(),
crates/amun-contract-integration/src/lib.rs:93:    pub fn compute_contract_evidence_root(registry: &ResourceRegistry) -> [u8; 32] {
crates/amun-contract-integration/src/lib.rs:97:        hasher.update(registry.compute_state_root());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:46:    let root1 = ContractExecutor::compute_contract_evidence_root(&reg1);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:47:    let root2 = ContractExecutor::compute_contract_evidence_root(&reg2);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:91:    assert_eq!(cr1.compute_registry_root(), cr2.compute_registry_root());
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:44:    let root_before = cr.compute_registry_root();
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:46:    let root_after = cr.compute_registry_root();
crates/amun-contract-registry/src/lib.rs:84:    pub fn compute_registry_root(&self) -> [u8; 32] {
crates/amun-contract-sdk/src/lib.rs:59:    pub fn compute_root(&self) -> [u8; 32] {
crates/amun-contract-sdk/src/lib.rs:60:        self.contract_registry.compute_registry_root()
crates/amun-contract-security/src/lib.rs:113:    let root1 = cr1.compute_registry_root();
crates/amun-contract-security/src/lib.rs:114:    let root2 = cr2.compute_registry_root();
crates/amun-contract-security/src/lib.rs:141:    let root1 = contract_reg.compute_registry_root();
crates/amun-contract-security/src/lib.rs:142:    let root2 = contract_reg.compute_registry_root();
crates/amun-core-optimization/src/lib.rs:24:    pub fn compute_state_root(&mut self) -> [u8; 32] {
crates/amun-core-optimization/src/lib.rs:25:        let root = self.registry.compute_state_root();
crates/amun-core-optimization/src/lib.rs:38:        self.registry.compute_state_root()
crates/amun-core-optimization/tests/n161_optimization_tests.rs:48:    let _root1 = reg.compute_state_root();
crates/amun-core-optimization/tests/n161_optimization_tests.rs:52:    let _root2 = opt_reg.compute_state_root();
crates/amun-crash-recovery/src/recovery.rs:16:    pub fn verify_recovery(&self) -> Result<bool, String> {
crates/amun-crash-recovery/src/recovery.rs:21:        WriteAheadLog::verify_chain_continuity(&entries)?;
crates/amun-crypto-hardening/src/key_rotation.rs:160:        assert!(chain.verify_chain());
crates/amun-crypto-hardening/src/key_rotation.rs:98:    pub fn verify_chain(&self) -> bool {
crates/amun-crypto/src/audit.rs:19:    pub fn verify_entropy(&mut self, epoch: u64) -> bool {
crates/amun-defi-amm/src/lib.rs:32:        let pool_id_bytes = DefiPool::compute_pool_id(token_a, token_b);
crates/amun-defi-amm/src/lib.rs:71:    pub fn compute_evidence_root(&self) -> [u8; 32] {
crates/amun-defi-amm/tests/n153_amm_tests.rs:32:    assert_eq!(amm1.compute_evidence_root(), amm2.compute_evidence_root());
crates/amun-defi-amm/tests/n153_amm_tests.rs:43:    let root_before = amm.compute_evidence_root();
crates/amun-defi-amm/tests/n153_amm_tests.rs:45:    let root_after = amm.compute_evidence_root();
crates/amun-defi-core/src/lib.rs:77:    pub fn compute_pool_id(token_a: [u8; 32], token_b: [u8; 32]) -> [u8; 32] {
crates/amun-defi-governance/src/lib.rs:115:    pub fn compute_governance_root(&self) -> [u8; 32] {
crates/amun-defi-governance/tests/n157_governance_tests.rs:25:        engine1.compute_governance_root(),
crates/amun-defi-governance/tests/n157_governance_tests.rs:26:        engine2.compute_governance_root()
crates/amun-defi-governance/tests/n157_governance_tests.rs:33:    let root_before = engine.compute_governance_root();
crates/amun-defi-governance/tests/n157_governance_tests.rs:37:    let root_after = engine.compute_governance_root();
crates/amun-defi-lending-core/src/lib.rs:31:    pub fn compute_interest(principal: u64, rate_bps: u64, blocks_elapsed: u64) -> u64 {
crates/amun-defi-lending-core/src/lib.rs:39:    pub fn compute_health_factor(collateral: u64, debt: u64, liquidation_threshold: u64) -> u64 {
crates/amun-defi-lending-engine/src/lib.rs:142:            let pending_interest = InterestModel::compute_interest(
crates/amun-defi-lending-engine/src/lib.rs:148:            InterestModel::compute_health_factor(loan.collateral_locked, total_debt, 8000)
crates/amun-defi-lending-engine/src/lib.rs:154:    pub fn compute_lending_root(&self) -> [u8; 32] {
crates/amun-defi-lending-engine/src/lib.rs:87:            let interest = InterestModel::compute_interest(
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:109:        engine1.compute_lending_root(),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:110:        engine2.compute_lending_root()
crates/amun-defi-stablecoin/src/lib.rs:78:    pub fn compute_stablecoin_root(&self) -> [u8; 32] {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:32:        engine1.compute_stablecoin_root(),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:33:        engine2.compute_stablecoin_root()
crates/amun-defi-stress/src/lib.rs:112:            let root_before = engine.compute_stablecoin_root();
crates/amun-defi-stress/src/lib.rs:115:            let root_after = engine.compute_stablecoin_root();
crates/amun-defi-stress/src/lib.rs:52:            let root_before = amm.compute_evidence_root();
crates/amun-defi-stress/src/lib.rs:54:            let root_after = amm.compute_evidence_root();
crates/amun-deterministic-scheduler/src/scheduler.rs:59:    pub fn execute_batch(&mut self, max_tasks: usize) -> Vec<ExecutedTask> {
crates/amun-dual-verification/src/dual_verifier.rs:14:    pub fn execute_and_verify(
crates/amun-dual-verification/src/dual_verifier.rs:22:        let pre_root = registry.compute_state_root();
crates/amun-dual-verification/src/dual_verifier.rs:42:                let enhanced_proof = PCCVEngine::build_proof(
crates/amun-economic-service/src/main.rs:48:                "ledger_root": hex::encode(ledger.compute_ledger_root()),
crates/amun-entropy-transcript/src/transcript.rs:33:    pub fn verify_replay(&self, other: &EntropyTranscript) -> bool {
crates/amun-evidence-finality/src/evidence_finality.rs:106:        cert.certificate_hash = cert.compute_hash();
crates/amun-evidence-finality/src/evidence_finality.rs:110:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evidence-finality/src/evidence_finality.rs:127:        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
crates/amun-evidence-finality/src/evidence_finality.rs:146:    pub fn execute_and_verify(
crates/amun-evidence-finality/src/evidence_finality.rs:202:        let state_root = registry.compute_state_root();
crates/amun-evidence-finality/src/evidence_finality.rs:205:        let evidence_root = EvidenceVerifiedBlock::compute_evidence_root(&all_evidence);
crates/amun-evidence-finality/src/evidence_finality.rs:206:        let block_hash = EvidenceVerifiedBlock::compute_block_hash(
crates/amun-evidence-finality/src/evidence_finality.rs:26:    pub fn compute_evidence_root(evidence: &[ConstitutionalEvidence]) -> [u8; 32] {
crates/amun-evidence-finality/src/evidence_finality.rs:294:            pre_state_root: registry.compute_state_root(),
crates/amun-evidence-finality/src/evidence_finality.rs:297:        let block = EvidenceBackedConsensus::execute_and_verify(
crates/amun-evidence-finality/src/evidence_finality.rs:321:            pre_state_root: registry.compute_state_root(),
crates/amun-evidence-finality/src/evidence_finality.rs:324:        let block = EvidenceBackedConsensus::execute_and_verify(
crates/amun-evidence-finality/src/evidence_finality.rs:352:            pre_state_root: registry.compute_state_root(),
crates/amun-evidence-finality/src/evidence_finality.rs:355:        let block = EvidenceBackedConsensus::execute_and_verify(
crates/amun-evidence-finality/src/evidence_finality.rs:38:    pub fn compute_block_hash(
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:29:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:68:        let b = builder.build_block(i, parent, &mut mempool, 100, [9u8; 32], i * 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:98:    builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence/src/equivocation.rs:31:    pub fn compute_id(&self) -> [u8; 32] {
crates/amun-evidence/src/evidence.rs:61:    pub fn compute_hash(&self) -> BlockHash {
crates/amun-evidence/src/tests.rs:31:        let hash = evidence.compute_hash();
crates/amun-evidence/src/tests.rs:36:    fn test_evidence_verify_same_hash_rejected() {
crates/amun-evolution/src/certificate.rs:119:        cert.certificate_hash = cert.compute_hash();
crates/amun-evolution/src/certificate.rs:123:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evolution/src/certificate.rs:147:        self.physics_proof.verify() && self.compute_hash() == self.certificate_hash
crates/amun-evolution/src/certificate.rs:34:        proof.proof_hash = proof.compute_hash();
crates/amun-evolution/src/certificate.rs:38:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evolution/src/certificate.rs:49:        self.compute_hash() == self.proof_hash
crates/amun-evolution/src/signatures.rs:34:        sig.signature_hash = sig.compute_hash();
crates/amun-evolution/src/signatures.rs:38:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evolution/src/signatures.rs:48:    pub fn verify_structure(&self) -> bool {
crates/amun-evolution/src/signatures.rs:49:        self.compute_hash() == self.signature_hash
crates/amun-evolution/src/signatures.rs:74:        agg.aggregate_hash = agg.compute_hash();
crates/amun-evolution/src/signatures.rs:78:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evolution/src/signatures.rs:89:        self.compute_hash() == self.aggregate_hash && self.quorum_reached
crates/amun-evolution/src/transition.rs:36:        cp.checkpoint_hash = cp.compute_hash();
crates/amun-evolution/src/transition.rs:40:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-evolution/src/transition.rs:53:        self.compute_hash() == self.checkpoint_hash
crates/amun-evolution/src/validator.rs:9:    pub fn verify_consistency(
crates/amun-execution-receipt/src/lib.rs:143:    pub fn verify_against_initial_root(
crates/amun-execution-receipt/src/lib.rs:162:    pub fn verify_transcript(&self) -> Result<(), Vec<String>> {
crates/amun-execution-receipt/src/lib.rs:167:            if let Err(e) = receipt.verify_consistency() {
crates/amun-execution-receipt/src/lib.rs:93:    pub fn verify_consistency(&self) -> Result<(), &'static str> {
crates/amun-execution/src/executor.rs:18:    pub fn execute_wasm(&mut self, code: &[u8], input: &[u8]) -> AmunResult<Vec<u8>> {
crates/amun-execution/src/executor.rs:20:        let result = self.interpreter.execute_deterministic(code, input)?;
crates/amun-execution/src/lib.rs:108:    pub fn execute_block(&mut self, txs: &[Transaction]) -> Vec<TransactionReceipt> {
crates/amun-execution/src/lib.rs:118:            .build_economic_snapshot_with_ledger(&self.economic)
crates/amun-execution/src/lib.rs:234:    fn n26_execute_block() {
crates/amun-execution/src/lib.rs:250:        let receipts = engine.execute_block(&[tx1, tx2]);
crates/amun-execution/src/tests.rs:31:fn test_wasm_profile_verify_module_ok() {
crates/amun-execution/src/tests.rs:33:    assert!(profile.verify_module(b"").is_ok());
crates/amun-execution/src/tests.rs:50:fn test_verify_deterministic_wasm_ok() {
crates/amun-execution/src/tests.rs:52:    assert!(wasm_deterministic_subset::verify_deterministic_wasm(b"").is_ok());
crates/amun-execution/src/verified_interpreter.rs:38:            .verify_module(module)
crates/amun-execution/src/wasm_deterministic.rs:46:    pub fn verify_deterministic_wasm(_module: &[u8]) -> Result<(), &'static str> {
crates/amun-execution/src/wasm_profile.rs:78:    pub fn verify_module(&self, _module: &[u8]) -> Result<(), &'static str> {
crates/amun-experimental-framework/src/main.rs:183:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:190:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:242:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:249:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:256:        let exec_stats = measure_us(&format!("execute_{}", name), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:26:fn compute_stats(times: &[f64]) -> Stats {
crates/amun-experimental-framework/src/main.rs:353:                    pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:396:            let (mut reg, tip) = build_deep_chain(depth);
crates/amun-experimental-framework/src/main.rs:438:            let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:445:                pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:46:    let s = compute_stats(&times);
crates/amun-experimental-framework/src/main.rs:489:        let stats = measure_us(&format!("verify_{}_resources", size), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:490:            let reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:491:            let _root = reg.compute_state_root();
crates/amun-experimental-framework/src/main.rs:65:fn build_registry(size: u64) -> ResourceRegistry {
crates/amun-experimental-framework/src/main.rs:81:fn build_deep_chain(depth: u64) -> (ResourceRegistry, ResourceId) {
crates/amun-explorer-api/src/server.rs:4:pub fn build_app() -> Router {
crates/amun-failure-memory/src/ontology.rs:28:        self.recompute_chain_hash();
crates/amun-failure-memory/src/ontology.rs:32:    fn recompute_chain_hash(&mut self) {
crates/amun-failure/src/kernel_state.rs:42:                invalidate_snapshots: true,
crates/amun-failure/src/kernel_state.rs:54:    pub invalidate_snapshots: bool,
crates/amun-failure/src/kernel_state.rs:64:            invalidate_snapshots: false,
crates/amun-failure/src/tests.rs:96:    assert!(actions.invalidate_snapshots);
crates/amun-finality-certificate/src/lib.rs:111:    fn issue_cert(h: u64) -> ConstitutionalFinalityCertificate {
crates/amun-finality-certificate/src/lib.rs:126:        let cert = issue_cert(1);
crates/amun-finality-certificate/src/lib.rs:133:        let c1 = issue_cert(1);
crates/amun-finality-certificate/src/lib.rs:134:        let c2 = issue_cert(1);
crates/amun-finality-certificate/src/lib.rs:140:        let mut cert = issue_cert(1);
crates/amun-finality-certificate/src/lib.rs:147:        let mut cert = issue_cert(1);
crates/amun-finality-certificate/src/lib.rs:45:        cert.finality_id = cert.compute_id();
crates/amun-finality-certificate/src/lib.rs:49:    fn compute_id(&self) -> [u8; 32] {
crates/amun-finality-certificate/src/lib.rs:64:        self.finality_id == self.compute_id()
crates/amun-gas-engine/src/gas_engine.rs:101:            GasEngine::execute_with_gas(100, make_id(2), 42, [0xbb; 32], |meter| {
crates/amun-gas-engine/src/gas_engine.rs:125:            GasEngine::execute_with_gas(500, make_id(3), 10, [0xcc; 32], |meter| {
crates/amun-gas-engine/src/gas_engine.rs:136:            GasEngine::execute_with_gas(200, make_id(4), 1, [0xdd; 32], |meter| {
crates/amun-gas-engine/src/gas_engine.rs:14:    pub fn execute_with_gas<F>(
crates/amun-gas-engine/src/gas_engine.rs:19:        execute_fn: F,
crates/amun-gas-engine/src/gas_engine.rs:25:        match execute_fn(&mut meter) {
crates/amun-gas-engine/src/gas_engine.rs:87:    fn w7_execute_within_gas() {
crates/amun-gas-engine/src/gas_engine.rs:89:            GasEngine::execute_with_gas(1000, make_id(1), 1, [0xaa; 32], |meter| {
crates/amun-gas-engine/src/gas_engine.rs:99:    fn w7_execute_out_of_gas_produces_evidence() {
crates/amun-genesis/src/block.rs:41:    pub fn verify_integrity(&self) -> bool {
crates/amun-genesis/src/validator.rs:26:pub fn compute_validator_set_hash(validators: &[GenesisValidator]) -> [u8; 32] {
crates/amun-handle-resolver/src/handle_resolver.rs:166:            HandleResolver::validate_handle_safety(&buffer, make_id(99), 1, [0xcc; 32]);
crates/amun-handle-resolver/src/handle_resolver.rs:179:            HandleResolver::validate_handle_safety(&buffer, make_id(99), 1, [0xdd; 32]);
crates/amun-handle-resolver/src/handle_resolver.rs:75:    pub fn validate_handle_safety(
crates/amun-host/src/boundary.rs:7:    pub fn validate_ingress(payload: &[u8], max_size: usize) -> Result<&[u8], &'static str> {
crates/amun-light-client/src/constitutional_client.rs:153:        cert.certificate_hash = cert.compute_hash();
crates/amun-light-client/src/constitutional_client.rs:178:    fn n55_verify_valid_certificate() {
crates/amun-light-client/src/constitutional_client.rs:181:        assert!(client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:189:        assert!(!client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:193:    fn n55_verify_chain_extension() {
crates/amun-light-client/src/constitutional_client.rs:210:        assert!(client.verify_chain_extension(&chain));
crates/amun-light-client/src/constitutional_client.rs:37:    pub fn verify_certificate(&self, cert: &ConstitutionalFinalityCertificate) -> bool {
crates/amun-light-client/src/constitutional_client.rs:41:    pub fn verify_chain_extension(&self, chain: &CertificateChain) -> bool {
crates/amun-light-client/src/constitutional_client.rs:46:            if !chain.verify_chain() {
crates/amun-light-client/src/constitutional_client.rs:51:            chain.verify_chain()
crates/amun-light-client/src/constitutional_client.rs:55:    pub fn verify_history_root(
crates/amun-light-client/src/constitutional_client.rs:60:        root.verify_chain(chain)
crates/amun-light-client/src/constitutional_client.rs:64:        if !self.verify_chain_extension(chain) {
crates/amun-light-client/src/constitutional_client.rs:87:    pub fn verify_checkpoint(checkpoint: &ConstitutionalCheckpoint) -> bool {
crates/amun-light-client/tests/light_client_tests.rs:107:    assert!(!client.verify_chain_extension(&chain));
crates/amun-light-client/tests/light_client_tests.rs:53:    cert.certificate_hash = cert.compute_hash();
crates/amun-lineage-law/src/ancestry.rs:63:        chain.chain_hash = chain.compute_hash();
crates/amun-lineage-law/src/ancestry.rs:77:        self.chain_hash = self.compute_hash();
crates/amun-lineage-law/src/ancestry.rs:80:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/ancestry.rs:92:        self.compute_hash() == self.chain_hash
crates/amun-lineage-law/src/compatibility.rs:48:        theorem.theorem_hash = theorem.compute_hash();
crates/amun-lineage-law/src/compatibility.rs:52:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/compatibility.rs:65:        self.compute_hash() == self.theorem_hash
crates/amun-lineage-law/src/lineage.rs:128:        Self::verify_chain(chain)
crates/amun-lineage-law/src/lineage.rs:37:        proof.proof_hash = proof.compute_hash();
crates/amun-lineage-law/src/lineage.rs:41:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/lineage.rs:54:        let computed = self.compute_hash();
crates/amun-lineage-law/src/lineage.rs:77:        cert.certificate_hash = cert.compute_hash();
crates/amun-lineage-law/src/lineage.rs:81:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/lineage.rs:90:        self.compute_hash() == self.certificate_hash && self.proof.is_verified
crates/amun-lineage-law/src/lineage.rs:99:    pub fn verify_chain(chain: &[LineageProof]) -> bool {
crates/amun-lineage-law/src/migration.rs:119:        rules.rules_hash = rules.compute_hash();
crates/amun-lineage-law/src/migration.rs:123:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/migration.rs:135:        self.compute_hash() == self.rules_hash
crates/amun-lineage-law/src/migration.rs:38:        witness.witness_hash = witness.compute_hash();
crates/amun-lineage-law/src/migration.rs:42:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/migration.rs:54:        self.compute_hash() == self.witness_hash
crates/amun-lineage-law/src/migration.rs:74:        cert.certificate_hash = cert.compute_hash();
crates/amun-lineage-law/src/migration.rs:78:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage-law/src/migration.rs:87:        self.compute_hash() == self.certificate_hash && self.witness.verify()
crates/amun-lineage/src/record.rs:145:        record.record_hash = record.compute_hash();
crates/amun-lineage/src/record.rs:149:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage/src/record.rs:161:        self.compute_hash() == self.record_hash
crates/amun-lineage/src/record.rs:197:        ep.proof_hash = ep.compute_hash();
crates/amun-lineage/src/record.rs:201:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-lineage/src/record.rs:214:        self.compute_hash() == self.proof_hash
crates/amun-live-cluster/src/bin/e2e_test.rs:24:    let app = build_app(state);
crates/amun-live-cluster/src/bin/e2e_test.rs:4:use amun_rpc::{build_app, AppState};
crates/amun-live-cluster/src/bin/rpc_server.rs:35:    let app = build_app(state);
crates/amun-live-cluster/src/bin/rpc_server.rs:3:use amun_rpc::{build_app, AppState};
crates/amun-live-cluster/src/validator.rs:1073:        let v1_cert = ValidatorCertificate::issue_v2(
crates/amun-live-cluster/src/validator.rs:112:        let self_cert = amun_networking::validator_certificate::ValidatorCertificate::issue_v2(
crates/amun-live-cluster/src/validator.rs:1136:                reg.verify_certificate_at(&v1_cert, 400),
crates/amun-live-cluster/src/validator.rs:1145:                reg.verify_certificate_at(&v1_cert, 550),
crates/amun-live-cluster/src/validator.rs:1154:                !reg.verify_certificate_at(&v1_cert, 650),
crates/amun-live-cluster/src/validator.rs:1185:                !reg.verify_certificate_at(&v1_cert, 800),
crates/amun-live-cluster/src/validator.rs:121:        if !registry.verify_certificate_at(&self_cert, 0) {
crates/amun-live-cluster/src/validator.rs:139:            if !registry.verify_certificate_at(&peer_cert, 0) {
crates/amun-live-cluster/src/validator.rs:404:                    let mut block = bld.build_block_with_certificates(
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:12:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:17:        block.verify_slashing_root(&root).is_ok(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:26:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:30:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:41:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:50:    let result = block.verify_slashing_root(&[0x42; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:66:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:78:        block.verify_slashing_root(&validator_root).is_err(),
crates/amun-load-generator/src/main.rs:118:    tx.tx_hash = tx.compute_hash();
crates/amun-mempool-gossip/src/mempool.rs:215:        tx.tx_hash = tx.compute_hash();
crates/amun-mempool-gossip/src/mempool.rs:26:        if !tx.verify_hash() {
crates/amun-mempool-gossip/src/messages.rs:38:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-mempool-gossip/src/messages.rs:51:    pub fn verify_hash(&self) -> bool {
crates/amun-mempool-gossip/src/messages.rs:52:        self.tx_hash == self.compute_hash()
crates/amun-mempool-gossip/src/messages.rs:70:        tx.tx_hash = tx.compute_hash();
crates/amun-mempool-gossip/src/messages.rs:91:        assert!(tx.verify_hash());
crates/amun-mempool-gossip/src/messages.rs:98:        assert!(!tx.verify_hash());
crates/amun-merkle/src/proof.rs:156:        let root = MerkleTree::compute_root(&[l1, l2]);
crates/amun-merkle/src/proof.rs:171:        assert!(decoded.verify(&l1, &MerkleTree::compute_root(&[l1, l2])));
crates/amun-merkle/src/tests.rs:33:    let root = MerkleTree::compute_root(&[leaf]);
crates/amun-merkle/src/tests.rs:42:        MerkleTree::compute_root(&[l1, l2]),
crates/amun-merkle/src/tests.rs:43:        MerkleTree::compute_root(&[l1, l2])
crates/amun-merkle/src/tests.rs:53:        MerkleTree::compute_root(&[l1, l2, l3]),
crates/amun-merkle/src/tests.rs:54:        MerkleTree::compute_root(&[l1, l2, l3])
crates/amun-merkle/src/tests.rs:63:    let root3 = MerkleTree::compute_root(&[l1, l2, l3]);
crates/amun-merkle/src/tests.rs:64:    let root4 = MerkleTree::compute_root(&[l1, l2, l3, l3]);
crates/amun-merkle/src/tests.rs:73:        MerkleTree::compute_root(&[l1, l2]),
crates/amun-merkle/src/tests.rs:74:        MerkleTree::compute_root(&[l2, l1])
crates/amun-merkle/src/tests.rs:82:    let root = MerkleTree::compute_root(&[l1, l2]);
crates/amun-merkle/src/tests.rs:95:    let root = MerkleTree::compute_root(&[l1, l2]);
crates/amun-merkle/src/tree.rs:26:    pub fn compute_root(leaves: &[PublicHash32]) -> PublicHash32 {
crates/amun-network-scheduler/src/scheduler.rs:60:    pub fn compute_delay(&self, msg_hash: &[u8; 32]) -> u64 {
crates/amun-networking/src/sovereignty.rs:70:    pub fn verify_physics(&mut self) {
crates/amun-networking/src/validator_certificate.rs:106:    fn n21_validator_certificate_issue_and_verify() {
crates/amun-networking/src/validator_certificate.rs:76:    pub fn issue_v2(
crates/amun-networking/src/verification.rs:30:    pub fn verify_remote(
crates/amun-networking/tests/harness/handlers.rs:22:pub fn build_prevote_envelope(
crates/amun-networking/tests/harness/handlers.rs:45:pub fn build_precommit_envelope(
crates/amun-networking/tests/harness/handlers.rs:5:pub fn build_proposal_envelope(node_id: &str, leader_id: [u8; 32], round: u64) -> Envelope {
crates/amun-networking/tests/n18_checkpoint_sync.rs:116:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_checkpoint_sync.rs:117:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n18_checkpoint_sync.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_checkpoint_sync.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_checkpoint_sync.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_checkpoint_sync.rs:54:    let cp_a = build_checkpoint(0, 49);
crates/amun-networking/tests/n18_checkpoint_sync.rs:57:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_a.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:116:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n18_full_rejoin.rs:117:    let cp2 = build_checkpoint(20, 39);
crates/amun-networking/tests/n18_full_rejoin.rs:118:    let cp3 = build_checkpoint(40, 59);
crates/amun-networking/tests/n18_full_rejoin.rs:123:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:124:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:125:    let proof3 = prove_checkpoint_inclusion(&checkpoints, &cp3.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_full_rejoin.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_full_rejoin.rs:58:    let _cp_early = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_full_rejoin.rs:62:    let cp_late = build_checkpoint(10, 49);
crates/amun-networking/tests/n18_full_rejoin.rs:66:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_node_rejoin.rs:77:fn n18_rejoin001_bootstrapping_node_must_verify_before_active() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:101:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:102:    let cp2 = build_checkpoint(40, 59);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:106:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:107:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:11:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:129:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:130:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:134:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:138:        prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:156:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:159:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:22:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n19_adversarial_rejoin.rs:55:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:58:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:73:    let cp_high = build_checkpoint(50, 59);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:81:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_high.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:126:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:129:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:198:    let cp = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:201:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:85:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:8:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:96:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:120:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:122:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:147:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:150:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:65:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:83:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:86:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:107:    let cp = build_checkpoint(100, 199);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:110:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:12:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:131:    let cp_old = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:135:        prove_checkpoint_inclusion(&checkpoints_old, &cp_old.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:141:    let cp_new = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:145:        prove_checkpoint_inclusion(&checkpoints_new, &cp_new.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:162:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:165:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:57:    let cp_late = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:61:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:118:    let result = NftEvidenceKernel::verify_ownership(&reg, &token_id, &thief);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:159:    let root = reg.compute_state_root();
crates/amun-nft-bridge/src/lib.rs:71:            if record.unlock.is_none() && compute_lock_id(&record.lock) == unlock.lock_id {
crates/amun-nft-bridge/src/lib.rs:85:    pub fn compute_bridge_root(&self) -> [u8; 32] {
crates/amun-nft-bridge/src/lib.rs:96:fn compute_lock_id(lock: &BridgeLock) -> [u8; 32] {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:105:    assert_ne!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:81:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:98:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-collateral/src/lib.rs:124:    pub fn compute_evidence_root(&self) -> [u8; 32] {
crates/amun-nft-collateral/src/lib.rs:132:        hasher.update(self.lending.compute_lending_root());
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:144:        engine1.compute_evidence_root(),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:145:        engine2.compute_evidence_root()
crates/amun-nft-constitutional-enforcement/src/lib.rs:29:                let lock_id = compute_lock_id(lock);
crates/amun-nft-constitutional-enforcement/src/lib.rs:62:                return Some(amun_nft_royalty::RoyaltyEngine::compute_royalty(
crates/amun-nft-constitutional-enforcement/src/lib.rs:79:        let nft_root = registry.compute_constitutional_root();
crates/amun-nft-constitutional-enforcement/src/lib.rs:99:fn compute_lock_id(lock: &amun_nft_bridge::BridgeLock) -> [u8; 32] {
crates/amun-nft-constitutional-registry/src/lib.rs:45:    pub fn compute_constitutional_root(&self) -> [u8; 32] {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:124:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:125:        r2.compute_constitutional_root()
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:155:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:156:        r2.compute_constitutional_root()
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:91:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:92:        r2.compute_constitutional_root()
crates/amun-nft-core/src/lib.rs:63:        evidence.evidence_hash = evidence.compute_hash();
crates/amun-nft-core/src/lib.rs:67:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-nft-evidence/src/lib.rs:104:        Self::verify_metadata_hash(ctx.metadata_hash, ctx.actual_metadata_hash)?;
crates/amun-nft-evidence/src/lib.rs:107:        Self::verify_replay_protection(ctx.last_event_time, ctx.timestamp)?;
crates/amun-nft-evidence/src/lib.rs:113:    pub fn verify_transfer(
crates/amun-nft-evidence/src/lib.rs:121:        Self::verify_ownership(registry, token_id, from)?;
crates/amun-nft-evidence/src/lib.rs:124:        Self::verify_replay_protection(last_event_time, timestamp)?;
crates/amun-nft-evidence/src/lib.rs:130:    pub fn verify_burn(
crates/amun-nft-evidence/src/lib.rs:138:        Self::verify_ownership(registry, token_id, owner)?;
crates/amun-nft-evidence/src/lib.rs:141:        Self::verify_replay_protection(last_event_time, timestamp)?;
crates/amun-nft-evidence/src/lib.rs:31:    pub fn verify_ownership(
crates/amun-nft-evidence/src/lib.rs:46:    pub fn verify_non_duplicate(
crates/amun-nft-evidence/src/lib.rs:57:    pub fn verify_metadata_hash(
crates/amun-nft-evidence/src/lib.rs:68:    pub fn verify_replay_protection(
crates/amun-nft-evidence/src/lib.rs:88:    pub fn verify_mint(ctx: MintVerificationContext) -> Result<(), CekError> {
crates/amun-nft-evidence/src/lib.rs:90:        Self::verify_non_duplicate(ctx.registry, ctx.token_id)?;
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:103:    let result = NftEvidenceKernel::verify_replay_protection(1000, 500);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:159:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:40:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:72:    let result = NftEvidenceKernel::verify_transfer(&reg, &token_id, &thief, 2000, 1000);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:91:    let result = NftEvidenceKernel::verify_non_duplicate(&reg, &token_id);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-explorer/src/lib.rs:113:pub fn start_explorer_server(registry: Arc<Mutex<ResourceRegistry>>, bind_addr: &str) {
crates/amun-nft-explorer/src/lib.rs:145:fn handle_request(path: &str, registry: &ResourceRegistry) -> String {
crates/amun-nft-explorer/src/lib.rs:44:    pub fn get_collections(registry: &ResourceRegistry) -> Vec<ExplorerCollection> {
crates/amun-nft-explorer/src/lib.rs:59:    pub fn get_nft(registry: &ResourceRegistry, token_id: &ResourceId) -> Option<ExplorerNft> {
crates/amun-nft-explorer/src/lib.rs:70:    pub fn get_owner_nfts(registry: &ResourceRegistry, owner: &[u8; 32]) -> ExplorerOwner {
crates/amun-nft-fuzz/src/lib.rs:176:        let amount = amun_nft_royalty::RoyaltyEngine::compute_royalty(sale_price, bps);
crates/amun-nft-fuzz/src/lib.rs:248:        let root1 = ledger.compute_bridge_root();
crates/amun-nft-fuzz/src/lib.rs:249:        let root2 = ledger.compute_bridge_root();
crates/amun-nft-governance-execution/src/lib.rs:129:    pub fn compute_execution_root(&self) -> [u8; 32] {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:37:fn n143_execute_passing_proposal() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:98:    assert_eq!(e1.compute_execution_root(), e2.compute_execution_root());
crates/amun-nft-governance/src/lib.rs:64:    pub fn compute_governance_root(&self) -> [u8; 32] {
crates/amun-nft-governance/tests/n138_governance_tests.rs:79:    assert_eq!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-governance/tests/n138_governance_tests.rs:97:    assert_ne!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-indexer/src/lib.rs:148:    pub fn compute_index_root(&self) -> [u8; 32] {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:112:    let old_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:119:    let new_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:92:    assert_eq!(i1.compute_index_root(), i2.compute_index_root());
crates/amun-nft-marketplace/src/lib.rs:333:    pub fn compute_evidence_root(&self) -> [u8; 32] {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:182:    let root = mp.compute_evidence_root();
crates/amun-nft-mining/src/lib.rs:62:pub fn issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:1:use amun_nft_mining::{evaluate_contribution, issue_mining_reward, ContributionType};
crates/amun-nft-mining/tests/n133_mining_tests.rs:35:    let nft_id = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:71:    let id1 = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:80:    let id2 = issue_mining_reward(
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:100:    let (reg2, royalty2, gov2, bridge2) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:102:    assert_eq!(state_root_before, reg2.compute_state_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:103:    assert_eq!(royalty_root_before, royalty2.compute_accounting_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:104:    assert_eq!(gov_root_before, gov2.compute_governance_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:105:    assert_eq!(bridge_root_before, bridge2.compute_bridge_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:16:fn build_full_state() -> (
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:93:    let (reg1, royalty1, gov1, bridge1) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:95:    let state_root_before = reg1.compute_state_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:96:    let royalty_root_before = royalty1.compute_accounting_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:97:    let gov_root_before = gov1.compute_governance_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:98:    let bridge_root_before = bridge1.compute_bridge_root();
crates/amun-nft-rights-enforcement/src/lib.rs:105:fn compute_lock_id(lock: &amun_nft_bridge::BridgeLock) -> [u8; 32] {
crates/amun-nft-rights-enforcement/src/lib.rs:19:    pub fn validate_transfer(
crates/amun-nft-rights-enforcement/src/lib.rs:50:            let lock_id = compute_lock_id(lock);
crates/amun-nft-rights-enforcement/src/lib.rs:75:            .map(|policy| RoyaltyEngine::compute_royalty(sale_price, policy.royalty_bps));
crates/amun-nft-rights-enforcement/src/lib.rs:95:        hasher.update(registry.compute_constitutional_root());
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:12:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:131:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:41:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:82:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-royalty-accounting/src/lib.rs:38:    pub fn compute_accounting_root(&self) -> [u8; 32] {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:74:        ledger1.compute_accounting_root(),
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:75:        ledger2.compute_accounting_root()
crates/amun-nft-royalty-settlement/src/lib.rs:51:    pub fn compute_settlement_root(&self) -> [u8; 32] {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:58:    assert_eq!(e1.compute_settlement_root(), e2.compute_settlement_root());
crates/amun-nft-royalty/src/lib.rs:28:    pub fn compute_royalty(sale_price: u64, royalty_bps: u16) -> u64 {
crates/amun-nft-royalty/src/lib.rs:45:        let royalty_amount = Self::compute_royalty(sale_price, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:31:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:41:    let amount = RoyaltyEngine::compute_royalty(u64::MAX, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:9:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:109:    let state_root = reg.compute_state_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:110:    let royalty_root = royalty.compute_accounting_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:111:    let gov_root = gov.compute_governance_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:112:    let bridge_root = bridge.compute_bridge_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:113:    let const_root = const_reg.compute_constitutional_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:131:    let (_, _, _, _, _, sr1, rr1, gr1, br1, cr1) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:132:    let (_, _, _, _, _, sr2, rr2, gr2, br2, cr2) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:143:    let (_, _, _, _, _, sr, rr, gr, br, cr) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:165:    ) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:218:    assert_ne!(sr_before, reg.compute_state_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:219:    assert_ne!(rr_before, royalty.compute_accounting_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:220:    assert_ne!(gr_before, gov.compute_governance_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:221:    assert_ne!(cr_before, const_reg.compute_constitutional_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:30:fn build_state() -> SnapshotTestState {
crates/amun-nft-stress/tests/n146_stress_tests.rs:183:    let root_before_sale = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:250:    let root_after_all = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:254:    let state_root = reg.compute_state_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:94:    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
crates/amun-node/src/certificate_loader.rs:22:    verify_certificate_against_genesis(&cert, genesis)?;
crates/amun-node/src/certificate_loader.rs:23:    verify_validator_membership(&cert, genesis)?;
crates/amun-node/src/certificate_loader.rs:28:pub fn verify_certificate_against_genesis(
crates/amun-node/src/certificate_loader.rs:55:pub fn verify_validator_membership(
crates/amun-operations/src/backup_recovery.rs:130:        let state_root = reg.compute_state_root();
crates/amun-operations/src/backup_recovery.rs:141:        assert_eq!(restored.compute_state_root(), state_root);
crates/amun-operations/src/backup_recovery.rs:35:        backup.backup_hash = backup.compute_hash();
crates/amun-operations/src/backup_recovery.rs:39:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-operations/src/backup_recovery.rs:54:        self.backup_hash == self.compute_hash()
crates/amun-operations/src/backup_recovery.rs:57:    pub fn restore(&self) -> Result<ResourceRegistry, String> {
crates/amun-pccv/src/lib.rs:104:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:127:            pre_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:128:            post_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:156:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:179:            pre_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:180:            post_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:208:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:233:            pre_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:234:            post_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:266:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:27:    fn n49_structural_verify_empty() {
crates/amun-pccv/src/lib.rs:48:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:72:            pre_state_root: reg.compute_state_root(),
crates/amun-pccv/src/lib.rs:73:            post_state_root: reg.compute_state_root(),
crates/amun-pccv/src/pccv_verifier.rs:114:        if Self::compute_proof_hash(proof) != proof.proof_hash {
crates/amun-pccv/src/pccv_verifier.rs:148:    pub fn compute_proof_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
crates/amun-pccv/src/transition_proof_engine.rs:119:    fn n49b_build_and_verify_simple_transition() {
crates/amun-pccv/src/transition_proof_engine.rs:12:    pub fn build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:132:        let pre_root = reg.compute_state_root();
crates/amun-pccv/src/transition_proof_engine.rs:154:        let proof = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:183:        let pre_root = reg.compute_state_root();
crates/amun-pccv/src/transition_proof_engine.rs:233:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
crates/amun-pccv/src/transition_proof_engine.rs:251:        let pre_root = reg.compute_state_root();
crates/amun-pccv/src/transition_proof_engine.rs:253:        let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:264:        let proof2 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:68:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
crates/amun-pccv/src/transition_proof_engine.rs:72:    pub fn prove_and_verify(
crates/amun-pccv/src/transition_proof_engine.rs:83:        let proof = Self::build_proof(
crates/amun-pccv/src/witness_builder.rs:105:    fn build_lineage_proof(registry: &ResourceRegistry, resource_id: &ResourceId) -> LineageProof {
crates/amun-pccv/src/witness_builder.rs:145:    fn w21_build_witness_from_registry() {
crates/amun-pccv/src/witness_builder.rs:17:                let state_root = registry.compute_state_root();
crates/amun-pccv/src/witness_builder.rs:18:                let merkle_proof = Self::build_merkle_proof(registry, resource_id, state_root);
crates/amun-pccv/src/witness_builder.rs:21:                let lineage_proof = Self::build_lineage_proof(registry, resource_id);
crates/amun-pccv/src/witness_builder.rs:33:    fn build_merkle_proof(
crates/amun-pccv/src/witness_builder.rs:38:        let siblings = Self::compute_siblings(registry, resource_id);
crates/amun-pccv/src/witness_builder.rs:46:    fn compute_siblings(
crates/amun-pccv/tests/replay_equivalence.rs:118:    let pre_root = reg.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:144:            TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:33:    let pre_root1 = reg1.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:34:    let pre_root2 = reg2.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:73:    let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:84:    let proof2 = TransitionProofEngine::build_proof(
crates/amun-persistent-node/src/persistent_store.rs:112:    pub fn registry(&self) -> &ResourceRegistry {
crates/amun-persistent-node/src/persistent_store.rs:115:    pub fn registry_mut(&mut self) -> &mut ResourceRegistry {
crates/amun-persistent-node/src/persistent_store.rs:122:        self.registry.compute_state_root()
crates/amun-proof-carrying/src/verifier.rs:14:    pub fn verify_receipt(receipt: &ProofCarryingReceipt) -> Result<(), String> {
crates/amun-protocol-event/src/transcript.rs:38:    pub fn verify_continuity(&self) -> bool {
crates/amun-qc-canonical/src/validator.rs:5:pub fn validate_qc_validators(
crates/amun-quorum-certificate/src/verifier.rs:5:pub fn verify_quorum(
crates/amun-recovery/src/lib.rs:68:        if !self.replay.verify_chain().unwrap_or(false) {
crates/amun-replay-cert/src/certifier.rs:33:                .compute_chain_root(target_height)
crates/amun-replay-cert/src/certifier.rs:47:                    .compute_chain_root(target_height)
crates/amun-replay-cert/src/transcript.rs:59:    pub fn verify_continuity(&self) -> bool {
crates/amun-replay-cert/src/verifier.rs:5:pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:140:    fn compute_proof_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:166:    fn w18_execute_and_replay_block() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:175:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:178:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:201:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:204:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:21:    pub fn execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:227:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:230:        let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:255:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:258:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:282:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:285:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:293:        assert_eq!(block.replay_root, block.compute_replay_root());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:82:        let state_root = registry.compute_state_root();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:83:        let proof_root = Self::compute_proof_root(&transitions);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:96:        block.replay_root = block.compute_replay_root();
crates/amun-replay-consensus/src/replay_backed_types.rs:107:        cert.certificate_hash = cert.compute_hash();
crates/amun-replay-consensus/src/replay_backed_types.rs:111:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_types.rs:126:        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
crates/amun-replay-consensus/src/replay_backed_types.rs:36:    pub fn compute_replay_root(&self) -> [u8; 32] {
crates/amun-replay-engine/src/adaptive_relay.rs:220:        f.recompute_completeness();
crates/amun-replay-engine/src/constitutional_economics.rs:121:    pub fn verify_no_semantic_reward(&self) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:224:        assert!(incentives.verify_no_semantic_reward());
crates/amun-replay-engine/src/constitutional_governance.rs:47:        s.surface_hash = s.compute_hash();
crates/amun-replay-engine/src/constitutional_governance.rs:51:    fn compute_hash(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/cross_constitution_federation.rs:104:    pub fn verify_sovereignty(&self) -> bool {
crates/amun-replay-engine/src/cross_constitution_federation.rs:186:        assert!(translation.verify_sovereignty());
crates/amun-replay-engine/src/derivational_equivalence.rs:103:        self.class_hash = self.compute_class_hash();
crates/amun-replay-engine/src/derivational_equivalence.rs:79:        c.class_hash = c.compute_class_hash();
crates/amun-replay-engine/src/derivational_equivalence.rs:83:    fn compute_class_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/derivational_frontier.rs:43:        self.recompute_completeness();
crates/amun-replay-engine/src/derivational_frontier.rs:45:    pub fn recompute_completeness(&mut self) {
crates/amun-replay-engine/src/deterministic.rs:113:    pub fn verify_integrity(&self) -> bool {
crates/amun-replay-engine/src/deterministic.rs:52:    pub fn execute_step(
crates/amun-replay-engine/src/deterministic.rs:77:    pub fn execute_with_trace(
crates/amun-replay-engine/src/deterministic.rs:86:            let step = Self::execute_step(entry, current_hash, expected)?;
crates/amun-replay-engine/src/deterministic.rs:94:    pub fn compute_transcript_hash(entries: &[TranscriptEntry]) -> ConstitutionalHash {
crates/amun-replay-engine/src/equivalence.rs:116:    fn execute_and_self_verify_produces_valid_proof() {
crates/amun-replay-engine/src/equivalence.rs:119:        let proof = EquivalenceProver::execute_and_self_verify(&state, &entries, 1).unwrap();
crates/amun-replay-engine/src/equivalence.rs:125:    fn prove_over_result_accepts_valid_execution() {
crates/amun-replay-engine/src/equivalence.rs:137:            DeterministicExecutor::execute_with_trace(&entries, state.state_root, 1).unwrap();
crates/amun-replay-engine/src/equivalence.rs:148:        let proof = EquivalenceProver::prove_over_result(&result, &expected_state).unwrap();
crates/amun-replay-engine/src/equivalence.rs:54:    pub fn prove_over_result(
crates/amun-replay-engine/src/equivalence.rs:67:    pub fn execute_and_self_verify(
crates/amun-replay-engine/src/equivalence.rs:73:        let trace = DeterministicExecutor::execute_with_trace(
crates/amun-replay-engine/src/equivalence.rs:98:        Self::prove_over_result(&result, &state)
crates/amun-replay-engine/src/frontier_reconciliation.rs:100:        f.recompute_completeness();
crates/amun-replay-engine/src/frontier_reconciliation.rs:118:        let d = compute_closure_delta(&source, &target);
crates/amun-replay-engine/src/frontier_reconciliation.rs:42:    merged.recompute_completeness();
crates/amun-replay-engine/src/frontier_reconciliation.rs:79:pub fn compute_closure_delta(source: &DerivationalFrontier, target: &DerivationalFrontier) -> Vec<ConstitutionalHash> {
crates/amun-replay-engine/src/lib.rs:62:        let transcript_hash = DeterministicExecutor::compute_transcript_hash(entries);
crates/amun-replay-engine/src/lib.rs:67:            EquivalenceProver::execute_and_self_verify(&saved_state, entries, start_sequence)?;
crates/amun-replay-engine/src/proof_routing.rs:210:        f.recompute_completeness();
crates/amun-replay-engine/src/runtime_receipt.rs:34:        r.receipt_hash = r.compute_hash();
crates/amun-replay-engine/src/runtime_receipt.rs:38:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/witness_envelope.rs:79:        e.envelope_hash = e.compute_hash();
crates/amun-replay-engine/src/witness_envelope.rs:83:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/zk_adapters.rs:164:        s.surface_binding = s.compute_binding();
crates/amun-replay-engine/src/zk_adapters.rs:168:    fn compute_binding(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/zk_adapters.rs:48:        e.envelope_hash = e.compute_hash();
crates/amun-replay-engine/src/zk_adapters.rs:52:    fn compute_hash(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/zk_adapters.rs:95:        c.commitment = c.compute_commitment();
crates/amun-replay-engine/src/zk_adapters.rs:99:    fn compute_commitment(&self) -> ConstitutionalHash {
crates/amun-replay-optimization/src/lib.rs:70:    pub fn compute_cache_root(&self) -> [u8; 32] {
crates/amun-replay-optimization/src/lib.rs:78:    pub fn batch_verify_certificates(
crates/amun-replay-optimization/tests/n163_replay_tests.rs:40:    let valid = cache.batch_verify_certificates(&cert_hashes, true);
crates/amun-replay-optimization/tests/n163_replay_tests.rs:81:    assert_eq!(cache1.compute_cache_root(), cache2.compute_cache_root());
crates/amun-replay-semantics/src/lib.rs:34:        cert.certificate_hash = cert.compute_hash(); cert
crates/amun-replay-semantics/src/lib.rs:36:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-semantics/src/lib.rs:42:    pub fn verify(&self) -> bool { self.certificate_hash == self.compute_hash() }
crates/amun-replay-semantics/src/lib.rs:43:    pub fn prove_equivalence(a: &Self, b: &Self) -> bool { a.transcript_root == b.transcript_root && a.state_root == b.state_root && a.receipt_root == b.receipt_root && a.ordering_root == b.ordering_root && a.domain == b.domain }
crates/amun-replay-semantics/src/lib.rs:49:        match self { ReplayEquivalence::Strict => a.certificate_hash == b.certificate_hash, ReplayEquivalence::Semantic => ReplayCertificate::prove_equivalence(a, b), ReplayEquivalence::EpochBounded(ep) => a.epoch.epoch_id == ep.epoch_id && b.epoch.epoch_id == ep.epoch_id && ReplayCertificate::prove_equivalence(a, b) }
crates/amun-replay-store/src/lib.rs:120:        assert!(store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:132:        assert!(!store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:155:        assert!(store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:70:    pub fn verify_chain(&self) -> Result<bool, String> {
crates/amun-replay-verifier/src/replay_verifier.rs:141:            pre_state_root: r1.compute_state_root(),
crates/amun-replay-verifier/src/replay_verifier.rs:170:            pre_state_root: r.compute_state_root(),
crates/amun-replay/src/certificate.rs:43:        cert.certificate_id = cert.compute_id();
crates/amun-replay/src/certificate.rs:58:        cert.certificate_id = cert.compute_id();
crates/amun-replay/src/certificate.rs:62:    fn compute_id(&self) -> [u8; 32] {
crates/amun-replay/src/certificate.rs:76:        self.certificate_id == self.compute_id()
crates/amun-replay/src/validation.rs:51:    pub fn validate_log(log: &CommitLog) -> ReplayResult {
crates/amun-resource-core/src/resource_registry.rs:212:    pub fn compute_state_root(&self) -> [u8; 32] {
crates/amun-resource-core/src/resource_registry.rs:505:        let root1 = reg.compute_state_root();
crates/amun-resource-core/src/resource_registry.rs:512:        let root2 = reg2.compute_state_root();
crates/amun-resource-core/src/resource_registry.rs:522:        let root = reg.compute_state_root();
crates/amun-resource-core/tests/n130_constitutional_nft.rs:153:    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
crates/amun-resource-core/tests/stress_tests.rs:141:    let root = reg.compute_state_root();
crates/amun-rpc/src/lib.rs:181:pub fn build_app(state: AppState) -> Router {
crates/amun-rpc/src/lib.rs:325:    let app = build_app(state);
crates/amun-runtime-law/src/law.rs:13:    pub fn verify_code(code: &str) -> Result<(), Vec<&'static str>> {
crates/amun-runtime/src/executor.rs:16:    pub fn execute_batch(
crates/amun-sdk-layer/src/tests.rs:17:    fn test_transaction_builder_transfer() { let sender = amun_kernel_types::PublicKey::new([1u8; 48]); let recipient = amun_kernel_types::PublicHash32::new([2u8; 32]); let result = TransactionBuilder::build_transfer(42, 0, sender, recipient, 100, 1000); assert!(result.success); }
crates/amun-sdk-layer/src/transaction_builder.rs:15:    pub fn build_stake(chain_id: u64, nonce: u64, sender: PublicKey, validator: PublicHash32, amount: u128, gas: u64) -> SdkResult<UnsignedTransaction> {
crates/amun-sdk-layer/src/transaction_builder.rs:8:    pub fn build_transfer(chain_id: u64, nonce: u64, sender: PublicKey, recipient: PublicHash32, amount: u128, gas: u64) -> SdkResult<UnsignedTransaction> {
crates/amun-self-preservation/src/action_principle.rs:63:    pub fn verify_principle(
crates/amun-self-preservation/src/consistency.rs:47:    pub fn verify_consistency(
crates/amun-self-preservation/src/consistency.rs:90:    pub fn can_prove_legitimacy(&self) -> bool {
crates/amun-self-preservation/src/self_modeling.rs:52:    pub fn verify_amendment(model: &SelfModel, new_depth: u64) -> Result<(), String> {
crates/amun-serialization-audit/src/verifier.rs:21:    pub fn verify_roundtrip(original: &[u8]) -> bool {
crates/amun-serialization-audit/src/verifier.rs:6:    pub fn verify_determinism(data: &[&[u8]]) -> bool {
crates/amun-smt/src/lib.rs:34:pub use validator::validate_tree;
crates/amun-smt/src/tree.rs:149:                    return self.build_canonical_subtree(&new_leaves, depth);
crates/amun-smt/src/tree.rs:368:        self.build_canonical_subtree(&leaves, depth)
crates/amun-smt/src/tree.rs:435:        let (vh, ver) = match self.prove_inclusion(0, self.internal_root, &key_hash, &mut steps)?
crates/amun-smt/src/tree.rs:444:    fn prove_inclusion(
crates/amun-smt/src/tree.rs:489:                self.prove_inclusion(next + 1, child, key_hash, steps)
crates/amun-smt/src/tree.rs:501:        self.prove_absence(0, self.internal_root, &key_hash)
crates/amun-smt/src/tree.rs:504:    fn prove_absence(
crates/amun-smt/src/tree.rs:603:                let mut proof = match self.prove_absence(next + 1, child, key_hash)? {
crates/amun-smt/src/tree.rs:665:    fn build_canonical_subtree(
crates/amun-smt/src/tree.rs:703:        let lh = self.build_canonical_subtree(&ll, next_depth + 1)?;
crates/amun-smt/src/tree.rs:704:        let rh = self.build_canonical_subtree(&rl, next_depth + 1)?;
crates/amun-smt/src/validator.rs:13:pub fn validate_tree(root: &Hash, ctx: &Context) -> Result<(), SmtError> {
crates/amun-smt/src/validator.rs:18:    validate_node(root, 0, ctx, &mut path)
crates/amun-smt/src/validator.rs:21:fn validate_node(
crates/amun-smt/src/validator.rs:69:                validate_node(left, next, ctx, path)?;
crates/amun-smt/src/validator.rs:70:                validate_node(right, next, ctx, path)?;
crates/amun-smt/tests/fuzz.rs:60:            let _ = validate_tree(&t.internal_root(), t.context());
crates/amun-smt/tests/validator.rs:11:fn validate_after_inserts() {
crates/amun-smt/tests/validator.rs:18:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-smt/tests/validator.rs:22:fn validate_after_delete() {
crates/amun-smt/tests/validator.rs:29:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-smt/tests/validator.rs:5:fn validate_empty_tree() {
crates/amun-smt/tests/validator.rs:7:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-snapshot-constitution/src/import.rs:19:    let recomputed_root = StateMachine::compute_root(
crates/amun-snapshot-engine/src/byzantine_sync.rs:118:    pub fn verify_peer_identity(&self, peer: &PeerManifest) -> bool {
crates/amun-snapshot-engine/src/chunk.rs:118:    fn compute_merkle_root(chunks: &[SnapshotChunk]) -> [u8; 32] {
crates/amun-snapshot-engine/src/chunk.rs:148:        Self::compute_merkle_root(&self.chunks) == self.chunk_root
crates/amun-snapshot-engine/src/chunk.rs:24:        let chunk_hash = Self::compute_hash(index, &nodes);
crates/amun-snapshot-engine/src/chunk.rs:34:    fn compute_hash(index: u64, nodes: &[SerializedNode]) -> [u8; 32] {
crates/amun-snapshot-engine/src/chunk.rs:46:        Self::compute_hash(self.index, &self.nodes) == self.chunk_hash
crates/amun-snapshot-engine/src/chunk.rs:94:        let chunk_root = ChunkIndex::compute_merkle_root(&self.chunks);
crates/amun-snapshot-engine/src/constitutional_identity.rs:34:        id.identity_hash = id.compute_identity_hash();
crates/amun-snapshot-engine/src/constitutional_identity.rs:38:    fn compute_identity_hash(&self) -> [u8; 32] {
crates/amun-snapshot-engine/src/constitutional_identity.rs:53:        self.compute_identity_hash() == self.identity_hash
crates/amun-snapshot-engine/src/manifest.rs:61:        m.manifest_hash = m.compute_self_hash();
crates/amun-snapshot-engine/src/manifest.rs:65:    fn compute_self_hash(&self) -> [u8; 32] {
crates/amun-snapshot-engine/src/manifest.rs:85:        self.compute_self_hash() == self.manifest_hash
crates/amun-snapshot-engine/src/replay_continuity.rs:30:    pub fn verify_continuity(
crates/amun-snapshot-engine/src/replay_continuity.rs:87:    pub fn verify_roundtrip(chunks: &ChunkIndex, expected_root: [u8; 32]) -> Result<bool, String> {
crates/amun-snapshot-engine/src/structural_verifier.rs:175:    pub fn verify_unique_root(
crates/amun-snapshot-engine/src/verifier.rs:20:    pub fn verify_manifest(manifest: &SnapshotManifest) -> VerificationResult {
crates/amun-snapshot-engine/src/verifier.rs:38:    pub fn verify_chunks(chunks: &ChunkIndex, manifest: &SnapshotManifest) -> VerificationResult {
crates/amun-snapshot-engine/src/verifier.rs:52:    pub fn verify_header(header: &SnapshotHeader) -> VerificationResult {
crates/amun-snapshot-engine/src/verifier.rs:62:    pub fn verify_roundtrip(
crates/amun-snapshot-optimization/src/lib.rs:23:    pub fn apply_to(&self, registry: &mut ResourceRegistry) {
crates/amun-snapshot-optimization/src/lib.rs:34:pub fn compress_snapshot(registry: &ResourceRegistry) -> (Vec<ResourceMetadata>, [u8; 32]) {
crates/amun-snapshot-optimization/src/lib.rs:40:    let state_root = registry.compute_state_root();
crates/amun-snapshot-optimization/src/lib.rs:44:pub fn restore_from_compressed(compressed: &[ResourceMetadata]) -> ResourceRegistry {
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:41:    let base_root = reg.compute_state_root();
crates/amun-soak-full/src/lib.rs:130:                    let pool_id_bytes = amun_defi_core::DefiPool::compute_pool_id(token_a, token_b);
crates/amun-soak-full/src/lib.rs:190:                let root = self.registry.compute_state_root();
crates/amun-soak-full/src/lib.rs:205:                        let root1 = self.registry.compute_state_root();
crates/amun-soak-full/src/lib.rs:206:                        let root2 = self.registry.compute_state_root();
crates/amun-soak-test/src/lib.rs:124:                    let _root = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:141:                        let root1 = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:142:                        let root2 = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:160:            state_root: self.registry.lock().unwrap().compute_state_root(),
crates/amun-soak-test/src/lib.rs:94:                    let pool_id = amun_defi_core::DefiPool::compute_pool_id(token_a, token_b);
crates/amun-state-machine/src/axioms.rs:66:    pub fn verify_all() -> Vec<AxiomVerification> {
crates/amun-state-machine/src/delta_algebra.rs:66:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/engine.rs:121:    pub fn prove_transition(
crates/amun-state-machine/src/engine.rs:65:        Precondition::verify_all(from, transition_type, preconditions)?;
crates/amun-state-machine/src/engine.rs:81:        if let Err(e) = transition.verify_monotonicity() {
crates/amun-state-machine/src/fork_merge.rs:123:        d.declaration_hash = d.compute_hash();
crates/amun-state-machine/src/fork_merge.rs:127:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/fork_merge.rs:48:        d.declaration_hash = d.compute_hash();
crates/amun-state-machine/src/fork_merge.rs:52:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/meta_amendment.rs:45:    pub fn validate_amendment_scope(
crates/amun-state-machine/src/preconditions.rs:23:    pub fn verify_legal(
crates/amun-state-machine/src/preconditions.rs:38:    pub fn verify_all(
crates/amun-state-machine/src/preconditions.rs:46:                Precondition::LegalTransition { .. } => Self::verify_legal(from, transition),
crates/amun-state-machine/src/reconciliation.rs:40:    pub fn verify_reconciliation(
crates/amun-state-machine/src/replay_log.rs:28:        e.entry_hash = e.compute_hash();
crates/amun-state-machine/src/replay_log.rs:32:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/replay_log.rs:82:    pub fn verify_dag(&self) -> bool {
crates/amun-state-machine/src/states.rs:66:        s.state_hash = s.compute_hash();
crates/amun-state-machine/src/states.rs:70:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/states.rs:87:        self.compute_hash() == self.state_hash
crates/amun-state-machine/src/transitions.rs:160:        t.transition_id = t.compute_id();
crates/amun-state-machine/src/transitions.rs:164:    fn compute_id(&self) -> [u8; 32] {
crates/amun-state-machine/src/transitions.rs:183:    pub fn verify_monotonicity(&self) -> Result<(), String> {
crates/amun-state-machine/src/verifier.rs:21:    pub fn verify_transition(
crates/amun-state-machine/src/verifier.rs:46:        if let Err(errors) = Precondition::verify_all(from, transition_type, preconditions) {
crates/amun-state-machine/src/verifier.rs:63:        if let Err(e) = transition.verify_monotonicity() {
crates/amun-state-machine/src/verifier.rs:89:    pub fn verify_invariants(
crates/amun-state-pruning/src/lib.rs:56:    pub fn compute_pruned_root(&self) -> [u8; 32] {
crates/amun-state-pruning/src/lib.rs:62:        hasher.update(self.registry.compute_state_root());
crates/amun-state-pruning/tests/n166_pruning_tests.rs:152:    assert_eq!(pr1.compute_pruned_root(), pr2.compute_pruned_root());
crates/amun-state-root/src/continuity.rs:6:    pub fn verify_link(parent_hash: &[u8; 32], child: &ConstitutionalSnapshot) -> bool {
crates/amun-state-root/src/snapshot.rs:68:            self.verify_constitutional_integrity(),
crates/amun-state-root/src/snapshot.rs:75:    pub fn verify_constitutional_integrity(&self) -> bool {
crates/amun-state-root/src/snapshot.rs:76:        self.execution_transcript.verify_transcript().is_ok()
crates/amun-state-sync/src/lib.rs:120:        let (root, proofs) = build_chunk_merkle_tree(&chunks);
crates/amun-state-sync/src/lib.rs:136:        let state_root = reg.compute_state_root();
crates/amun-state-sync/src/lib.rs:149:        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
crates/amun-state-sync/src/lib.rs:206:        let state_root = reg.compute_state_root();
crates/amun-state-sync/src/lib.rs:213:        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
crates/amun-state-sync/src/lib.rs:242:        let state_root = reg.compute_state_root();
crates/amun-state-sync/src/lib.rs:252:        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
crates/amun-state-sync/src/snapshot_certificate.rs:45:        cert.certificate_hash = cert.compute_hash();
crates/amun-state-sync/src/snapshot_certificate.rs:49:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-sync/src/snapshot_certificate.rs:68:        self.certificate_hash == self.compute_hash()
crates/amun-state-sync/src/state_chunk.rs:120:fn compute_merkle_siblings(leaves: &[[u8; 32]], target_idx: usize) -> Vec<([u8; 32], bool)> {
crates/amun-state-sync/src/state_chunk.rs:13:        let chunk_hash = Self::compute_chunk_hash(chunk_id, &resources);
crates/amun-state-sync/src/state_chunk.rs:21:    pub fn compute_chunk_hash(chunk_id: u32, resources: &[ResourceMetadata]) -> [u8; 32] {
crates/amun-state-sync/src/state_chunk.rs:39:        self.chunk_hash == Self::compute_chunk_hash(self.chunk_id, &self.resources)
crates/amun-state-sync/src/state_chunk.rs:75:pub fn build_chunk_merkle_tree(chunks: &[StateChunk]) -> ([u8; 32], Vec<ChunkMerkleProof>) {
crates/amun-state-sync/src/state_chunk.rs:80:    let chunk_root = compute_merkle_root(&leaf_hashes);
crates/amun-state-sync/src/state_chunk.rs:83:        let siblings = compute_merkle_siblings(&leaf_hashes, i);
crates/amun-state-sync/src/state_chunk.rs:94:fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
crates/amun-state-sync/src/stateless_verifier.rs:91:        let reconstructed_root = temp_reg.compute_state_root();
crates/amun-state-sync/src/sync_package.rs:27:        pkg.package_hash = pkg.compute_hash();
crates/amun-state-sync/src/sync_package.rs:31:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-sync/src/sync_package.rs:46:        self.package_hash == self.compute_hash()
crates/amun-state-sync/src/sync_protocol.rs:120:        let (chunk_root, chunk_proofs) = crate::state_chunk::build_chunk_merkle_tree(&chunks);
crates/amun-state-sync/src/sync_protocol.rs:155:                if registry.compute_state_root() != state_root {
crates/amun-state-sync/src/sync_protocol.rs:174:                state_root: registry.compute_state_root(),
crates/amun-state-sync/src/sync_protocol.rs:196:            if registry.compute_state_root() != block.state_root {
crates/amun-state-sync/src/sync_protocol.rs:257:    fn create_test_registry(n: u64) -> ResourceRegistry {
crates/amun-state-sync/src/sync_protocol.rs:268:        let root = reg.compute_state_root();
crates/amun-state-sync/src/sync_protocol.rs:273:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:279:        let root = reg.compute_state_root();
crates/amun-state-sync/src/sync_protocol.rs:283:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:353:        let root = reg.compute_state_root();
crates/amun-state-sync/src/sync_protocol.rs:379:        assert_eq!(reg.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:384:// to avoid this. N65.1 will fix compute_merkle_siblings to handle odd leaves.
crates/amun-state-sync/src/sync_protocol.rs:67:        let state_root = registry.compute_state_root();
crates/amun-state-sync/src/sync_protocol.rs:74:        let (chunk_root, _proofs) = crate::state_chunk::build_chunk_merkle_tree(&chunks);
crates/amun-state-transition/src/journal.rs:84:    pub fn verify_continuity(&self) -> bool {
crates/amun-state-transition/src/state.rs:118:            Self::compute_root_with_overlay(state, execution_version, epoch_seal_hash, &overlay);
crates/amun-state-transition/src/state.rs:134:        Self::execute_transition(
crates/amun-state-transition/src/state.rs:144:    pub fn compute_root(
crates/amun-state-transition/src/state.rs:161:    pub fn compute_root_with_overlay(
crates/amun-state-transition/src/state.rs:86:    pub fn execute_transition(
crates/amun-state-transition/src/state.rs:94:        let from_root = Self::compute_root(state, execution_version, epoch_seal_hash);
crates/amun-state-types/src/tests.rs:11:fn test_state_verify_success() {
crates/amun-state-types/src/tests.rs:23:fn test_state_verify_failure() {
crates/amun-stateless-sync/src/lib.rs:118:    pub fn verify_height(&self, height: u64) -> Result<(), String> {
crates/amun-stateless-sync/src/lib.rs:132:        amun_constitutional_block::verify_light_client_proof(header, &bundle.certificate, proof)
crates/amun-stateless-sync/src/lib.rs:135:    pub fn verify_chain(&self) -> Result<(), String> {
crates/amun-stateless-sync/src/lib.rs:137:            self.verify_height(h)?;
crates/amun-stateless-sync/src/lib.rs:182:        node.verify_chain()?;
crates/amun-stateless-sync/src/lib.rs:203:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-stateless-sync/src/lib.rs:268:        assert!(node.verify_height(0).is_ok());
crates/amun-stateless-sync/src/lib.rs:272:    fn n11d_stateless_node_verify_chain() {
crates/amun-stateless-sync/src/lib.rs:285:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:296:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:311:        assert!(node.verify_height(5).is_err());
crates/amun-stf/src/nonce.rs:27:    pub fn validate_nonce<S: StateStore>(
crates/amun-stf/src/state.rs:114:        Ok(amun_merkle::MerkleTree::compute_root(&leaves))
crates/amun-stf/src/stf.rs:50:        self.state_root = self.compute_root();
crates/amun-stf/src/stf.rs:62:    fn compute_root(&self) -> PublicHash32 {
crates/amun-stf/src/transition_result.rs:14:pub fn execute_transition_with_receipt(
crates/amun-stf/src/transition_result.rs:17:    execute_logic: impl FnOnce(&[u8]) -> TransitionExecutionResult,
crates/amun-stf/src/transition_result.rs:29:    let result = execute_logic(pre_state);
crates/amun-stf/src/transition_result.rs:52:    receipt.verify_consistency().map_err(alloc::string::String::from)?;
crates/amun-stf/tests/integration_test.rs:10:    let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:27:    assert!(receipt.verify_consistency().is_ok());
crates/amun-stf/tests/integration_test.rs:2:use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};
crates/amun-stf/tests/integration_test.rs:41:    let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:68:        let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:94:    assert!(transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:11:fn execute_block(txs: &[[u8; 32]]) -> (ExecutionTranscript, Vec<u8>, [u8; 32]) {
crates/amun-stf/tests/replay_equivalence.rs:17:        let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/replay_equivalence.rs:51:    let (live_transcript, live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:52:    let (replay_transcript, replay_state, replay_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:5:use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};
crates/amun-stf/tests/replay_equivalence.rs:83:    assert!(live_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:84:    assert!(replay_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:91:    let (live_transcript, _live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:98:        let receipt = execute_transition_with_receipt(
crates/amun-storage-kernel/src/lib.rs:123:    pub fn verify_epoch_transition(
crates/amun-storage-kernel/src/lib.rs:136:        Self::verify_lineage_continuity(prev, next)
crates/amun-storage-kernel/src/lib.rs:94:    pub fn verify_lineage_continuity(
crates/amun-storage-kernel/src/persistence/wal/entry.rs:34:        let entry_hash = Self::compute_hash(
crates/amun-storage-kernel/src/persistence/wal/entry.rs:61:    pub fn verify_chain(&self, expected_prev: &[u8; 32]) -> bool {
crates/amun-storage-kernel/src/persistence/wal/entry.rs:65:        let computed = Self::compute_hash(
crates/amun-storage-kernel/src/persistence/wal/entry.rs:80:    fn compute_hash(
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:71:        if !entry.verify_chain(&self.last_entry_hash) {
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:85:    pub fn verify_full_replay(wal_path: &str) -> Result<([u8; 32], u64), String> {
crates/amun-storage-kernel/src/smt/tree.rs:128:                    return self.build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:171:    fn build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:198:        let child = self.build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:23:    fn build_empty_ladder() -> Vec<NodeHash> {
crates/amun-storage-kernel/src/smt/tree.rs:36:        let empty_ladder = Arc::new(Self::build_empty_ladder());
crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/terminal_empty.rs:9:        // Verified by build_empty_ladder which sets ladder[256] = NodeHash::ZERO.
crates/amun-survival-console/src/dashboard.rs:49:        let truth_ok = self.truth_engine.compute_chain_root(0).is_ok();
crates/amun-sync/src/bin/bootstrap.rs:1:use amun_chain_store::snapshot::{create_snapshot, restore_snapshot, verify_snapshot};
crates/amun-sync/src/bin/bootstrap.rs:38:    verify_snapshot(&snap_dir).expect("Snapshot verification failed");
crates/amun-testnet-sim/tests/adversarial_tests.rs:117:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:124:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:127:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:148:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:149:    let state_root_before = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:156:    assert_eq!(recovered_reg.compute_state_root(), state_root_before);
crates/amun-testnet-sim/tests/adversarial_tests.rs:167:    let initial_reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:185:            pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:207:                let mut fresh = build_registry(10); // Same genesis state
crates/amun-testnet-sim/tests/adversarial_tests.rs:22:fn build_registry(count: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:232:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:239:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:242:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:263:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:264:    let state_root = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:270:    assert_eq!(result.unwrap().compute_state_root(), state_root);
crates/amun-testnet-sim/tests/adversarial_tests.rs:45:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:52:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:55:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:88:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:95:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:98:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-tokenomics-ledger/src/lib.rs:155:    pub fn compute_economic_root(&self) -> [u8; 32] {
crates/amun-tokenomics-ledger/src/lib.rs:170:    pub fn compute_ledger_root(&self) -> [u8; 32] {
crates/amun-tokenomics-ledger/src/lib.rs:171:        self.compute_economic_root()
crates/amun-tokenomics-ledger/src/lib.rs:234:        let root1 = ledger.compute_economic_root();
crates/amun-tokenomics-ledger/src/lib.rs:236:        let root2 = ledger.compute_economic_root();
crates/amun-tokenomics-ledger/tests/test_ledger.rs:57:    assert_eq!(l1.compute_ledger_root(), l2.compute_ledger_root());
crates/amun-tokenomics/src/lib.rs:36:    pub fn compute_epoch_rewards(total_supply: u64) -> u64 {
crates/amun-tokenomics/src/lib.rs:40:    pub fn compute_distribution(reward: u64) -> (u64, u64, u64) {
crates/amun-tokenomics/src/lib.rs:53:        let (treasury, validators, ecosystem) = Self::compute_distribution(total_new_ntr);
crates/amun-tokenomics/tests/test_tokenomics.rs:19:    let (treasury, validators, ecosystem) = EpochEconomics::compute_distribution(reward);
crates/amun-tokenomics/tests/test_tokenomics.rs:8:    let reward = EpochEconomics::compute_epoch_rewards(total_supply);
crates/amun-transaction/src/tests.rs:110:    assert!(r.expect("test invariant").validate_basic().is_err());
crates/amun-transaction/src/tests.rs:49:    assert!(tx.validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:64:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:79:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:94:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tx.rs:160:    pub fn validate_basic(&self) -> AmunResult<()> {
crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
crates/amun-transcript-semantics/src/lib.rs:14:    pub fn verify_causal_chain(&self, parent: &EventIdentity) -> bool { self.causal_parent == parent.event_hash && self.transcript_position == parent.transcript_position + 1 }
crates/amun-transcript-semantics/src/lib.rs:73:    pub fn prove_equivalence(a: &Self, b: &Self) -> bool { a.inner.transcript_root == b.inner.transcript_root && a.inner.state_root == b.inner.state_root && a.inner.receipt_root == b.inner.receipt_root && a.inner.ordering_root == b.inner.ordering_root && a.inner.domain == b.inner.domain }
crates/amun-transcript-semantics/src/lib.rs:96:    pub fn causal_integrity(parent: &EventIdentity, child: &EventIdentity) -> Result<(), TranscriptError> { if !child.verify_causal_chain(parent) { Err(TranscriptError::CausalChainBroken { parent_hash: parent.event_hash, child_parent_hash: child.causal_parent }) } else { Ok(()) } }
crates/amun-transition-proof/src/proof_builder.rs:114:        proof.proof_hash = Self::compute_enhanced_hash(&proof);
crates/amun-transition-proof/src/proof_builder.rs:118:    fn build_merkle_proof(
crates/amun-transition-proof/src/proof_builder.rs:123:        let siblings = Self::compute_siblings(registry, rid);
crates/amun-transition-proof/src/proof_builder.rs:131:    fn compute_siblings(registry: &ResourceRegistry, rid: &ResourceId) -> Vec<([u8; 32], bool)> {
crates/amun-transition-proof/src/proof_builder.rs:178:    fn build_lineage_proof(registry: &ResourceRegistry, rid: &ResourceId) -> LineageProof {
crates/amun-transition-proof/src/proof_builder.rs:203:    fn compute_enhanced_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
crates/amun-transition-proof/src/proof_builder.rs:54:    pub fn build_enhanced(
crates/amun-transition-proof/src/proof_builder.rs:86:                let mp = Self::build_merkle_proof(registry, rid, pre_state_root);
crates/amun-transition-proof/src/proof_builder.rs:88:                let lp = Self::build_lineage_proof(registry, rid);
crates/amun-transition-proof/src/transition_proof.rs:100:    pub fn verify_state_transition(&self) -> bool {
crates/amun-transition-proof/src/transition_proof.rs:57:        proof.proof_hash = proof.compute_hash();
crates/amun-transition-proof/src/transition_proof.rs:63:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-transition-proof/src/transition_proof.rs:93:    pub fn verify_integrity(&self) -> bool {
crates/amun-transition-proof/src/transition_proof.rs:94:        self.proof_hash == self.compute_hash()
crates/amun-truth-engine/src/engine.rs:102:        let (receipt, overlay) = StateMachine::execute_transition(
crates/amun-truth-engine/src/engine.rs:132:            self.compute_chain_root_until(self.current_position)
crates/amun-truth-engine/src/engine.rs:165:                let (receipt, overlay) = StateMachine::execute_transition(
crates/amun-truth-engine/src/engine.rs:198:                        .compute_chain_root_until(self.current_position)
crates/amun-truth-engine/src/engine.rs:248:    pub fn compute_chain_root_until(&self, until: ChainPosition) -> Result<[u8; 32], ReplayError> {
crates/amun-truth-engine/src/engine.rs:276:    pub fn compute_chain_root(&self, target_tx_count: u64) -> Result<[u8; 32], ReplayError> {
crates/amun-truth-engine/src/engine.rs:96:    pub fn execute_live(
crates/amun-unlock-law/src/lib.rs:13:pub fn verify_unlock_condition(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
crates/amun-unlock-law/src/lib.rs:81:        assert!(verify_unlock_condition(&qc, &set));
crates/amun-unsigned-messages/src/proposal.rs:155:    pub fn verify_unsigned(&self) -> bool {
crates/amun-unsigned-messages/src/proposal.rs:156:        self.unsigned.verify_hash()
crates/amun-unsigned-messages/src/proposal.rs:53:    pub fn verify_hash(&self) -> bool {
crates/amun-unsigned-messages/src/vote.rs:149:    pub fn verify_unsigned(&self) -> bool {
crates/amun-unsigned-messages/src/vote.rs:150:        self.unsigned.verify_hash()
crates/amun-unsigned-messages/src/vote.rs:51:    pub fn verify_hash(&self) -> bool {
crates/amun-validator-identity/src/lib.rs:6:pub use signature::{verify_ed25519, vote_signing_payload};
crates/amun-validator-identity/src/signature.rs:32:pub fn verify_ed25519(pk: &[u8; 32], payload: &[u8], signature: &[u8; 64]) -> bool {
crates/amun-validator-identity/src/signature.rs:55:    fn n105_sign_and_verify_vote() {
crates/amun-validator-identity/src/signature.rs:68:        assert!(verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-identity/src/signature.rs:97:        assert!(!verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-networking/src/lib.rs:111:        assert_eq!(imported_reg.compute_state_root(), state_root);
crates/amun-validator-networking/src/lib.rs:117:        let source_reg = build_registry(10);
crates/amun-validator-networking/src/lib.rs:130:        let source_reg = build_registry(100);
crates/amun-validator-networking/src/lib.rs:131:        let _state_root = source_reg.compute_state_root();
crates/amun-validator-networking/src/lib.rs:155:        let source_reg = build_registry(10);
crates/amun-validator-networking/src/lib.rs:58:    fn build_registry(count: u8) -> ResourceRegistry {
crates/amun-validator-networking/src/lib.rs:96:        let source_reg = build_registry(50);
crates/amun-validator-networking/src/lib.rs:97:        let state_root = source_reg.compute_state_root();
crates/amun-validator-networking/src/sync_transport.rs:18:        let state_root = registry.compute_state_root();
crates/amun-validator-networking/src/sync_transport.rs:27:        let (chunk_root, chunk_proofs) = build_chunk_merkle_tree(&chunks);
crates/amun-validator-networking/src/sync_transport.rs:3:use amun_state_sync::state_chunk::{build_chunk_merkle_tree, StateChunk};
crates/amun-validator-networking/src/sync_transport.rs:60:                let imported_root = registry.compute_state_root();
crates/amun-verification-kernel/src/lib.rs:104:    fn compute_hash(&self) -> String {
crates/amun-verification-kernel/src/lib.rs:120:        self.certificate_id == self.compute_id() && self.certificate_hash == self.compute_hash()
crates/amun-verification-kernel/src/lib.rs:185:    fn n46_5_issue_certificate() {
crates/amun-verification-kernel/src/lib.rs:87:        cert.certificate_id = cert.compute_id();
crates/amun-verification-kernel/src/lib.rs:88:        cert.certificate_hash = cert.compute_hash();
crates/amun-verification-kernel/src/lib.rs:92:    fn compute_id(&self) -> String {
crates/amun-verified-pipeline/src/vote_pipeline.rs:61:        if !self.signed.verify_unsigned() {
crates/amun-verifier-node/src/verifier.rs:15:        report.block_valid = block.compute_hash() == block.block_hash;
crates/amun-verifier-node/src/verifier.rs:51:            if ProofVerifier::verify_receipt(receipt).is_err() {
crates/amun-verifier-node/src/verifier.rs:60:    pub fn verify_chain(
crates/amun-verifier-node/src/verifier.rs:71:            reports.push(Self::verify_block(block, receipts, prev));
crates/amun-verifier-node/src/verifier.rs:8:    pub fn verify_block(
crates/amun-vm-kernel/src/vm_kernel.rs:130:        let post_state_root = registry.compute_state_root();
crates/amun-vm-kernel/src/vm_kernel.rs:30:    pub fn verify(buffer: &mut PendingBuffer, registry: &ResourceRegistry) -> bool {
crates/amun-wal/src/authority_index.rs:22:    pub fn build_from_entries(entries: &[super::WALEntry]) -> Self {
crates/amun-wal/src/authority_index.rs:47:    pub fn verify_snapshot_anchor(
crates/amun-wal/src/lib.rs:149:        let _auth = wal.validate_authority_chain()?;
crates/amun-wal/src/lib.rs:604:    pub fn verify_chain_continuity(entries: &[WALEntry]) -> Result<(), String> {
crates/amun-wal/src/lib.rs:748:    pub fn validate_authority_chain(&self) -> Result<AuthorityValidation, String> {
crates/amun-wal/src/lib.rs:848:    pub fn build_manifest(&self) -> WALManifest {
crates/amun-wal/src/wal.rs:132:            if !Self::validate_header(&header) { break; }
crates/amun-wal/src/wal.rs:40:    fn validate_header(header: &[u8; FRAME_HEADER_SIZE]) -> bool {
crates/amun-wal/src/wal.rs:55:            if !Self::validate_header(&header) { break; }
crates/amun-wallet-api/src/lib.rs:29:        let app = server::build_app();
crates/amun-wallet-api/src/lib.rs:44:        let app = server::build_app();
crates/amun-wallet-api/src/lib.rs:58:    async fn n48_3_build_transaction() {
crates/amun-wallet-api/src/lib.rs:59:        let app = server::build_app();
crates/amun-wallet-api/src/lib.rs:76:    async fn n48_3_build_transaction_invalid() {
crates/amun-wallet-api/src/lib.rs:77:        let app = server::build_app();
crates/amun-wallet-api/src/main.rs:10:    let app = server::build_app();
crates/amun-wallet-api/src/routes/transactions.rs:27:        .route("/build", post(build_transaction))
crates/amun-wallet-api/src/routes/transactions.rs:5:async fn build_transaction(
crates/amun-wallet-api/src/routes/transactions.rs:8:    TransactionService::build_transaction(req)
crates/amun-wallet-api/src/server.rs:4:pub fn build_app() -> Router {
crates/amun-wallet-api/src/services/transaction_service.rs:12:    pub fn build_transaction(req: BuildTransactionRequest) -> ApiResult<BuildTransactionResponse> {
crates/amun-wallet-management/src/lib.rs:57:    fn key07_sign_and_verify_message() {
crates/amun-wallet-management/src/lib.rs:61:        assert!(signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:72:        assert!(!signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:80:        assert!(signer::verify_signature(&kp.public_key, tx_bytes, &sig));
crates/amun-wallet-management/src/signer.rs:13:pub fn verify_signature(public_key: &[u8; 32], message: &[u8], signature: &[u8]) -> bool {
crates/amun_state_machine/src/certification/mod.rs:107:pub fn compute_execution_fingerprint() -> [u8; 32] {
crates/amun_state_machine/src/lib.rs:24:pub use certification::{ExecutionCertificate, ReplayCertificate, compute_execution_fingerprint};
crates/amun_state_machine/src/log.rs:57:        self.recompute_root();
crates/amun_state_machine/src/log.rs:60:    pub fn recompute_root(&mut self) {
crates/amun_state_machine/src/receipt/mod.rs:106:        self.recompute_accumulator();
crates/amun_state_machine/src/receipt/mod.rs:109:    pub fn recompute_accumulator(&mut self) {
crates/amun_state_machine/src/scheduler.rs:104:        scheduler1.execute_all(&mut state1);
crates/amun_state_machine/src/scheduler.rs:112:        scheduler2.execute_all(&mut state2);
crates/amun_state_machine/src/scheduler.rs:33:    pub fn execute_next(&mut self, state: &mut ConstitutionalState) -> Option<[u8; 32]> {
crates/amun_state_machine/src/scheduler.rs:49:    pub fn execute_all(&mut self, state: &mut ConstitutionalState) -> usize {
crates/amun_state_machine/src/scheduler.rs:51:        while self.execute_next(state).is_some() {
crates/amun_state_machine/src/snapshot.rs:104:        assert!(snap2.verify_chain(&snap1));
crates/amun_state_machine/src/snapshot.rs:105:        assert!(manager.verify_all());
crates/amun_state_machine/src/snapshot.rs:35:    pub fn verify_chain(&self, previous: &StateSnapshot) -> bool {
crates/amun_state_machine/src/snapshot.rs:67:    pub fn verify_all(&self) -> bool {
crates/amun_state_machine/src/snapshot.rs:69:            if !self.snapshots[i].verify_chain(&self.snapshots[i - 1]) {
crates/amun_state_machine/src/state.rs:56:        state.recompute_hash();
crates/amun_state_machine/src/state.rs:63:        self.recompute_hash();
crates/amun_state_machine/src/state.rs:75:    pub fn recompute_hash(&mut self) {
crates/amun_state_machine/src/transition.rs:122:        let amount = match validate_amount(event.amount) {
crates/amun_state_machine/src/transition.rs:140:        let amount = match validate_amount(event.amount) {
crates/amun_state_machine/src/transition.rs:163:        let amount = match validate_amount(event.amount) {
crates/amun_state_machine/src/transition.rs:216:        let amount = match validate_amount(event.amount) {
crates/amun_state_machine/src/transition.rs:237:        let amount = match validate_amount(event.amount) {
crates/amun_state_machine/src/transition.rs:42:fn validate_amount(amount_raw: i64) -> Result<Fixed, ErrorCode> {
crates/amun_state_machine/src/transition.rs:70:            state.recompute_hash();
crates/amun_state_machine/src/transition.rs:93:        let amount = match validate_amount(event.amount) {

## Tests
crates/amun-bench/tests/n161_state_root_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n161_state_root_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-bench/tests/n162_snapshot_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n162_snapshot_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-benchmarks/benches/sync_bench.rs:5:fn create_test_registry(size: u64) -> ResourceRegistry {
crates/amun-byzantine-tests/tests/attack_suite.rs:107:    let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:11:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-byzantine-tests/tests/attack_suite.rs:126:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:137:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:154:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:182:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:193:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:210:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:224:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:245:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:55:    let mut fresh_reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:82:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:94:    let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:39:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:40:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:53:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:54:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:74:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:75:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:33:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:5:use amun_resource_core::ResourceRegistry;
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:9:    let mut reg = ResourceRegistry::new(100);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(20000);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-defi-amm/tests/n153_amm_tests.rs:20:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:21:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-amm/tests/n153_amm_tests.rs:37:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:6:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:10:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:30:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:49:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:68:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:88:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:99:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:17:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:25:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:26:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:6:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:103:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:124:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:58:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:84:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:123:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:17:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:64:    let mut reg = ResourceRegistry::new(2000);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:109:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:110:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:35:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:81:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:131:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:57:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:78:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:28:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:46:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:118:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:141:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:165:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:38:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:62:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:91:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:18:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-mining/tests/n133_mining_tests.rs:55:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:17:    ResourceRegistry,
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:22:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:5:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:18:    ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:31:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:107:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:111:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:18:    let mut reg = ResourceRegistry::new(10000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:77:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:78:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:108:    let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:119:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:17:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:18:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:37:    let parent_hash = ResourceRegistry::hash_resource(reg1.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-resource-core/tests/n130_constitutional_nft.rs:115:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:116:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:119:    let setup = |reg: &mut ResourceRegistry| {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:3:    ResourceRegistry, ResourceState,
crates/amun-resource-core/tests/n130_constitutional_nft.rs:45:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:74:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:8:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/stress_tests.rs:107:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:133:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:149:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:174:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:186:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:211:        let parent_hash = ResourceRegistry::hash_resource(parent);
crates/amun-resource-core/tests/stress_tests.rs:222:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:234:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:254:            ResourceRegistry::hash_resource(tip),
crates/amun-resource-core/tests/stress_tests.rs:26:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-resource-core/tests/stress_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-resource-core/tests/stress_tests.rs:45:    let mut reg = ResourceRegistry::new(10_000);
crates/amun-resource-core/tests/stress_tests.rs:58:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:89:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(10000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:69:    let mut reg = ResourceRegistry::new(1000);
crates/amun-soak-test/src/lib.rs:13:    pub registry: Arc<Mutex<ResourceRegistry>>,
crates/amun-soak-test/src/lib.rs:23:            registry: Arc::new(Mutex::new(ResourceRegistry::new(5000))),
crates/amun-soak-test/src/lib.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-state-sync/src/sync_protocol.rs:257:    fn create_test_registry(n: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:22:fn build_registry(count: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:23:    let mut reg = ResourceRegistry::new((count * 2) as usize);
crates/amun-testnet-sim/tests/adversarial_tests.rs:9:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,

## Documentation
docs/audit/SECURITY_INVARIANTS.md:12:| R6 | Version monotonicity | ResourceRegistry::consume_and_derive | WitnessBuilder |
docs/audit/SECURITY_INVARIANTS.md:7:| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
docs/audit/SECURITY_INVARIANTS.md:9:| R3 | Child requires consumed parent | ResourceRegistry::consume_and_derive | WitnessBuilder |
