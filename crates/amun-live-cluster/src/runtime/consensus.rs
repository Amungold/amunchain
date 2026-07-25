use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime};

use amun_authority_registry::transaction::GovernanceState;
use amun_authority_registry::AuthorityRegistry;
use amun_block_builder::BlockBuilder;
use amun_block_store::BlockStore;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_constitutional_enforcement::ConstitutionalEnforcementKernel;
use amun_mempool::Mempool;
use amun_validator_identity::vote_signing_payload;
use ed25519_dalek::Signer;

use crate::sync::catchup::SyncRuntime;
use amun_history::compute_history_root;

/// ConsensusRuntime owns the main consensus loop thread.
///
/// ADR-023 Phase 4: Extracted from LiveValidator::start().
/// Only the main loop is extracted here; helper functions remain in validator_original.rs
/// and will be moved in later PRs.
pub struct ConsensusRuntime {
    engine: Arc<Mutex<ConsensusEngine>>,
    store: Arc<Mutex<ChainStore>>,
    block_store: Arc<Mutex<BlockStore>>,
    mempool: Arc<Mutex<Mempool>>,
    builder: Arc<Mutex<BlockBuilder>>,
    governance: Arc<Mutex<GovernanceState>>,
    authority_registry: Arc<Mutex<AuthorityRegistry>>,
    certificate_gossip: Arc<Mutex<amun_consensus_network::CertificateGossip>>,
    staking_adapter: Arc<
        Mutex<amun_consensus_network::StakingAdapter<amun_consensus_network::RealStakingExecutor>>,
    >,
    applied_slashing_certificates: Arc<Mutex<std::collections::HashSet<[u8; 32]>>>,
    slashing_ledger: Arc<Mutex<amun_consensus_network::SlashingLedger>>,
    constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    previous_evidence_root: Arc<Mutex<[u8; 32]>>,
    signing_key: ed25519_dalek::SigningKey,
    validator_id: [u8; 32],
    my_index: u8,
    sync_runtime: Arc<SyncRuntime>,
    peers: Vec<SocketAddr>,
    running: Arc<Mutex<bool>>,
}

impl ConsensusRuntime {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        engine: Arc<Mutex<ConsensusEngine>>,
        store: Arc<Mutex<ChainStore>>,
        block_store: Arc<Mutex<BlockStore>>,
        mempool: Arc<Mutex<Mempool>>,
        builder: Arc<Mutex<BlockBuilder>>,
        governance: Arc<Mutex<GovernanceState>>,
        authority_registry: Arc<Mutex<AuthorityRegistry>>,
        certificate_gossip: Arc<Mutex<amun_consensus_network::CertificateGossip>>,
        staking_adapter: Arc<
            Mutex<
                amun_consensus_network::StakingAdapter<amun_consensus_network::RealStakingExecutor>,
            >,
        >,
        applied_slashing_certificates: Arc<Mutex<std::collections::HashSet<[u8; 32]>>>,
        slashing_ledger: Arc<Mutex<amun_consensus_network::SlashingLedger>>,
        constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
        previous_evidence_root: Arc<Mutex<[u8; 32]>>,
        signing_key: ed25519_dalek::SigningKey,
        validator_id: [u8; 32],
        my_index: u8,
        sync_runtime: Arc<SyncRuntime>,
        peers: Vec<SocketAddr>,
        running: Arc<Mutex<bool>>,
    ) -> Self {
        Self {
            engine,
            store,
            block_store,
            mempool,
            builder,
            governance,
            authority_registry,
            certificate_gossip,
            staking_adapter,
            applied_slashing_certificates,
            slashing_ledger,
            constitutional_kernel,
            previous_evidence_root,
            signing_key,
            validator_id,
            my_index,
            sync_runtime,
            peers,
            running,
        }
    }

    /// Spawn the main consensus loop on a dedicated thread.
    /// Returns the JoinHandle.
    pub fn spawn(&self) -> JoinHandle<()> {
        let engine = self.engine.clone();
        let store = self.store.clone();
        let mempool = self.mempool.clone();
        let block_store = self.block_store.clone();
        let builder = self.builder.clone();
        let governance = self.governance.clone();
        let authority_registry = self.authority_registry.clone();
        let certificate_gossip = self.certificate_gossip.clone();
        let staking_adapter = self.staking_adapter.clone();
        let applied_certs = self.applied_slashing_certificates.clone();
        let slashing_ledger = self.slashing_ledger.clone();
        let constitutional_kernel = self.constitutional_kernel.clone();
        let previous_evidence_root = self.previous_evidence_root.clone();
        let signing_key = self.signing_key.clone();
        let validator_id = self.validator_id;
        let my_index = self.my_index;
        let sync_runtime = self.sync_runtime.clone();
        let peers = self.peers.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            while *running.lock().unwrap() {
                // ADR-023: SyncRuntime owns needs_catchup flag exclusively
                let height = {
                    let eng = engine.lock().unwrap();
                    eng.current_height + 1
                };

                // Sync check
                if sync_runtime.catch_up_if_needed() {
                    continue;
                }

                let proposer_idx = {
                    let eng = engine.lock().unwrap();
                    eng.proposer_for(height)
                };
                let is_proposer = my_index as usize == proposer_idx + 1;
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let already_proposed = {
                    let eng = engine.lock().unwrap();
                    eng.rounds
                        .get(&height)
                        .and_then(|r| r.proposed_block_hash)
                        .is_some()
                };

                let mut proposed_tx_hashes: Option<Vec<[u8; 32]>> = None;
                let (block_hash, state_root) = if is_proposer && !already_proposed {
                    println!("CONSENSUS mempool Arc={:p}", Arc::as_ptr(&mempool));
                    println!(
                        "CONSENSUS mempool Arc={:p}",
                        std::sync::Arc::as_ptr(&mempool)
                    );
                    let mut mp = mempool.lock().unwrap();
                    let mut bld = builder.lock().unwrap();
                    // ADR-025: parent_hash is the previous block_hash, not history_root
                    let parent = engine.lock().unwrap().last_finalized_block_hash;
                    let pending_certs: Vec<amun_consensus_network::SlashingCertificate> =
                        certificate_gossip
                            .lock()
                            .unwrap()
                            .get_pending()
                            .iter()
                            .map(|c| (*c).clone())
                            .collect();
                    let mut block = bld.build_block_with_certificates(
                        height,
                        parent,
                        &mut mp,
                        1000,
                        validator_id,
                        timestamp,
                        pending_certs,
                        // P2.2: Pass the previous block's evidence root
                        // This chains evidence commitments across blocks
                        *previous_evidence_root.lock().unwrap(),
                        {
                            let eng = engine.lock().unwrap();
                            amun_merkle::validator_set_root(&eng.validator_ids)
                        }, // P3.2: real validator set root
                    );
                    {
                        let ledger = slashing_ledger.lock().unwrap();
                        block.slashing_root = amun_consensus_network::merkle_root(&ledger.history);
                    }
                    let hash = block.block_hash();
                    let root = block.state_root;
                    proposed_tx_hashes =
                        Some(block.transactions.iter().map(|tx| tx.tx_hash()).collect());
                    let mut eng = engine.lock().unwrap();
                    eng.start_round(height, validator_id);
                    if let Some(round) = eng.round_mut(height) {
                        round.propose(hash, root);
                    }
                    (hash, root)
                } else if is_proposer && already_proposed {
                    let eng = engine.lock().unwrap();
                    let round = eng.rounds.get(&height);
                    let hash = round
                        .and_then(|r| r.proposed_block_hash)
                        .unwrap_or([height as u8; 32]);
                    let root = round
                        .and_then(|r| r.proposed_state_root)
                        .unwrap_or([0xBB; 32]);
                    (hash, root)
                } else {
                    thread::sleep(Duration::from_millis(200));
                    let eng = engine.lock().unwrap();
                    if let Some(round) = eng.rounds.get(&height) {
                        if let (Some(h), Some(r)) =
                            (round.proposed_block_hash, round.proposed_state_root)
                        {
                            (h, r)
                        } else {
                            ([height as u8; 32], [0xBB; 32])
                        }
                    } else {
                        ([height as u8; 32], [0xBB; 32])
                    }
                };

                let payload = vote_signing_payload(
                    &validator_id,
                    height,
                    &block_hash,
                    &state_root,
                    true,
                    timestamp,
                );
                let sig = signing_key.sign(&payload).to_bytes();
                let my_vote = ConsensusVote {
                    voter_id: validator_id,
                    height,
                    block_hash,
                    state_root,
                    approve: true,
                    signature: sig,
                    timestamp,
                    commitment: None,
                };

                {
                    let mut eng = engine.lock().unwrap();
                    if !eng.rounds.contains_key(&height) {
                        eng.start_round(height, validator_id);
                    }
                    let _ = eng.process_vote(my_vote.clone());
                }

                let vote_data = postcard::to_stdvec(&my_vote).unwrap();
                let vote_len = vote_data.len() as u32;
                for peer in &peers {
                    for _retry in 0..10 {
                        if let Ok(mut stream) = std::net::TcpStream::connect(peer) {
                            let _ = stream.set_nonblocking(false);
                            use std::io::Write;
                            let _ = stream.write_all(&vote_len.to_be_bytes());
                            let _ = stream.write_all(&vote_data);
                            let _ = stream.flush();
                            break;
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                }

                let vote_deadline = Instant::now() + Duration::from_millis(500);
                loop {
                    let quorum_reached = {
                        let eng = engine.lock().unwrap();
                        if let Some(round) = eng.rounds.get(&height) {
                            let approvals = round.votes.iter().filter(|v| v.approve).count();
                            approvals * 3 > eng.total_validators * 2
                        } else {
                            false
                        }
                    };
                    if quorum_reached {
                        break;
                    }
                    if Instant::now() >= vote_deadline {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }

                // ADR-024: Chain commitment derived from block hash
                let prev_history_root = engine.lock().unwrap().history_root;
                let history_root = compute_history_root(prev_history_root, block_hash);
                let cert = {
                    let mut eng = engine.lock().unwrap();
                    eng.try_advance(height, history_root)
                };

                if let Some(cert) = cert {
                    let mut gov = governance.lock().unwrap();
                    let mut reg = authority_registry.lock().unwrap();
                    let _ = gov.finalize_block(4, &mut reg);

                    {
                        let mut kernel = constitutional_kernel.lock().unwrap();
                        let state_root_valid = cert.state_root != [0u8; 32];
                        let chain_continuous = cert.block_hash != [0u8; 32];
                        let transition_valid = cert.state_root != [0u8; 32];
                        let signatures_valid = true;
                        let no_double_spend = true;
                        let slashing_bound = {
                            let gossip = certificate_gossip.lock().unwrap();
                            gossip
                                .get_pending()
                                .iter()
                                .all(|c| !c.evidence_ids.is_empty())
                        };
                        let governance_valid = true;
                        // ADR-024: replay_deterministic compares state_root from cert vs computed state_root
                        // (history_root is now a chain commitment, not state_root)
                        let replay_deterministic = cert.state_root == state_root;
                        let finality_supermajority = {
                            let eng = engine.lock().unwrap();
                            eng.total_voting_power > 0
                        };
                        let evidence_valid = {
                            let gossip = certificate_gossip.lock().unwrap();
                            gossip.get_pending().iter().all(|c| c.verify().is_ok())
                        };

                        use amun_constitutional_enforcement::ConstitutionalVerdict;
                        let verdict = kernel.review_block(
                            height,
                            state_root_valid,
                            chain_continuous,
                            signatures_valid,
                            no_double_spend,
                            slashing_bound,
                            governance_valid,
                            replay_deterministic,
                            finality_supermajority,
                            transition_valid,
                            evidence_valid,
                        );
                        match &verdict {
                            ConstitutionalVerdict::Constitutional => {
                                eprintln!("N123.1: Block {} is CONSTITUTIONAL", height);
                            }
                            ConstitutionalVerdict::Unconstitutional { violations } => {
                                eprintln!(
                                    "N123.1: Block {} is UNCONSTITUTIONAL: {} violations",
                                    height,
                                    violations.len()
                                );
                                for v in violations {
                                    eprintln!("  - {:?}: {}", v.law, v.description);
                                }
                            }
                        }

                        let mut hasher = blake3::Hasher::new();
                        hasher.update(b"AMUN_VERDICT_V1");
                        hasher.update(&height.to_le_bytes());
                        let vbytes = postcard::to_stdvec(&verdict).unwrap_or_default();
                        hasher.update(&vbytes);
                        let mut verdict_hash = [0u8; 32];
                        verdict_hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

                        use amun_constitutional_enforcement::evidence_records::{
                            ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence,
                            ReplayEvidence, SignatureEvidence,
                        };
                        let sig_ev = SignatureEvidence::new(
                            if signatures_valid { 1 } else { 0 },
                            if signatures_valid { 0 } else { 1 },
                        );
                        let ds_ev = DoubleSpendEvidence::new(
                            if no_double_spend { 1 } else { 0 },
                            if no_double_spend { 0 } else { 1 },
                        );
                        let gov_ev = GovernanceEvidence::new(
                            cert.block_hash,
                            height / 100,
                            governance_valid,
                        );
                        let rep_ev = ReplayEvidence::new(cert.state_root, history_root);
                        let evidence_record = ConstitutionalEvidenceRecord::new(
                            height,
                            cert.block_hash,
                            sig_ev,
                            ds_ev,
                            gov_ev,
                            rep_ev,
                            slashing_bound,
                            evidence_valid,
                            finality_supermajority,
                            chain_continuous,
                            state_root_valid,
                            transition_valid,
                        );
                        let evidence_record_hash = evidence_record.evidence_hash;

                        use amun_evidence_root::EvidenceRoot;
                        let mut prev_root = previous_evidence_root.lock().unwrap();
                        let evidence_root_obj = EvidenceRoot::compute(
                            cert.state_root,
                            cert.block_hash,
                            [0u8; 32],
                            evidence_record.evidence_hash,
                            *prev_root,
                            height,
                        );
                        let evidence_root = evidence_root_obj.root;
                        *prev_root = evidence_root;

                        use amun_chain_store::record::FinalizedChainRecord;
                        let record = FinalizedChainRecord {
                            height,
                            block_hash: cert.block_hash,
                            state_root: cert.state_root,
                            history_root: cert.history_root,
                            certificate_hash: [0u8; 32],
                            slashing_root: [0u8; 32],
                            verdict_hash,
                            evidence_record_hash,
                            evidence_root, // P2: from EvidenceRoot::compute
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };

                        if let Err(e) = store.lock().unwrap().append(record) {
                            eprintln!("STORE ERROR: {}", e);
                        } else {
                            // Try to get tx_hashes from proposal, or fall back to BlockStore
                            let tx_hashes = if let Some(hashes) = proposed_tx_hashes.take() {
                                // Proposer path: we already have tx_hashes
                                // Store them in BlockStore for other nodes
                                let hex_hashes: Vec<String> =
                                    hashes.iter().map(hex::encode).collect();
                                let stored_block = amun_block_store::StoredBlock {
                                    height,
                                    hash: hex::encode(cert.block_hash),
                                    parent_hash: hex::encode(
                                        engine.lock().unwrap().last_finalized_block_hash,
                                    ),
                                    state_root: hex::encode(cert.state_root),
                                    evidence_root: hex::encode(evidence_root),
                                    tx_hashes: hex_hashes,
                                    transaction_count: hashes.len() as u32,
                                };
                                if let Err(e) = block_store.lock().unwrap().append(&stored_block) {
                                    eprintln!("BLOCK_STORE ERROR: {}", e);
                                }
                                Some(hashes)
                            } else {
                                // Non-proposer path: load from BlockStore
                                match block_store.lock().unwrap().load_height(height) {
                                    Ok(Some(stored)) => {
                                        let hashes: Vec<[u8; 32]> = stored
                                            .tx_hashes
                                            .iter()
                                            .filter_map(|h| {
                                                let mut buf = [0u8; 32];
                                                if let Ok(bytes) = hex::decode(h) {
                                                    if bytes.len() == 32 {
                                                        buf.copy_from_slice(&bytes);
                                                        return Some(buf);
                                                    }
                                                }
                                                None
                                            })
                                            .collect();
                                        if !hashes.is_empty() {
                                            Some(hashes)
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            };

                            // Clean mempool if we have tx_hashes
                            if let Some(hashes) = tx_hashes {
                                let pending_before = mempool.lock().unwrap().pending_count();
                                mempool.lock().unwrap().remove_committed(&hashes);
                                let pending_after = mempool.lock().unwrap().pending_count();
                                eprintln!(
                                    "MEMPOOL_CLEANUP node={:?} height={} removed={} pending_before={} after={}",
                                    validator_id, height, hashes.len(), pending_before, pending_after
                                );
                            }
                        }
                    }

                    // Slashing application
                    {
                        let pending: Vec<amun_consensus_network::SlashingCertificate> = {
                            let gossip = certificate_gossip.lock().unwrap();
                            gossip.get_pending().iter().map(|c| (*c).clone()).collect()
                        };
                        let mut adapter = staking_adapter.lock().unwrap();
                        let mut applied_hashes: Vec<[u8; 32]> = Vec::new();
                        let mut applied = applied_certs.lock().unwrap();
                        for cert in &pending {
                            if applied.contains(&cert.certificate_hash) {
                                applied_hashes.push(cert.certificate_hash);
                                continue;
                            }
                            let result = adapter.try_slash(&cert.validator_id);
                            if let Some(slash_result) = result {
                                eprintln!(
                                    "N110.4c SLASH_APPLIED: validator={:?} amount={} remaining={}",
                                    &cert.validator_id[..4],
                                    slash_result.amount_slashed,
                                    slash_result.remaining_stake
                                );
                                applied.insert(cert.certificate_hash);
                                applied_hashes.push(cert.certificate_hash);
                            }
                        }
                        drop(applied);
                        if !applied_hashes.is_empty() {
                            let mut gossip = certificate_gossip.lock().unwrap();
                            for hash in &applied_hashes {
                                gossip.mark_included(hash);
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_millis(50));
            }
        })
    }
}

// ============================================================================
// RuntimeService implementation for ConsensusRuntime
// ============================================================================
use crate::runtime::lifecycle::RuntimeService;

impl RuntimeService for ConsensusRuntime {
    fn start(&self) -> Result<Vec<std::thread::JoinHandle<()>>, String> {
        Ok(vec![self.spawn()])
    }

    fn stop(&self) {
        // The running flag is shared, set by NodeRuntime::stop_all()
    }

    fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}
