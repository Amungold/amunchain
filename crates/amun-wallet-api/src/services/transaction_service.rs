use axum::Json;
use blake3::Hasher;

use crate::errors::{ApiError, ApiResult};
use crate::types::{
    BuildTransactionRequest, BuildTransactionResponse, SubmitTransactionRequest,
    SubmitTransactionResponse, TransactionStatusResponse,
};

pub struct TransactionService;

impl TransactionService {
    pub fn build_transaction(req: BuildTransactionRequest) -> ApiResult<BuildTransactionResponse> {
        if req.sender.is_empty() || req.recipient.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Sender or recipient is empty"));
        }
        if req.amount == 0 {
            return Err(ApiError::new("INVALID_REQUEST", "Amount must be greater than zero"));
        }

        let mut hasher = Hasher::new();
        hasher.update(req.sender.as_bytes());
        hasher.update(req.recipient.as_bytes());
        hasher.update(&req.amount.to_le_bytes());
        hasher.update(&req.nonce.to_le_bytes());
        let tx_hash = hex::encode(hasher.finalize().as_bytes());

        let tx_bytes = serde_json::to_vec(&req).unwrap_or_default();
        let tx_bytes_encoded = hex::encode(&tx_bytes);

        Ok(Json(BuildTransactionResponse {
            transaction_bytes: tx_bytes_encoded,
            transaction_hash: tx_hash,
        }))
    }

    pub fn submit_transaction(
        req: SubmitTransactionRequest,
    ) -> ApiResult<SubmitTransactionResponse> {
        if req.transaction_bytes.is_empty() || req.signature.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Missing transaction bytes or signature"));
        }

        let mut hasher = Hasher::new();
        hasher.update(req.transaction_bytes.as_bytes());
        hasher.update(req.signature.as_bytes());
        let hash = hex::encode(hasher.finalize().as_bytes());

        Ok(Json(SubmitTransactionResponse { hash }))
    }

    pub fn get_transaction(hash: &str) -> ApiResult<TransactionStatusResponse> {
        if hash.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Transaction hash is empty"));
        }

        Ok(Json(TransactionStatusResponse {
            hash: hash.to_string(),
            status: "pending".to_string(),
            block_height: None,
            timestamp: None,
            error: None,
        }))
    }

    pub fn get_transaction_status(hash: &str) -> ApiResult<TransactionStatusResponse> {
        Self::get_transaction(hash)
    }
}
