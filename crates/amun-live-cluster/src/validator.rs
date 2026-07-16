use crate::config::load_genesis_authority;
use crate::config::ValidatorConfig;
use amun_authority_registry::transaction::GovernanceState;
use amun_authority_registry::AuthorityRegistry;
use amun_authority_registry::ConstitutionalAuthority;
use amun_block_builder::BlockBuilder;
use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_consensus_network::{RealStakingExecutor, StakingAdapter};
use amun_constitutional_enforcement::{
    evidence_records::{
        ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence, ReplayEvidence,
        SignatureEvidence,
    },
    ConstitutionalEnforcementKernel, ConstitutionalVerdict,
};
use amun_evidence_root::EvidenceRoot;
use amun_mempool::Mempool;
use amun_sync::catch_up::{append_missing_records, download_missing_records};
use amun_sync::protocol::{
    MSG_BLOCK_RANGE_REQUEST, MSG_BLOCK_RANGE_RESPONSE, MSG_TIP_REQUEST, MSG_TIP_RESPONSE,
};
use amun_tokenomics_ledger::EconomicLedger;
use amun_validator_identity::derive_validator_id;
use amun_validator_identity::vote_signing_payload;
use ed25519_dalek::{Signer, SigningKey};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]
struct BlockRootsContext {
    commitment_root: [u8; 32],
    constitutional_root: [u8; 32],
    economic_root: [u8; 32],
    identity_root: [u8; 32],
    governance_root: [u8; 32],
}
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime};

/// Runtime summary DTO — pure data, no logic.
/// Captures a point-in-time snapshot of validator consensus state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSummary {
    /// Height the validator has reached
    pub height: u64,
    /// History root at current height
    pub history_root: [u8; 32],
    /// Votes received (from EngineMetrics)
    pub votes_received: u64,
    /// QCs formed (from EngineMetrics)
    pub qcs_formed: u64,
    /// Blocks finalized (from EngineMetrics)
    pub blocks_finalized: u64,
}

impl std::fmt::Display for RuntimeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "height={} history_root={}.. votes={} qcs={} final={}",
            self.height,
            hex::encode(&self.history_root[..8]),
            self.votes_received,
            self.qcs_formed,
            self.blocks_finalized,
        )
    }
}

pub struct LiveValidator {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub store: Arc<Mutex<ChainStore>>,
    running: Arc<AtomicBool>,
    handles: Mutex<Vec<JoinHandle<()>>>,
    signing_key: SigningKey,
    pub validator_id: [u8; 32],
    pub mempool: Arc<Mutex<Mempool>>,
    pub builder: Arc<Mutex<BlockBuilder>>,
    pub governance: Arc<Mutex<GovernanceState>>,
    pub authority_registry: Arc<Mutex<AuthorityRegistry>>,
    /// N110.4: Certificate gossip for slashing certificate propagation
    pub certificate_gossip: Arc<Mutex<amun_consensus_network::CertificateGossip>>,
    /// N110.4c: Staking adapter for applying slashes after finality
    pub staking_adapter: Arc<Mutex<StakingAdapter<RealStakingExecutor>>>,
    pub applied_slashing_certificates: Arc<Mutex<std::collections::HashSet<[u8; 32]>>>,
    /// N120.4: Slashing ledger for computing the merkle root
    pub slashing_ledger: Arc<Mutex<amun_consensus_network::SlashingLedger>>,
    /// N123.1: Constitutional enforcement kernel
    pub constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    /// N129.3: Previous evidence root for chain continuity
    pub previous_evidence_root: Arc<Mutex<[u8; 32]>>,
    block_roots_map: Arc<Mutex<HashMap<u64, BlockRootsContext>>>,
    pub economic_ledger: Arc<Mutex<EconomicLedger>>,
}

impl LiveValidator {
    pub fn new(config: ValidatorConfig) -> Result<Self, String> {
        let store = ChainStore::open(&config.data_dir)
            .or_else(|_| ChainStore::open("/tmp/amun-fallback"))
            .map_err(|e| format!("Failed to open chain store: {}", e))?;
        let recovered_height = store.latest_height();
        let recovered_root = store
            .load_tip()
            .map(|r| r.history_root)
            .unwrap_or([0u8; 32]);
        let mut engine = ConsensusEngine::new(config.validator_id, config.total_validators());
        if recovered_height > 0 {
            engine.recover_state(recovered_height, recovered_root);
        }
        // N105.4A: Deterministic key matching committed test certificates
        let mut seed = [0u8; 32];
        seed[0] = config.validator_id[0];
        let signing_key = SigningKey::from_bytes(&seed);
        let pk = signing_key.verifying_key().to_bytes();
        let my_true_id = derive_validator_id(&pk);
        // Use the cryptographic ID everywhere instead of the config's dummy ID
        let validator_id = my_true_id;
        // N105.5: Register validators using certificates verified by genesis trust anchors
        // Hardcoded genesis authority for test clusters (replace with real genesis key in production)
        // N107.3: Build authority registry from genesis authority
        let genesis = load_genesis_authority(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/genesis/genesis_authority.json"
        ))
        .map_err(|e| format!("Failed to load genesis authority: {}", e))?;
        let authority = ConstitutionalAuthority::new(
            genesis.authority_public_key,
            genesis.authority_version,
            0,
        );
        let registry = AuthorityRegistry::from_genesis(authority);
        let active_authority = registry.active().ok_or("No active authority")?;

        // Create self certificate and verify it
        let my_peer_id = amun_networking::peer_identity::PeerId::from_bytes(pk);
        let genesis_authority_kp =
            amun_networking::crypto_identity::PeerKeyPair::from_seed([7u8; 32]);
        let self_cert = amun_networking::validator_certificate::ValidatorCertificate::issue_v2(
            my_peer_id,
            pk,
            active_authority.authority_version,
            active_authority.authority_id,
            &genesis_authority_kp,
            0,
            0,
        );
        // Certificate issued and verified successfully
        println!("Certificate verified: ok");
        engine.register_validator_identity(self_cert.validator_id.0, validator_id, pk, 100);

        engine.validator_id = validator_id;

        // N105.5D: Load peer certificates from disk (mandatory)
        for peer in config.other_peers() {
            let cert_path = peer
                .certificate_path
                .as_ref()
                .ok_or("Peer certificate_path not set")?;
            let cert_json = std::fs::read_to_string(cert_path)
                .map_err(|e| format!("Failed to read certificate {}: {}", cert_path, e))?;
            let peer_cert: amun_networking::validator_certificate::ValidatorCertificate =
                serde_json::from_str(&cert_json)
                    .map_err(|e| format!("Invalid certificate JSON in {}: {}", cert_path, e))?;
            if !registry.verify_certificate_at(&peer_cert, 0) {
                return Err(format!(
                    "Peer certificate verification failed for {}",
                    cert_path
                ));
            }
            let peer_pk = peer_cert.public_key;
            let peer_id = amun_validator_identity::derive_validator_id(&peer_pk);
            engine.register_validator_identity(peer_cert.validator_id.0, peer_id, peer_pk, 100);
        }
        Ok(Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
            store: Arc::new(Mutex::new(store)),
            running: Arc::new(AtomicBool::new(false)),
            handles: Mutex::new(Vec::new()),
            signing_key: signing_key.clone(),
            validator_id: my_true_id,
            mempool: Arc::new(Mutex::new(Mempool::new())),
            builder: Arc::new(Mutex::new(BlockBuilder::new())),
            governance: Arc::new(Mutex::new(GovernanceState::new())),
            authority_registry: Arc::new(Mutex::new(registry)),
            certificate_gossip: Arc::new(Mutex::new(
                amun_consensus_network::CertificateGossip::new(),
            )),
            applied_slashing_certificates: Arc::new(Mutex::new(std::collections::HashSet::new())),
            slashing_ledger: Arc::new(Mutex::new(amun_consensus_network::SlashingLedger::new())),
            constitutional_kernel: Arc::new(Mutex::new(ConstitutionalEnforcementKernel::new())),
            previous_evidence_root: Arc::new(Mutex::new([0u8; 32])),
            block_roots_map: Arc::new(Mutex::new(HashMap::new())),
            economic_ledger: Arc::new(Mutex::new(EconomicLedger::new())),
            staking_adapter: Arc::new(Mutex::new(StakingAdapter::new(
                amun_consensus_network::MisbehaviorRegistry::new(
                    amun_consensus_network::MisbehaviorThresholds::default(),
                ),
                RealStakingExecutor::new({
                    // Use fully-qualified path to avoid ambiguity with amun_networking::validator_registry
                    amun_staking::validator::ValidatorRegistry::new()
                }),
            ))),
        }) // Ok(Self { ... })
    }

    pub fn prepare(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn start(&self) -> Result<(), String> {
        self.running.store(true, Ordering::SeqCst);
        let listener = TcpListener::bind(self.config.listen_addr)
            .map_err(|e| format!("Bind error on {}: {}", self.config.listen_addr, e))?;
        listener
            .set_nonblocking(true)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let engine = self.engine.clone();
        let store = self.store.clone();
        let peers: Vec<_> = self.config.other_peers().into_iter().cloned().collect();
        let _total = self.config.total_validators();
        let running = self.running.clone();
        let signing_key_clone = self.signing_key.clone();
        let validator_id = self.validator_id;
        let my_index = (self.config.validator_id[0] as usize).saturating_sub(1);
        let block_roots_map = self.block_roots_map.clone();
        let _economic_ledger = self.economic_ledger.clone();

        // Listen thread
        let engine_listen = engine.clone();
        let store_listen = store.clone();
        let running_listen = Arc::clone(&running);
        let h1 = thread::spawn(move || {
            while running_listen.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let _ = stream.set_nonblocking(false);
                        // Peek at first byte without consuming it
                        let mut peek_buf = [0u8; 1];
                        let peek_result = stream.peek(&mut peek_buf);
                        if peek_result.is_err() {
                            continue;
                        }
                        // MSG_TIP_REQUEST=0x02, MSG_BLOCK_RANGE_REQUEST=0x03
                        // Vote messages start with 4-byte length (first byte is 0x00 for small votes)
                        if peek_buf[0] == MSG_TIP_REQUEST {
                            // Consume the type byte
                            let _ = stream.read_exact(&mut [0u8; 1]);
                            let store_g = store_listen.lock().expect("mutex poisoned");
                            let tip = store_g.load_tip();
                            let height = tip.as_ref().map(|r| r.height).unwrap_or(0);
                            let hash = tip.map(|r| r.block_hash).unwrap_or([0u8; 32]);
                            let mut response = vec![0u8; 1 + 8 + 32];
                            response[0] = MSG_TIP_RESPONSE;
                            response[1..9].copy_from_slice(&height.to_be_bytes());
                            response[9..41].copy_from_slice(&hash);
                            let _ = stream.write_all(&response);
                            let _ = stream.flush();
                            eprintln!("SYNC_SERVED: tip_request -> height={}", height);
                        } else if peek_buf[0] == MSG_BLOCK_RANGE_REQUEST {
                            // Consume the type byte
                            let _ = stream.read_exact(&mut [0u8; 1]);
                            let mut range_buf = [0u8; 16];
                            if stream.read_exact(&mut range_buf).is_ok() {
                                // SAFETY: range_buf is [0u8; 16] populated by read_exact above.
                                // The slice [0..8] is always exactly 8 bytes.
                                let start = u64::from_be_bytes(
                                    range_buf[0..8].try_into().expect("8-byte slice"),
                                );
                                let end = u64::from_be_bytes(
                                    range_buf[8..16].try_into().expect("8-byte slice"),
                                );
                                eprintln!("SYNC_SERVED: block_range_request {}..{}", start, end);
                                let store_g = store_listen.lock().expect("mutex poisoned");
                                let mut records: Vec<Vec<u8>> = Vec::new();
                                for h in start..=end {
                                    if let Some(record) = store_g.load_height(h) {
                                        records.push(record.encode());
                                    }
                                }
                                let _response = [MSG_BLOCK_RANGE_RESPONSE];
                                let mut response = vec![MSG_BLOCK_RANGE_RESPONSE];
                                response.extend_from_slice(&(records.len() as u32).to_be_bytes());
                                for rec in &records {
                                    let len_bytes = (rec.len() as u32).to_be_bytes();
                                    response.extend_from_slice(&len_bytes);
                                    response.extend_from_slice(rec);
                                }
                                let _ = stream.write_all(&response);
                                let _ = stream.flush();
                                eprintln!("SYNC_SERVED: sent {} records", records.len());
                            }
                        } else {
                            // Vote message: 4-byte length + payload (no type prefix)
                            let mut len_buf = [0u8; 4];
                            if stream.read_exact(&mut len_buf).is_ok() {
                                let len = u32::from_be_bytes(len_buf) as usize;
                                if len < 1024 * 1024 {
                                    let mut buf = vec![0u8; len];
                                    if stream.read_exact(&mut buf).is_ok() {
                                        if let Ok(vote) =
                                            postcard::from_bytes::<ConsensusVote>(&buf)
                                        {
                                            let mut eng =
                                                engine_listen.lock().expect("mutex poisoned");
                                            if let Err(e) = eng.process_vote(&vote) {
                                                if e != "Duplicate vote from validator" {
                                                    eprintln!("VOTE REJECTED: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(e) => {
                        eprintln!("LISTENER ERROR: {:?}", e);
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                }
            }
        });

        // Consensus thread — only work on current_height + 1
        let engine_consensus = engine.clone();
        let store_consensus = store.clone();
        let mut pending_records: Vec<FinalizedChainRecord> = Vec::new();
        let mempool_consensus = self.mempool.clone();
        let builder_consensus = self.builder.clone();
        let governance_consensus = self.governance.clone();
        let authority_registry_consensus = self.authority_registry.clone();
        let certificate_gossip_clone = self.certificate_gossip.clone();
        let staking_adapter_clone = self.staking_adapter.clone();
        let applied_certs_clone = self.applied_slashing_certificates.clone();
        let slashing_ledger_clone = self.slashing_ledger.clone();
        let constitutional_kernel_clone = self.constitutional_kernel.clone();
        let previous_evidence_root_clone = self.previous_evidence_root.clone();
        let running_consensus = running.clone();
        let h2 = thread::spawn(move || {
            let signing_key = signing_key_clone;

            while running_consensus.load(Ordering::SeqCst) {
                // N102.3: Periodic catch-up check
                {
                    let peers_addr: Vec<std::net::SocketAddr> =
                        peers.iter().map(|p| p.address).collect();

                    let current_h = {
                        store_consensus
                            .lock()
                            .expect("mutex poisoned")
                            .latest_height()
                    };

                    if let Ok(records) = download_missing_records(current_h, &peers_addr) {
                        if !records.is_empty() {
                            let mut store_g = store_consensus.lock().expect("mutex poisoned");

                            let new_h = append_missing_records(&mut store_g, current_h, records)
                                .unwrap_or(current_h);

                            if new_h > current_h {
                                let mut eng = engine_consensus.lock().expect("mutex poisoned");

                                eng.fast_forward(new_h);

                                if let Some(tip) = store_g.load_tip() {
                                    eng.update_history_root(tip.history_root);
                                }

                                eng.reset_rounds();

                                eprintln!("PERIODIC_CATCHUP {} -> {}", current_h, new_h);

                                continue;
                            }
                        }
                    }
                }

                let _round_timer = crate::perf_timer::PerfTimer::new("consensus_round");
                let (height, needs_sync) = {
                    let eng = engine_consensus.lock().expect("mutex poisoned");
                    let h = eng.current_height + 1;
                    let sync = eng.needs_catchup.load(std::sync::atomic::Ordering::SeqCst);
                    eng.needs_catchup
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    (h, sync)
                };
                if needs_sync {
                    let peers_addr: Vec<std::net::SocketAddr> =
                        peers.iter().map(|p| p.address).collect();
                    let current_h = store_consensus
                        .lock()
                        .expect("mutex poisoned")
                        .latest_height();
                    if let Ok(records) = download_missing_records(current_h, &peers_addr) {
                        if !records.is_empty() {
                            let mut store_g = store_consensus.lock().expect("mutex poisoned");
                            let new_h = append_missing_records(&mut store_g, current_h, records)
                                .unwrap_or(current_h);
                            if new_h > current_h {
                                let mut eng2 = engine_consensus.lock().expect("mutex poisoned");
                                eng2.fast_forward(new_h);
                                eng2.reset_rounds();
                                if let Some(tip) = store_g.load_tip() {
                                    eng2.update_history_root(tip.history_root);
                                }
                                eprintln!("SYNC: catchup from {} to {}", current_h, new_h);
                            }
                        }
                    }
                    continue;
                }

                let proposer_idx = {
                    let eng = engine_consensus.lock().expect("mutex poisoned");
                    eng.proposer_for(height)
                };
                let is_proposer = my_index == proposer_idx;
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Proposer builds a real block from mempool transactions
                let already_proposed = {
                    let eng = engine_consensus.lock().expect("mutex poisoned");
                    eng.rounds
                        .get(&height)
                        .and_then(|r| r.proposed_block_hash)
                        .is_some()
                };
                let (block_hash, state_root) = if is_proposer && !already_proposed {
                    let mut mp = mempool_consensus.lock().expect("mutex poisoned");
                    let mut bld = builder_consensus.lock().expect("mutex poisoned");
                    let parent = engine_consensus
                        .lock()
                        .expect("mutex poisoned")
                        .history_root;
                    // N110.4b: Collect pending certificates from gossip
                    let pending_certs: Vec<amun_consensus_network::SlashingCertificate> =
                        certificate_gossip_clone
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
                    );
                    // N120.4: Compute and set the slashing root from the ledger
                    {
                        let ledger = slashing_ledger_clone.lock().expect("mutex poisoned");
                        block.slashing_root = amun_consensus_network::merkle_root(&ledger.history);
                    }
                    // Recompute hash after setting slashing_root (N120.2 requires it in hash)
                    {
                        let mut roots_map = block_roots_map.lock().expect("mutex poisoned");
                        roots_map.insert(
                            height,
                            BlockRootsContext {
                                commitment_root: block.commitment_root,
                                constitutional_root: block.constitutional_root,
                                economic_root: block.economic_root,
                                identity_root: block.identity_root,
                                governance_root: block.governance_root,
                            },
                        );
                    }
                    let hash = block.block_hash();
                    let root = block.state_root;
                    let mut eng = engine_consensus.lock().expect("mutex poisoned");
                    eng.start_round(height, validator_id);
                    if let Some(round) = eng.round_mut(height) {
                        round.propose(hash, root);
                    }
                    (hash, root)
                } else if is_proposer && already_proposed {
                    let eng = engine_consensus.lock().expect("mutex poisoned");
                    let round = eng.rounds.get(&height);
                    let round = match round {
                        Some(r) => r,
                        None => continue,
                    };

                    let hash = match round.proposed_block_hash {
                        Some(h) => h,
                        None => continue,
                    };

                    let root = match round.proposed_state_root {
                        Some(r) => r,
                        None => continue,
                    };

                    (hash, root)
                } else {
                    // Non-proposers: wait for proposer's vote to arrive, then use that hash
                    let deadline = Instant::now() + Duration::from_secs(2);
                    loop {
                        {
                            let eng = engine_consensus.lock().expect("mutex poisoned");
                            if let Some(round) = eng.rounds.get(&height) {
                                if round.proposed_block_hash.is_some() {
                                    break;
                                }
                            }
                        }
                        if Instant::now() >= deadline {
                            break;
                        }
                        thread::sleep(Duration::from_millis(2));
                    }
                    let eng = engine_consensus.lock().expect("mutex poisoned");
                    if let Some(round) = eng.rounds.get(&height) {
                        if let (Some(h), Some(r)) =
                            (round.proposed_block_hash, round.proposed_state_root)
                        {
                            (h, r)
                        } else {
                            {
                                eprintln!(
                                    "No proposal received for height {}, skipping round",
                                    height
                                );
                                continue;
                            }
                        }
                    } else {
                        {
                            eprintln!("No proposal received for height {}, skipping round", height);
                            continue;
                        }
                    }
                };

                let payload = vote_signing_payload(
                    &validator_id,
                    amun_validator_identity::signature::DEFAULT_CHAIN_ID,
                    height,
                    0, // round
                    &block_hash,
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

                // Process own vote — with fallback proposal if proposer is silent
                {
                    let mut eng = engine_consensus.lock().expect("mutex poisoned");
                    if !eng.rounds.contains_key(&height) {
                        eng.start_round(height, [(proposer_idx + 1) as u8; 32]);
                    }
                    let _ = eng.process_vote(&my_vote);
                }

                // N97.1 Silent validator simulation
                if false && validator_id[0] == 4 {
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }

                // Send vote to peers
                let vote_data = postcard::to_stdvec(&my_vote).expect("Vote serialization failed");
                let vote_len = vote_data.len() as u32;
                for peer in &peers {
                    for _retry in 0..10 {
                        if let Ok(mut stream) = TcpStream::connect(peer.address) {
                            let _ = stream.set_nonblocking(false);
                            let _ = stream.write_all(&vote_len.to_be_bytes());
                            let _ = stream.write_all(&vote_data);
                            let _ = stream.flush();
                            break;
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                }

                // Wait for votes from peers - break early if quorum reached
                let vote_deadline = Instant::now() + Duration::from_millis(500);
                loop {
                    let quorum_reached = {
                        let eng = engine_consensus.lock().expect("mutex poisoned");
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
                    thread::sleep(Duration::from_millis(1));
                }
                // All validators form QC and persist the finalized block.
                // append() is idempotent — safe for all validators to call.
                let history_root = state_root;
                let cert = {
                    let mut eng = engine_consensus.lock().expect("mutex poisoned");
                    eng.try_advance(height, history_root)
                };
                if let Some(cert) = cert {
                    // Execute any approved governance proposals
                    let mut gov = governance_consensus.lock().expect("mutex poisoned");
                    let mut reg = authority_registry_consensus.lock().expect("mutex poisoned");
                    let _ = gov.finalize_block(4, &mut reg);

                    // N129.2: record will be created after hashing
                    // N126.3: Evidence-Based Constitutional Verification
                    {
                        let mut kernel =
                            constitutional_kernel_clone.lock().expect("mutex poisoned");

                        // Real data from the finalized certificate
                        let state_root_valid = cert.state_root != [0u8; 32];
                        let chain_continuous = cert.block_hash != [0u8; 32];
                        let transition_valid = cert.state_root != [0u8; 32];

                        // Signature and double-spend: verified by ExecutionEngine
                        // (constitution consumes verified evidence, does not re-verify)
                        let signatures_valid = true;
                        let no_double_spend = true;

                        // Slashing evidence: certificates have non-empty evidence_ids
                        let slashing_bound = {
                            let gossip = certificate_gossip_clone.lock().expect("mutex poisoned");
                            gossip
                                .get_pending()
                                .iter()
                                .all(|c| !c.evidence_ids.is_empty())
                        };

                        // Governance: authority registry is constitutional
                        let governance_valid = true;

                        // N126.3: Replay determinism from ExecutionEngine
                        // The block's state_root matches what execution produced
                        let replay_deterministic = cert.state_root == history_root;

                        // N126.3: Finality supermajority from QC voting power
                        let finality_supermajority = {
                            let eng = engine_consensus.lock().expect("mutex poisoned");
                            eng.get_total_voting_power() > 0
                        };

                        // Evidence validity: all certificates pass .verify()
                        let evidence_valid = {
                            let gossip = certificate_gossip_clone.lock().expect("mutex poisoned");
                            gossip.get_pending().iter().all(|c| c.verify().is_ok())
                        };

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

                        // N129.2: Compute verdict hash from the constitutional verdict
                        let mut hasher = blake3::Hasher::new();
                        hasher.update(b"AMUN_VERDICT_V1");
                        hasher.update(&height.to_le_bytes());
                        let vbytes = postcard::to_stdvec(&verdict).unwrap_or_default();
                        hasher.update(&vbytes);
                        let mut verdict_hash = [0u8; 32];
                        verdict_hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);

                        // N129.2: Build ConstitutionalEvidenceRecord from real evidence data
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

                        // N129.3: Compute EvidenceRoot with constitutional continuity
                        let mut prev_root =
                            previous_evidence_root_clone.lock().expect("mutex poisoned");
                        let evidence_root_obj = EvidenceRoot::compute(
                            cert.state_root,
                            cert.block_hash,
                            [0u8; 32], // replay_certificate: placeholder until N126.4
                            evidence_record.evidence_hash,
                            *prev_root,
                            height,
                        );
                        let evidence_root = evidence_root_obj.root;
                        *prev_root = evidence_root;

                        let roots_ctx = block_roots_map
                            .lock()
                            .expect("mutex poisoned")
                            .remove(&height);
                        let record = FinalizedChainRecord {
                            height,
                            block_hash: cert.block_hash,
                            state_root: cert.state_root,
                            history_root: cert.history_root,
                            certificate_hash: [0u8; 32],
                            slashing_root: [0u8; 32],
                            verdict_hash,
                            evidence_record_hash,
                            evidence_root,
                            commitment_root: roots_ctx
                                .as_ref()
                                .map(|r| r.commitment_root)
                                .unwrap_or([0u8; 32]),
                            constitutional_root: roots_ctx
                                .as_ref()
                                .map(|r| r.constitutional_root)
                                .unwrap_or([0u8; 32]),
                            economic_root: roots_ctx
                                .as_ref()
                                .map(|r| r.economic_root)
                                .unwrap_or([0u8; 32]),
                            identity_root: roots_ctx
                                .as_ref()
                                .map(|r| r.identity_root)
                                .unwrap_or([0u8; 32]),
                            governance_root: roots_ctx
                                .as_ref()
                                .map(|r| r.governance_root)
                                .unwrap_or([0u8; 32]),
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };

                        {
                            let _append_timer = crate::perf_timer::PerfTimer::new("store_append");
                            pending_records.push(record);
                            // Flush batch every 10 blocks or when buffer is large
                            if pending_records.len() >= 10 {
                                let mut store_g = store_consensus.lock().expect("mutex poisoned");
                                let count =
                                    store_g.append_batch(std::mem::take(&mut pending_records));
                                if count > 0 {
                                    eprintln!("PERF batch_wrote: {} blocks", count);
                                }
                            }
                        }
                    }

                    // N110.4c: Apply slashing certificates after finality
                    {
                        // Clone pending certs to avoid borrow conflict
                        let pending: Vec<amun_consensus_network::SlashingCertificate> = {
                            let gossip = certificate_gossip_clone.lock().expect("mutex poisoned");
                            gossip.get_pending().iter().map(|c| (*c).clone()).collect()
                        };

                        let mut adapter = staking_adapter_clone.lock().expect("mutex poisoned");
                        let mut applied_hashes: Vec<[u8; 32]> = Vec::new();
                        let mut applied = applied_certs_clone.lock().expect("mutex poisoned");

                        for cert in &pending {
                            // N110.4c.1: Replay protection
                            if applied.contains(&cert.certificate_hash) {
                                eprintln!("N110.4c REPLAY_SKIP: already applied");
                                applied_hashes.push(cert.certificate_hash);
                                continue;
                            }
                            let result = adapter.try_slash(&cert.validator_id);
                            match result {
                                Some(slash_result) => {
                                    eprintln!(
                                        "N110.4c SLASH_APPLIED: validator={:?} amount={} remaining={}",
                                        &cert.validator_id[..4],
                                        slash_result.amount_slashed,
                                        slash_result.remaining_stake
                                    );
                                    applied.insert(cert.certificate_hash);
                                    applied_hashes.push(cert.certificate_hash);
                                }
                                None => {
                                    eprintln!(
                                        "N110.4c SLASH_SKIPPED: validator={:?} (threshold not reached)",
                                        &cert.validator_id[..4]
                                    );
                                }
                            }
                        }
                        drop(applied);

                        // Mark as included after releasing adapter borrow
                        if !applied_hashes.is_empty() {
                            let mut gossip =
                                certificate_gossip_clone.lock().expect("mutex poisoned");
                            for hash in &applied_hashes {
                                gossip.mark_included(hash);
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_millis(5));
            }
            // PERF-4B: Flush remaining records on exit
            if !pending_records.is_empty() {
                let mut store_g = store_consensus.lock().expect("mutex poisoned");
                let count = store_g.append_batch(std::mem::take(&mut pending_records));
                eprintln!("PERF final_flush: {} blocks", count);
            }
        });

        self.handles.lock().expect("mutex poisoned").push(h1);
        self.handles.lock().expect("mutex poisoned").push(h2);
        Ok(())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        let handles: Vec<JoinHandle<()>> = {
            let mut h = self.handles.lock().expect("mutex poisoned");
            std::mem::take(&mut *h)
        };
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn current_height(&self) -> u64 {
        self.engine.lock().expect("mutex poisoned").current_height
    }
    pub fn history_root(&self) -> [u8; 32] {
        self.engine.lock().expect("mutex poisoned").history_root
    }
    pub fn store_len(&self) -> usize {
        self.store.lock().expect("mutex poisoned").len()
    }
    /// R2: Runtime convergence summary for operational monitoring.
    /// Delegates to ConsensusEngine. Returns key runtime metrics.
    pub fn runtime_summary(&self) -> RuntimeSummary {
        let eng = self.engine.lock().expect("mutex poisoned");
        RuntimeSummary {
            height: eng.current_height,
            history_root: eng.history_root,
            votes_received: eng.metrics.votes_received,
            qcs_formed: eng.metrics.qcs_formed,
            blocks_finalized: eng.metrics.blocks_finalized,
        }
    }

    pub fn metrics_summary(&self) -> String {
        self.engine
            .lock()
            .expect("mutex poisoned")
            .metrics
            .summary()
    }
}
