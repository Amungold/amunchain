use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/faucet/request", post(faucet_request))
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
