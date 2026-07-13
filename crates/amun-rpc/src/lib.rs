pub mod client;
pub mod faucet;
pub mod network;
use amun_authority_registry::transaction::GovernanceState;
use amun_authority_registry::AuthorityRegistry;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::{CertificateGossip, SlashingLedger};
use amun_constitutional_enforcement::ConstitutionalEnforcementKernel;
use amun_tokenomics_ledger::EconomicLedger;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<ChainStore>>,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub mempool: Arc<Mutex<amun_mempool::Mempool>>,
    pub faucet: Arc<Mutex<crate::faucet::FaucetState>>,
    pub account_store: Arc<Mutex<amun_accounts::AccountStore>>,
    pub governance: Arc<Mutex<GovernanceState>>,
    pub authority_registry: Arc<Mutex<AuthorityRegistry>>,
    pub constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    pub certificate_gossip: Arc<Mutex<CertificateGossip>>,
    pub slashing_ledger: Arc<Mutex<SlashingLedger>>,
    pub economic_ledger: Arc<Mutex<EconomicLedger>>,
    pub previous_evidence_root: Arc<Mutex<[u8; 32]>>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub peer_count: usize,
}

#[derive(Serialize)]
pub struct HeadResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub history_root: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct BlockResponse {
    pub height: u64,
    pub block_hash: String,
    pub state_root: String,
    pub certificate_hash: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct RangeResponse {
    pub blocks: Vec<BlockResponse>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub height: u64,
    pub qcs_formed: u64,
    pub blocks_finalized: u64,
    pub votes_received: u64,
    pub rounds_active: usize,
    pub peer_count: usize,
}

async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    let engine = state.engine.lock().unwrap();
    Json(StatusResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        peer_count: engine.total_validators,
    })
}

async fn head(State(state): State<AppState>) -> Result<Json<HeadResponse>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_tip() {
        Some(record) => Ok(Json(HeadResponse {
            height: record.height,
            block_hash: hex::encode(record.block_hash),
            state_root: hex::encode(record.state_root),
            history_root: hex::encode(record.history_root),
            timestamp: record.timestamp,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn block(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<BlockResponse>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(BlockResponse {
            height: record.height,
            block_hash: hex::encode(record.block_hash),
            state_root: hex::encode(record.state_root),
            certificate_hash: hex::encode(record.certificate_hash),
            timestamp: record.timestamp,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn block_range(
    State(state): State<AppState>,
    Path((from, to)): Path<(u64, u64)>,
) -> Json<RangeResponse> {
    let store = state.store.lock().unwrap();
    let end = std::cmp::min(to, store.latest_height());
    let start = std::cmp::max(from, 1);
    let blocks: Vec<BlockResponse> = (start..=end)
        .filter_map(|h| store.load_height(h))
        .map(|r| BlockResponse {
            height: r.height,
            block_hash: hex::encode(r.block_hash),
            state_root: hex::encode(r.state_root),
            certificate_hash: hex::encode(r.certificate_hash),
            timestamp: r.timestamp,
        })
        .collect();
    Json(RangeResponse { blocks })
}

async fn metrics(State(state): State<AppState>) -> Json<MetricsResponse> {
    let engine = state.engine.lock().unwrap();
    Json(MetricsResponse {
        height: engine.current_height,
        qcs_formed: engine.metrics.qcs_formed,
        blocks_finalized: engine.metrics.blocks_finalized,
        votes_received: engine.metrics.votes_received,
        rounds_active: engine.rounds.len(),
        peer_count: engine.total_validators,
    })
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    eprintln!("########## RPC SUBMIT_TX HIT ##########");
    eprintln!("[RPC] submit_tx called");
    let tx_bytes = hex::decode(body["transaction_bytes"].as_str().unwrap_or(""))
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut tx: amun_transactions::Transaction =
        serde_json::from_slice(&tx_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(sig_hex) = body["signature"].as_str() {
        if let Ok(sig) = hex::decode(sig_hex) {
            if sig.len() == 64 {
                tx.signature = sig;
            }
        }
    }

    if !tx.verify() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hash = tx.tx_hash();

    eprintln!("[RPC] tx={} nonce={}", hex::encode(hash), tx.nonce);
    let mut mp = state
        .mempool
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    eprintln!("[RPC] mempool before={}", mp.pending_count());
    mp.add_transaction(tx)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    eprintln!("[RPC] mempool after={}", mp.pending_count());

    Ok(Json(serde_json::json!({
        "hash": hex::encode(hash),
        "status": "pending"
    })))
}

async fn mempool_count(State(state): State<AppState>) -> Json<serde_json::Value> {
    let count = state.mempool.lock().unwrap().pending_count();
    Json(serde_json::json!({
        "pending_transactions": count
    }))
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(network::router())
        .route("/status", get(status))
        .route("/head", get(head))
        .route("/block/:height", get(block))
        .route("/blocks/:from/:to", get(block_range))
        .route("/metrics", get(metrics))
        .route("/mempool/count", get(mempool_count))
        .route("/tx/submit", post(submit_tx))
        .route("/faucet/request", post(faucet_request))
        .route("/explorer/summary", get(explorer_summary))
        .route("/explorer/validators", get(explorer_validators))
        .route("/explorer/blocks/:height", get(explorer_block_detail))
        .route("/constitutional/status", get(constitutional_status))
        .with_state(state)
}

async fn faucet_request(
    State(state): State<AppState>,
    Json(req): Json<crate::faucet::FaucetRequest>,
) -> Result<Json<crate::faucet::FaucetResponse>, (StatusCode, String)> {
    use amun_transactions::{Transaction, TransactionPayload, TransferPayload};

    let ip = "0.0.0.0".to_string(); // N122.5: IP tracking via proxy header in production
    let addr_bytes =
        hex::decode(&req.address).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid hex".into()))?;
    if addr_bytes.len() != 32 {
        return Err((StatusCode::BAD_REQUEST, "Address must be 32 bytes".into()));
    }
    let mut recipient = [0u8; 32];
    recipient.copy_from_slice(&addr_bytes);

    // Rate limiting
    {
        let faucet = state.faucet.lock().unwrap();
        if let Err(e) = faucet.can_request(&req.address, &ip) {
            return Err((StatusCode::TOO_MANY_REQUESTS, e));
        }
    }

    let faucet_sender = [0xFA; 32];
    let amount = 100_000u64;

    // Ensure faucet has funds and credit recipient
    {
        let mut store = state.account_store.lock().unwrap();
        if store.balance_of(&faucet_sender) < amount {
            store.create_account(faucet_sender, 1_000_000_000);
        }
        store.create_account(recipient, amount);
    }

    // Create and submit faucet transaction with unique nonce
    let nonce = {
        let faucet = state.faucet.lock().unwrap();
        faucet.audit_log.len() as u64 + 1
    };
    let tx = Transaction {
        version: 1,
        sender: faucet_sender,
        nonce,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: recipient,
            amount,
        }),
        signature: vec![0xFA; 64],
    };
    let tx_hash = hex::encode(tx.tx_hash());
    {
        let mut mp = state.mempool.lock().unwrap();
        let _ = mp.add_transaction(tx);
    }

    // Record
    {
        let mut faucet = state.faucet.lock().unwrap();
        faucet.record(req.address.clone(), ip, amount, tx_hash.clone());
    }

    Ok(Json(crate::faucet::FaucetResponse {
        address: req.address,
        amount,
        tx_hash,
        message: "Tokens sent!".into(),
    }))
}

async fn explorer_summary(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let store = state.store.lock().unwrap();
    let mempool = state.mempool.lock().unwrap();

    Json(serde_json::json!({
        "height": engine.current_height,
        "validators": engine.total_validators,
        "qcs_formed": engine.metrics.qcs_formed,
        "blocks_finalized": engine.metrics.blocks_finalized,
        "votes_received": engine.metrics.votes_received,
        "pending_transactions": mempool.pending_count(),
        "chain_head": {
            "hash": store.load_tip().map(|r| hex::encode(r.block_hash)).unwrap_or_default(),
            "state_root": store.load_tip().map(|r| hex::encode(r.state_root)).unwrap_or_default(),
        }
    }))
}

async fn explorer_validators(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let validators: Vec<serde_json::Value> = engine
        .validator_ids
        .iter()
        .map(|id| {
            let power = engine.get_validator_voting_power(id);
            serde_json::json!({
                "id": hex::encode(id),
                "voting_power": power,
                "active": power > 0
            })
        })
        .collect();

    Json(serde_json::json!({
        "total": validators.len(),
        "validators": validators
    }))
}

async fn explorer_block_detail(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(record) => Ok(Json(serde_json::json!({
            "height": record.height,
            "block_hash": hex::encode(record.block_hash),
            "state_root": hex::encode(record.state_root),
            "history_root": hex::encode(record.history_root),
            "timestamp": record.timestamp,
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn constitutional_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let governance = state.governance.lock().unwrap();
    let authority = state.authority_registry.lock().unwrap();
    let kernel = state.constitutional_kernel.lock().unwrap();
    let slashing = state.slashing_ledger.lock().unwrap();
    let gossip = state.certificate_gossip.lock().unwrap();
    let economic = state.economic_ledger.lock().unwrap();
    let evidence_root = state.previous_evidence_root.lock().unwrap();

    Json(serde_json::json!({
        "consensus": {
            "height": engine.current_height,
            "validators": engine.total_validators,
            "active_validators": engine.active_validator_count(),
            "qcs_formed": engine.metrics.qcs_formed,
            "blocks_finalized": engine.metrics.blocks_finalized,
            "votes_received": engine.metrics.votes_received,
        },
        "governance": {
            "proposals": governance.proposals.len(),
            "votes": governance.votes.len()
        },
        "authority_registry": {
            "active_version": authority.active().map(|a| a.authority_version),
            "transition_pending": authority.transition.is_some()
        },
        "constitutional_kernel": {
            "constitutional_blocks": kernel.constitutional_count,
            "unconstitutional_blocks": kernel.unconstitutional_count,
            "compliance_ratio": kernel.compliance_ratio(),
            "active_laws": kernel.active_laws.len(),
            "verdict_history": kernel.verdict_history.len()
        },
        "slashing": {
            "executed": slashing.executed_count()
        },
        "certificate_gossip": {
            "pending": gossip.len()
        },
        "economic": {
            "treasury": economic.treasury(),
            "validator_pool": economic.validator_pool(),
            "ecosystem_pool": economic.ecosystem_pool(),
            "issued_supply": economic.issued_supply(),
            "burned_supply": economic.burned_supply(),
            "staked_supply": economic.staked_supply(),
            "economic_root": hex::encode(economic.compute_economic_root())
        },
        "evidence": {
            "previous_root": hex::encode(*evidence_root)
        }
    }))
}

pub async fn serve(state: AppState, port: u16) {
    let app = build_app(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    eprintln!("RPC server listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
pub mod provider;
