use std::sync::{Arc, Mutex};

use amun_accounts::AccountStore;
use amun_authority_registry::{transaction::GovernanceState, AuthorityRegistry};
use amun_chain_store::store::ChainStore;
use amun_consensus_network::{engine::ConsensusEngine, CertificateGossip, SlashingLedger};
use amun_constitutional_enforcement::ConstitutionalEnforcementKernel;
use amun_mempool::Mempool;
use amun_tokenomics_ledger::EconomicLedger;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<ChainStore>>,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub mempool: Arc<Mutex<Mempool>>,
    pub faucet: Arc<Mutex<crate::faucet::FaucetState>>,
    pub account_store: Arc<Mutex<AccountStore>>,
    pub governance: Arc<Mutex<GovernanceState>>,
    pub authority_registry: Arc<Mutex<AuthorityRegistry>>,
    pub constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    pub certificate_gossip: Arc<Mutex<CertificateGossip>>,
    pub slashing_ledger: Arc<Mutex<SlashingLedger>>,
    pub economic_ledger: Arc<Mutex<EconomicLedger>>,
    pub previous_evidence_root: Arc<Mutex<[u8; 32]>>,
}
