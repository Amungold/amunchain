// N122.5 — Faucet Service
// ========================
// Distributes test tokens to new users with rate limiting.

use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetRequest {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetResponse {
    pub address: String,
    pub amount: u64,
    pub tx_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct FaucetState {
    /// Per-address: count of requests
    pub address_requests: HashMap<String, u64>,
    /// Per-IP: count of requests
    pub ip_requests: HashMap<String, u64>,
    /// Audit log
    pub audit_log: Vec<FaucetAuditEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FaucetAuditEntry {
    pub timestamp: u64,
    pub address: String,
    pub ip: String,
    pub amount: u64,
    pub tx_hash: String,
}

impl FaucetState {
    const MAX_PER_ADDRESS: u64 = 5;
    const MAX_PER_IP: u64 = 10;
    const FAUCET_AMOUNT: u64 = 100_000;

    pub fn can_request(&self, address: &str, ip: &str) -> Result<(), String> {
        let addr_count = self.address_requests.get(address).copied().unwrap_or(0);
        if addr_count >= Self::MAX_PER_ADDRESS {
            return Err(format!(
                "Address limit reached ({}/{})",
                addr_count,
                Self::MAX_PER_ADDRESS
            ));
        }
        let ip_count = self.ip_requests.get(ip).copied().unwrap_or(0);
        if ip_count >= Self::MAX_PER_IP {
            return Err(format!(
                "IP limit reached ({}/{})",
                ip_count,
                Self::MAX_PER_IP
            ));
        }
        Ok(())
    }

    pub fn record(&mut self, address: String, ip: String, amount: u64, tx_hash: String) {
        *self.address_requests.entry(address.clone()).or_insert(0) += 1;
        *self.ip_requests.entry(ip.clone()).or_insert(0) += 1;
        self.audit_log.push(FaucetAuditEntry {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            address,
            ip,
            amount,
            tx_hash,
        });
    }
}

#[derive(Clone)]
pub struct FaucetAppState {
    pub faucet: std::sync::Arc<Mutex<FaucetState>>,
    pub account_store: std::sync::Arc<std::sync::Mutex<amun_accounts::AccountStore>>,
    pub mempool: std::sync::Arc<std::sync::Mutex<amun_mempool::Mempool>>,
}

pub fn faucet_routes() -> Router<FaucetAppState> {
    Router::new().route("/faucet/request", post(handle_faucet_request))
}

async fn handle_faucet_request(
    State(state): State<FaucetAppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<FaucetRequest>,
) -> Result<Json<FaucetResponse>, (StatusCode, String)> {
    let ip = addr.ip().to_string();

    // Validate address
    let addr_bytes = hex::decode(&req.address)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid hex address".into()))?;
    if addr_bytes.len() != 32 {
        return Err((StatusCode::BAD_REQUEST, "Address must be 32 bytes".into()));
    }
    let mut recipient = [0u8; 32];
    recipient.copy_from_slice(&addr_bytes);

    // Rate limiting
    {
        let faucet = state.faucet.lock().unwrap();
        faucet
            .can_request(&req.address, &ip)
            .map_err(|e| (StatusCode::TOO_MANY_REQUESTS, e))?;
    }

    // Create a faucet transfer transaction
    use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
    let faucet_sender = [0xFA; 32]; // Well-known faucet address
    let tx = Transaction {
        version: 1,
        sender: faucet_sender,
        nonce: 0,
        payload: TransactionPayload::Transfer(TransferPayload {
            to: recipient,
            amount: FaucetState::FAUCET_AMOUNT,
        }),
        signature: vec![0xFA; 64], // Faucet signature
    };

    // Ensure faucet account exists with sufficient balance
    {
        let mut store = state.account_store.lock().unwrap();
        if store.balance_of(&faucet_sender) < FaucetState::FAUCET_AMOUNT {
            store.create_account(faucet_sender, 1_000_000_000);
        }
        // Credit the recipient
        store.create_account(recipient, FaucetState::FAUCET_AMOUNT);
    }

    // Add to mempool
    let tx_hash = tx.tx_hash();
    let tx_hash_hex = hex::encode(tx_hash);
    {
        let mut mp = state.mempool.lock().unwrap();
        let _ = mp.add_transaction(tx);
    }

    // Record the request
    {
        let mut faucet = state.faucet.lock().unwrap();
        faucet.record(
            req.address.clone(),
            ip,
            FaucetState::FAUCET_AMOUNT,
            tx_hash_hex.clone(),
        );
    }

    Ok(Json(FaucetResponse {
        address: req.address,
        amount: FaucetState::FAUCET_AMOUNT,
        tx_hash: tx_hash_hex,
        message: "Tokens sent! Check balance in next block.".into(),
    }))
}
