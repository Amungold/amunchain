use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use amun_authority_registry::transaction::GovernanceState;
use amun_authority_registry::AuthorityRegistry;
use amun_block_builder::BlockBuilder;
use amun_block_store::BlockStore;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::{RealStakingExecutor, StakingAdapter};
use amun_constitutional_enforcement::ConstitutionalEnforcementKernel;
use amun_mempool::Mempool;
use ed25519_dalek::SigningKey;

use crate::config::ValidatorConfig;
use crate::identity::service::IdentityService;

/// RuntimeParts holds all constructed services before they are assembled into LiveValidator.
///
/// ADR-023 Phase 5: Built entirely by LiveValidatorBuilder::build().
pub struct RuntimeParts {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub store: Arc<Mutex<ChainStore>>,
    pub running: Arc<Mutex<bool>>,
    pub signing_key: SigningKey,
    pub validator_id: [u8; 32],
    pub block_store: Arc<Mutex<BlockStore>>,
    pub mempool: Arc<Mutex<Mempool>>,
    pub block_builder: Arc<Mutex<BlockBuilder>>,
    pub governance: Arc<Mutex<GovernanceState>>,
    pub authority_registry: Arc<Mutex<AuthorityRegistry>>,
    pub certificate_gossip: Arc<Mutex<amun_consensus_network::CertificateGossip>>,
    pub staking_adapter: Arc<Mutex<StakingAdapter<RealStakingExecutor>>>,
    pub applied_slashing_certificates: Arc<Mutex<std::collections::HashSet<[u8; 32]>>>,
    pub slashing_ledger: Arc<Mutex<amun_consensus_network::SlashingLedger>>,
    pub constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    pub previous_evidence_root: Arc<Mutex<[u8; 32]>>,
    pub peer_addrs: Vec<SocketAddr>,
}

/// LiveValidatorBuilder constructs all runtime parts from configuration.
///
/// ADR-023 Phase 5: Uses staged construction to allow reuse for different node types.
pub struct LiveValidatorBuilder {
    config: ValidatorConfig,
}

impl LiveValidatorBuilder {
    pub fn new(config: ValidatorConfig) -> Self {
        Self { config }
    }

    /// Build all runtime parts in stages.
    /// Each stage is a separate method for testability and future reuse.
    pub fn build(self) -> Result<RuntimeParts, String> {
        // Stage 1: Storage
        let (store, recovered_height, recovered_root) = self.build_storage()?;

        // Stage 2: Consensus Engine
        let mut engine = self.build_engine(recovered_height, recovered_root);

        // Stage 3: Identity (keys, certificates, registry)
        let (signing_key, validator_id, registry) = self.build_identity(&mut engine)?;

        // Stage 4: Supporting services (mempool, governance, slashing, etc.)
        let running = Arc::new(Mutex::new(false));
        let block_store = Arc::new(Mutex::new(BlockStore::new(&format!(
            "{}/blocks.json",
            self.config.data_dir
        ))));
        let mempool = Arc::new(Mutex::new(Mempool::new()));
        let block_builder = Arc::new(Mutex::new(BlockBuilder::new()));
        let governance = Arc::new(Mutex::new(GovernanceState::new()));
        let authority_registry = Arc::new(Mutex::new(registry));
        let certificate_gossip =
            Arc::new(Mutex::new(amun_consensus_network::CertificateGossip::new()));
        let staking_adapter = Arc::new(Mutex::new(StakingAdapter::new(
            amun_consensus_network::MisbehaviorRegistry::new(
                amun_consensus_network::MisbehaviorThresholds::default(),
            ),
            RealStakingExecutor::new(amun_staking::validator::ValidatorRegistry::new()),
        )));
        let applied_slashing_certificates = Arc::new(Mutex::new(std::collections::HashSet::new()));
        let slashing_ledger = Arc::new(Mutex::new(amun_consensus_network::SlashingLedger::new()));
        let constitutional_kernel = Arc::new(Mutex::new(ConstitutionalEnforcementKernel::new()));
        let previous_evidence_root = Arc::new(Mutex::new([0u8; 32]));

        let peer_addrs: Vec<SocketAddr> = self
            .config
            .other_peers()
            .iter()
            .map(|p| p.address)
            .collect();

        Ok(RuntimeParts {
            block_store,
            config: self.config,
            engine: Arc::new(Mutex::new(engine)),
            store: Arc::new(Mutex::new(store)),
            running,
            signing_key,
            validator_id,
            mempool,
            block_builder,
            governance,
            authority_registry,
            certificate_gossip,
            staking_adapter,
            applied_slashing_certificates,
            slashing_ledger,
            constitutional_kernel,
            previous_evidence_root,
            peer_addrs,
        })
    }

    fn build_storage(&self) -> Result<(ChainStore, u64, [u8; 32]), String> {
        let store = ChainStore::open(&self.config.data_dir)
            .unwrap_or_else(|_| ChainStore::open("/tmp/amun-fallback").unwrap());
        let recovered_height = store.latest_height();
        let recovered_root = store
            .load_tip()
            .map(|r| r.history_root)
            .unwrap_or([0u8; 32]);
        Ok((store, recovered_height, recovered_root))
    }

    fn build_engine(&self, recovered_height: u64, recovered_root: [u8; 32]) -> ConsensusEngine {
        let mut engine =
            ConsensusEngine::new(self.config.validator_id, self.config.total_validators());
        if recovered_height > 0 {
            engine.current_height = recovered_height;
            engine.history_root = recovered_root;
        }
        engine
    }

    fn build_identity(
        &self,
        engine: &mut ConsensusEngine,
    ) -> Result<(SigningKey, [u8; 32], AuthorityRegistry), String> {
        let (signing_key, validator_id, registry) =
            IdentityService::initialize(&self.config, engine);
        Ok((signing_key, validator_id, registry))
    }
}
