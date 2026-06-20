pub mod client;
pub mod faucet;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
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
    let mut mp = state
        .mempool
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mp.add_transaction(tx)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

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
        .route("/api/status", get(api_status))
        .route("/api/blocks", get(api_blocks))
        .route("/api/block/:height", get(api_block_by_height))
        .route("/api/validators", get(api_validators))
        .route("/api/constitution/:height", get(api_constitution))
        .route("/wallet/create", post(wallet_create))
        .route("/wallet/:address/balance", get(wallet_balance))
        .route("/wallet/faucet", post(wallet_faucet))
        .route("/wallet/send", post(wallet_send))
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
            let power = engine.validator_powers.get(id).copied().unwrap_or(0);
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

async fn api_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let store = state.store.lock().unwrap();
    let tip = store.load_tip();
    let latest_height = store.latest_height();

    Json(serde_json::json!({
        "network": "AmunChain (NTR)",
        "latest_height": latest_height,
        "validators": engine.total_validators,
        "active_validators": engine.active_validator_count(),
        "total_voting_power": engine.total_voting_power,
        "constitutional": tip.map(|r| r.verdict_hash != [0u8; 32]).unwrap_or(false),
        "evidence_root": tip.map(|r| hex::encode(r.evidence_root)).unwrap_or_default(),
        "uptime_blocks": engine.metrics.blocks_finalized,
        "qcs_formed": engine.metrics.qcs_formed,
        "votes_received": engine.metrics.votes_received,
    }))
}

async fn api_blocks(State(state): State<AppState>) -> Json<serde_json::Value> {
    let store = state.store.lock().unwrap();
    let latest = store.latest_height();
    let start = if latest > 20 { latest - 20 } else { 1 };
    let mut blocks = Vec::new();

    for h in start..=latest {
        if let Some(r) = store.load_height(h) {
            blocks.push(serde_json::json!({
                "height": r.height,
                "block_hash": hex::encode(r.block_hash),
                "state_root": hex::encode(r.state_root),
                "evidence_root": hex::encode(r.evidence_root),
                "verdict_hash": hex::encode(r.verdict_hash),
                "evidence_record_hash": hex::encode(r.evidence_record_hash),
                "timestamp": r.timestamp,
            }));
        }
    }

    Json(serde_json::json!({
        "count": blocks.len(),
        "blocks": blocks
    }))
}

async fn api_block_by_height(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(r) => Ok(Json(serde_json::json!({
            "height": r.height,
            "block_hash": hex::encode(r.block_hash),
            "state_root": hex::encode(r.state_root),
            "history_root": hex::encode(r.history_root),
            "certificate_hash": hex::encode(r.certificate_hash),
            "slashing_root": hex::encode(r.slashing_root),
            "verdict_hash": hex::encode(r.verdict_hash),
            "evidence_record_hash": hex::encode(r.evidence_record_hash),
            "evidence_root": hex::encode(r.evidence_root),
            "timestamp": r.timestamp,
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn api_validators(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine = state.engine.lock().unwrap();
    let validators: Vec<serde_json::Value> = engine
        .validator_ids
        .iter()
        .map(|id| {
            let power = engine.validator_powers.get(id).copied().unwrap_or(0);
            serde_json::json!({
                "id": hex::encode(id),
                "voting_power": power,
                "active": power > 0
            })
        })
        .collect();

    Json(serde_json::json!({
        "total": validators.len(),
        "active": engine.active_validator_count(),
        "total_power": engine.total_voting_power,
        "validators": validators
    }))
}

async fn api_constitution(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = state.store.lock().unwrap();
    match store.load_height(height) {
        Some(r) => Ok(Json(serde_json::json!({
            "height": r.height,
            "verdict_hash": hex::encode(r.verdict_hash),
            "evidence_record_hash": hex::encode(r.evidence_record_hash),
            "evidence_root": hex::encode(r.evidence_root),
            "constitutional": r.verdict_hash != [0u8; 32],
            "chain_evidence_root": hex::encode(r.evidence_root),
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn wallet_create() -> Json<serde_json::Value> {
    let keypair = amun_wallet_management::keygen::generate_keypair();
    Json(serde_json::json!({
        "address": keypair.public_key_hex(),
        "public_key": keypair.public_key_hex(),
        "created": true
    }))
}

async fn wallet_balance(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Json<serde_json::Value> {
    let store = state.account_store.lock().unwrap();

    let mut addr = [0u8; 32];
    let addr_lower = address.to_lowercase();
    if let Ok(bytes) = hex::decode(&addr_lower) {
        let len = bytes.len().min(32);
        addr[..len].copy_from_slice(&bytes[..len]);
    }

    let balance = store.balance_of(&addr);
    let nonce = store.nonce_of(&addr);

    Json(serde_json::json!({
        "address": address,
        "balance": balance,
        "nonce": nonce
    }))
}

async fn wallet_faucet(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let address = req["address"].as_str().unwrap_or("");
    if address.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Address required".into()));
    }
    let amount = req["amount"].as_u64().unwrap_or(100_000);

    let mut faucet = state.faucet.lock().unwrap();
    let ip = "127.0.0.1".to_string();
    if let Err(e) = faucet.can_request(address, &ip) {
        return Err((StatusCode::TOO_MANY_REQUESTS, e));
    }

    let faucet_sender = [0xFA; 32];
    let mut store = state.account_store.lock().unwrap();

    // Ensure faucet has funds
    if store.balance_of(&faucet_sender) < amount {
        store.create_account(faucet_sender, 1_000_000_000);
    }

    // Parse recipient address (hex string to [u8;32])
    let mut recipient = [0u8; 32];
    if let Ok(bytes) = hex::decode(address) {
        let len = bytes.len().min(32);
        recipient[..len].copy_from_slice(&bytes[..len]);
    }
    store.create_account(recipient, amount);

    let tx_hash = format!(
        "faucet_{}",
        hex::encode(&blake3::hash(address.as_bytes()).as_bytes()[..8])
    );
    faucet.record(address.to_string(), ip, amount, tx_hash.clone());

    Ok(Json(serde_json::json!({
        "tx_hash": tx_hash,
        "amount": amount,
        "recipient": address
    })))
}

async fn wallet_send(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let sender = req["sender"].as_str().unwrap_or("");
    let recipient = req["recipient"].as_str().unwrap_or("");
    let amount = req["amount"].as_u64().unwrap_or(0);

    if sender.is_empty() || recipient.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Sender and recipient required".into(),
        ));
    }
    if amount == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Amount must be greater than 0".into(),
        ));
    }

    let mut store = state.account_store.lock().unwrap();

    // Parse sender address
    let mut sender_addr = [0u8; 32];
    if let Ok(bytes) = hex::decode(sender.to_lowercase()) {
        let len = bytes.len().min(32);
        sender_addr[..len].copy_from_slice(&bytes[..len]);
    }

    // Parse recipient address
    let mut recipient_addr = [0u8; 32];
    if let Ok(bytes) = hex::decode(recipient.to_lowercase()) {
        let len = bytes.len().min(32);
        recipient_addr[..len].copy_from_slice(&bytes[..len]);
    }

    // Check balance
    let sender_balance = store.balance_of(&sender_addr);
    if sender_balance < amount {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Insufficient balance: {} < {}", sender_balance, amount),
        ));
    }

    // Execute transfer
    let sender_nonce = store.nonce_of(&sender_addr);
    let tx_result = store.debit(&sender_addr, amount);
    if tx_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Debit failed".into()));
    }

    store.credit(&recipient_addr, amount);

    // Generate TX hash
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"AMUN_TX_V1");
    hasher.update(&sender_addr);
    hasher.update(&recipient_addr);
    hasher.update(&amount.to_le_bytes());
    hasher.update(&sender_nonce.to_le_bytes());
    let tx_hash = hex::encode(&hasher.finalize().as_bytes()[..16]);

    let sender_new_balance = store.balance_of(&sender_addr);
    let recipient_new_balance = store.balance_of(&recipient_addr);

    Ok(Json(serde_json::json!({
        "tx_hash": tx_hash,
        "sender": sender,
        "recipient": recipient,
        "amount": amount,
        "sender_balance_after": sender_new_balance,
        "recipient_balance_after": recipient_new_balance,
        "status": "confirmed"
    })))
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
