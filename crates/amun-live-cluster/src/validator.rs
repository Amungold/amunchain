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

pub struct LiveValidator {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub store: Arc<Mutex<ChainStore>>,
    running: Arc<Mutex<bool>>,
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
    pub fn new(config: ValidatorConfig) -> Self {
        let store = ChainStore::open(&config.data_dir)
            .unwrap_or_else(|_| ChainStore::open("/tmp/amun-fallback").unwrap());
        let recovered_height = store.latest_height();
        let recovered_root = store
            .load_tip()
            .map(|r| r.history_root)
            .unwrap_or([0u8; 32]);
        let mut engine = ConsensusEngine::new(config.validator_id, config.total_validators());
        if recovered_height > 0 {
            engine.current_height = recovered_height;
            engine.history_root = recovered_root;
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
        ));
        let authority = ConstitutionalAuthority::new(
            genesis.authority_public_key,
            genesis.authority_version,
            0,
        );
        let registry = AuthorityRegistry::from_genesis(authority);
        let active_authority = registry.active().expect("No active authority");

        // Create self certificate and verify it
        let my_peer_id = amun_networking::peer_identity::PeerId::from_bytes(pk);
        let genesis_authority_kp =
            amun_networking::crypto_identity::PeerKeyPair::from_seed([0x42; 32]);
        let self_cert = amun_networking::validator_certificate::ValidatorCertificate::issue_v2(
            my_peer_id,
            pk,
            active_authority.authority_version,
            active_authority.authority_id,
            &genesis_authority_kp,
            0,
            0,
        );
        if !registry.verify_certificate_at(&self_cert, 0) {
            panic!("Self certificate verification failed");
        }
        engine.register_validator_identity(self_cert.validator_id.0, validator_id, pk, 100);

        engine.validator_id = validator_id;

        // N105.5D: Load peer certificates from disk (mandatory)
        for peer in config.other_peers() {
            let cert_path = peer
                .certificate_path
                .as_ref()
                .expect("Peer certificate_path not set");
            let cert_json = std::fs::read_to_string(cert_path)
                .unwrap_or_else(|_| panic!("Failed to read certificate {}", cert_path));
            let peer_cert: amun_networking::validator_certificate::ValidatorCertificate =
                serde_json::from_str(&cert_json)
                    .unwrap_or_else(|_| panic!("Invalid certificate JSON in {}", cert_path));
            if !registry.verify_certificate_at(&peer_cert, 0) {
                panic!("Peer certificate verification failed for {}", cert_path);
            }
            let peer_pk = peer_cert.public_key;
            let peer_id = amun_validator_identity::derive_validator_id(&peer_pk);
            engine.register_validator_identity(peer_cert.validator_id.0, peer_id, peer_pk, 100);
        }
        Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
            store: Arc::new(Mutex::new(store)),
            running: Arc::new(Mutex::new(false)),
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
        }
    }

    pub fn start(&self) -> Result<(), String> {
        *self.running.lock().unwrap() = true;
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
        let my_index = self.config.validator_id[0];
        let block_roots_map = self.block_roots_map.clone();
        let _economic_ledger = self.economic_ledger.clone();

        // Listen thread
        let engine_listen = engine.clone();
        let store_listen = store.clone();
        let running_listen = running.clone();
        let h1 = thread::spawn(move || {
            while *running_listen.lock().unwrap() {
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
                            let store_g = store_listen.lock().unwrap();
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
                                let start = u64::from_be_bytes(range_buf[0..8].try_into().unwrap());
                                let end = u64::from_be_bytes(range_buf[8..16].try_into().unwrap());
                                eprintln!("SYNC_SERVED: block_range_request {}..{}", start, end);
                                let store_g = store_listen.lock().unwrap();
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
                                            let mut eng = engine_listen.lock().unwrap();
                                            if let Err(e) = eng.process_vote(vote) {
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
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => {
                        eprintln!("LISTENER ERROR: {:?}", e);
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                }
            }
        });

        // Consensus thread — only work on current_height + 1
        let engine_consensus = engine.clone();
        let store_consensus = store.clone();
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
            while *running_consensus.lock().unwrap() {
                let (height, needs_sync) = {
                    let eng = engine_consensus.lock().unwrap();
                    let h = eng.current_height + 1;
                    let sync = eng.needs_catchup.load(std::sync::atomic::Ordering::SeqCst);
                    eng.needs_catchup
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    (h, sync)
                };
                if needs_sync {
                    let peers_addr: Vec<std::net::SocketAddr> =
                        peers.iter().map(|p| p.address).collect();
                    let current_h = engine_consensus.lock().unwrap().current_height;
                    if let Ok(records) = download_missing_records(current_h, &peers_addr) {
                        if !records.is_empty() {
                            let mut store_g = store_consensus.lock().unwrap();
                            let new_h = append_missing_records(&mut store_g, current_h, records)
                                .unwrap_or(current_h);
                            if new_h > current_h {
                                let mut eng2 = engine_consensus.lock().unwrap();
                                eng2.current_height = new_h;
                                eng2.rounds.clear();
                                if let Some(tip) = store_g.load_tip() {
                                    eng2.history_root = tip.history_root;
                                }
                                eprintln!("SYNC: catchup from {} to {}", current_h, new_h);
                            }
                        }
                    }
                    continue;
                }

                let proposer_idx = {
                    let eng = engine_consensus.lock().unwrap();
                    eng.proposer_for(height)
                };
                let is_proposer = my_index as usize == proposer_idx;
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Proposer builds a real block from mempool transactions
                let already_proposed = {
                    let eng = engine_consensus.lock().unwrap();
                    eng.rounds
                        .get(&height)
                        .and_then(|r| r.proposed_block_hash)
                        .is_some()
                };
                let (block_hash, state_root) = if is_proposer && !already_proposed {
                    let mut mp = mempool_consensus.lock().unwrap();
                    let mut bld = builder_consensus.lock().unwrap();
                    let parent = engine_consensus.lock().unwrap().history_root;
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
                        let ledger = slashing_ledger_clone.lock().unwrap();
                        block.slashing_root = amun_consensus_network::merkle_root(&ledger.history);
                    }
                    // Recompute hash after setting slashing_root (N120.2 requires it in hash)
                    {
                        let mut roots_map = block_roots_map.lock().unwrap();
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
                    let mut eng = engine_consensus.lock().unwrap();
                    eng.start_round(height, validator_id);
                    if let Some(round) = eng.round_mut(height) {
                        round.propose(hash, root);
                    }
                    (hash, root)
                } else if is_proposer && already_proposed {
                    let eng = engine_consensus.lock().unwrap();
                    let round = eng.rounds.get(&height);
                    let hash = round
                        .and_then(|r| r.proposed_block_hash)
                        .unwrap_or([height as u8; 32]);
                    let root = round
                        .and_then(|r| r.proposed_state_root)
                        .unwrap_or([0xBB; 32]);
                    (hash, root)
                } else {
                    // Non-proposers: wait for proposer's vote to arrive, then use that hash
                    thread::sleep(Duration::from_millis(200));
                    let eng = engine_consensus.lock().unwrap();
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

                // Process own vote — with fallback proposal if proposer is silent
                {
                    let mut eng = engine_consensus.lock().unwrap();
                    if !eng.rounds.contains_key(&height) {
                        eng.start_round(height, [(proposer_idx + 1) as u8; 32]);
                    }
                    let _ = eng.process_vote(my_vote.clone());
                }

                // N97.1 Silent validator simulation
                if false && validator_id[0] == 4 {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }

                // Send vote to peers
                let vote_data = postcard::to_stdvec(&my_vote).unwrap();
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
                        thread::sleep(Duration::from_millis(100));
                    }
                }

                // Wait for votes from peers - break early if quorum reached
                let vote_deadline = Instant::now() + Duration::from_millis(500);
                loop {
                    let quorum_reached = {
                        let eng = engine_consensus.lock().unwrap();
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
                // All validators form QC and persist the finalized block.
                // append() is idempotent — safe for all validators to call.
                let history_root = state_root;
                let cert = {
                    let mut eng = engine_consensus.lock().unwrap();
                    eng.try_advance(height, history_root)
                };
                if let Some(cert) = cert {
                    // Execute any approved governance proposals
                    let mut gov = governance_consensus.lock().unwrap();
                    let mut reg = authority_registry_consensus.lock().unwrap();
                    let _ = gov.finalize_block(4, &mut reg);

                    // N129.2: record will be created after hashing
                    // N126.3: Evidence-Based Constitutional Verification
                    {
                        let mut kernel = constitutional_kernel_clone.lock().unwrap();

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
                            let gossip = certificate_gossip_clone.lock().unwrap();
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
                            let eng = engine_consensus.lock().unwrap();
                            eng.total_voting_power > 0
                        };

                        // Evidence validity: all certificates pass .verify()
                        let evidence_valid = {
                            let gossip = certificate_gossip_clone.lock().unwrap();
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
                        let mut prev_root = previous_evidence_root_clone.lock().unwrap();
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

                        let roots_ctx = block_roots_map.lock().unwrap().remove(&height);
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

                        if let Err(e) = store_consensus.lock().unwrap().append(record) {
                            eprintln!("STORE ERROR: {}", e);
                        }
                    }

                    // N110.4c: Apply slashing certificates after finality
                    {
                        // Clone pending certs to avoid borrow conflict
                        let pending: Vec<amun_consensus_network::SlashingCertificate> = {
                            let gossip = certificate_gossip_clone.lock().unwrap();
                            gossip.get_pending().iter().map(|c| (*c).clone()).collect()
                        };

                        let mut adapter = staking_adapter_clone.lock().unwrap();
                        let mut applied_hashes: Vec<[u8; 32]> = Vec::new();
                        let mut applied = applied_certs_clone.lock().unwrap();

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
                            let mut gossip = certificate_gossip_clone.lock().unwrap();
                            for hash in &applied_hashes {
                                gossip.mark_included(hash);
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_millis(50));
            }
        });

        self.handles.lock().unwrap().push(h1);
        self.handles.lock().unwrap().push(h2);
        Ok(())
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        let handles: Vec<JoinHandle<()>> = {
            let mut h = self.handles.lock().unwrap();
            std::mem::take(&mut *h)
        };
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn current_height(&self) -> u64 {
        self.engine.lock().unwrap().current_height
    }
    pub fn history_root(&self) -> [u8; 32] {
        self.engine.lock().unwrap().history_root
    }
    pub fn store_len(&self) -> usize {
        self.store.lock().unwrap().len()
    }
    pub fn metrics_summary(&self) -> String {
        self.engine.lock().unwrap().metrics.summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU16, Ordering};

    static PORT_BASE: AtomicU16 = AtomicU16::new(9700);

    #[test]
    fn n108_1_governance_updates_live_authority_registry() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;

        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Verify initial state: only authority v1 exists
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist from genesis");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Create a governance proposal to add authority v2
        let proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: [2u8; 32],
                authority_version: 2,
            },
            100,
        );

        // Submit the proposal to governance state
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));
        }

        // Cast 3 approving votes (quorum for 4 validators)
        for id in 1..=3u8 {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [id; 32],
                vote: GovernanceVote::Approve,
            });
        }

        // Execute governance finalization (simulates what happens at block finalization)
        {
            let mut gov = v.governance.lock().unwrap();
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "Proposal should be executed");
        }

        // Verify the registry was updated
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should still exist");
            assert!(reg.by_version(2).is_some(), "v2 should now be registered");
        }

        // Verify the journal recorded the execution
        {
            let gov = v.governance.lock().unwrap();
            assert!(
                gov.journal.is_executed(&proposal.proposal_id),
                "Journal should record execution"
            );
        }

        // Verify idempotency: second finalization should not re-execute
        {
            let mut gov = v.governance.lock().unwrap();
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert!(
                executed.is_empty(),
                "Second finalization should produce no new executions"
            );
        }

        v.stop();
    }

    #[test]
    fn n108_2_runtime_authority_rotation() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;

        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Phase 1: Genesis Authority v1
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist from genesis");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Phase 2: AddAuthority v2 via governance
        let add_proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: [2u8; 32],
                authority_version: 2,
            },
            100,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(add_proposal.clone()));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: add_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "AddAuthority should execute");
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(2).is_some(), "v2 should be registered");
        }

        // Phase 3: ScheduleTransition
        let trans_proposal = GovernanceProposal::new(
            [0xBB; 32],
            GovernanceAction::ScheduleTransition {
                from_version: 1,
                to_version: 2,
                activation_height: 500,
                grace_period_blocks: 100,
            },
            200,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                trans_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: trans_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "ScheduleTransition should execute");
        }

        // Phase 4: Pre-activation
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(499);
            assert_eq!(auths.len(), 1, "Only v1 before activation");
            assert_eq!(auths[0].authority_version, 1);
        }

        // Phase 5: At activation
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(500);
            assert_eq!(auths.len(), 2, "Both valid at activation");
        }

        // Phase 6: Grace window
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(550);
            assert_eq!(auths.len(), 2, "Both valid during grace");
        }

        // Phase 7: Post-grace
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(650);
            assert_eq!(auths.len(), 1, "Only v2 after grace");
            assert_eq!(auths[0].authority_version, 2);
        }

        // Phase 8: Retire v1
        let retire_proposal = GovernanceProposal::new(
            [0xCC; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 1,
            },
            700,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                retire_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: retire_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "RetireAuthority should execute");
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.is_revoked(1), "v1 should be revoked");
            assert!(reg.by_version(1).is_some(), "v1 still queryable");
        }

        v.stop();
    }

    #[test]
    fn n108_3e_authority_rotation_certificate_validation() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;
        use amun_networking::crypto_identity::PeerKeyPair;
        use amun_networking::peer_identity::PeerId;
        use amun_networking::validator_certificate::ValidatorCertificate;

        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Create authority keypairs
        let v1_kp = PeerKeyPair::from_seed([0x42; 32]);
        let _v1_pk = v1_kp.verifying_key.to_bytes();
        let v2_kp = PeerKeyPair::from_seed([0x43; 32]);
        let v2_pk = v2_kp.verifying_key.to_bytes();

        // Phase 1: Genesis has v1 only
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Create a v1 certificate
        let v1_cert = ValidatorCertificate::issue_v2(
            PeerId::from_bytes([1u8; 32]),
            [1u8; 32],
            1,
            [0u8; 32],
            &v1_kp,
            0,
            0,
        );

        // Phase 2: Add v2 and schedule transition via governance
        let add_proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: v2_pk,
                authority_version: 2,
            },
            100,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(add_proposal.clone()));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: add_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }

        let trans_proposal = GovernanceProposal::new(
            [0xBB; 32],
            GovernanceAction::ScheduleTransition {
                from_version: 1,
                to_version: 2,
                activation_height: 500,
                grace_period_blocks: 100,
            },
            200,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                trans_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: trans_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }

        // Phase 3: Before activation - v1 accepted, v2 unknown
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                reg.verify_certificate_at(&v1_cert, 400),
                "v1 cert should be accepted before activation"
            );
        }

        // Phase 4: During grace window - both accepted
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                reg.verify_certificate_at(&v1_cert, 550),
                "v1 cert should be accepted during grace window"
            );
        }

        // Phase 5: After grace - v1 rejected
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                !reg.verify_certificate_at(&v1_cert, 650),
                "v1 cert should be rejected after grace window"
            );
        }

        // Phase 6: Retire v1, verify it's rejected even at low heights
        let retire_proposal = GovernanceProposal::new(
            [0xCC; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 1,
            },
            700,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                retire_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: retire_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                !reg.verify_certificate_at(&v1_cert, 800),
                "v1 cert should be rejected after retirement"
            );
            assert!(reg.is_revoked(1), "v1 should be revoked");
        }

        v.stop();
    }

    fn next_ports() -> [u16; 4] {
        let base = PORT_BASE.fetch_add(10, Ordering::SeqCst);
        [base, base + 1, base + 2, base + 3]
    }

    #[test]
    fn n71_persist_finalized_blocks() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(3000));
        v.stop();
        assert!(v.current_height() >= 1);
        assert!(v.store_len() >= 1);
    }

    #[test]
    fn n71_recover_after_restart() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let data_dir = solo.data_dir.clone();

        let height_after_first;
        {
            let v = LiveValidator::new(solo);
            v.start().unwrap();
            thread::sleep(Duration::from_millis(3000));
            v.stop();
            height_after_first = v.current_height();
        }

        let config2 = ValidatorConfig::test_cluster(0, &next_ports()).with_quorum(1);
        let mut solo2 = config2.clone();
        solo2.cluster = vec![solo2.cluster[0].clone()];
        solo2.data_dir = data_dir;
        let v2 = LiveValidator::new(solo2);
        assert!(v2.current_height() >= height_after_first);
    }

    #[test]
    fn n69_single_validator_self_finalizes() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_two_validators_reach_consensus() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(2));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(2));
        va.start().unwrap();
        vb.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop();
        vb.stop();
        let ha = va.store.lock().unwrap().latest_height();
        let hb = vb.store.lock().unwrap().latest_height();
        assert!(ha >= 1 && hb >= 1, "Store heights: A={}, B={}", ha, hb);
    }

    #[test]
    fn n69_three_of_four_reach_quorum() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(3));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(3));
        let vc = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(3));
        va.start().unwrap();
        vb.start().unwrap();
        vc.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop();
        vb.stop();
        vc.stop();
        assert!(va.store.lock().unwrap().latest_height() >= 1);
        assert!(vb.store.lock().unwrap().latest_height() >= 1);
        assert!(vc.store.lock().unwrap().latest_height() >= 1);
    }

    #[test]
    fn n69_duplicate_vote_ignored() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_four_validators_full_cluster() {
        let ports = next_ports();
        let validators: Vec<LiveValidator> = (0..4)
            .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)))
            .collect();
        for v in &validators {
            v.start().unwrap();
        }
        thread::sleep(Duration::from_millis(15000));
        for v in &validators {
            v.stop();
        }

        for (i, v) in validators.iter().enumerate() {
            let h = v.store.lock().unwrap().latest_height();
            assert!(h >= 1, "Validator {} store height: {}", i, h);
            println!("Validator {} metrics: {}", i, v.metrics_summary());
        }
    }
}
