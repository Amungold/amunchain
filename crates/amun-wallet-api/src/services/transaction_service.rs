use axum::Json;

use crate::errors::{ApiError, ApiResult};
use crate::types::{
    BuildTransactionRequest, BuildTransactionResponse, SubmitTransactionRequest,
    SubmitTransactionResponse, TransactionStatusResponse,
};

pub struct TransactionService;

impl TransactionService {
    pub fn build_transaction(req: BuildTransactionRequest) -> ApiResult<BuildTransactionResponse> {
        if req.sender.is_empty() || req.recipient.is_empty() {
            return Err(ApiError::new(
                "INVALID_REQUEST",
                "Sender or recipient is empty",
            ));
        }
        if req.amount == 0 {
            return Err(ApiError::new(
                "INVALID_REQUEST",
                "Amount must be greater than zero",
            ));
        }

        use amun_transactions::{Transaction, TransactionPayload, TransferPayload};
        let sender: [u8; 32] = hex::decode(&req.sender)
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Invalid sender hex"))?
            .try_into()
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Sender must be 32 bytes"))?;
        let recipient: [u8; 32] = hex::decode(&req.recipient)
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Invalid recipient hex"))?
            .try_into()
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Recipient must be 32 bytes"))?;

        let tx = Transaction {
            version: 1,
            sender,
            nonce: req.nonce,
            payload: TransactionPayload::Transfer(TransferPayload {
                to: recipient,
                amount: req.amount,
            }),
            signature: vec![],
        };

        let tx_hash = tx.tx_hash();
        let tx_bytes = serde_json::to_vec(&tx).unwrap_or_default();

        Ok(Json(BuildTransactionResponse {
            transaction_bytes: hex::encode(&tx_bytes),
            transaction_hash: hex::encode(tx_hash),
        }))
    }

    pub fn submit_transaction(
        req: SubmitTransactionRequest,
    ) -> ApiResult<SubmitTransactionResponse> {
        if req.transaction_bytes.is_empty() || req.signature.is_empty() {
            return Err(ApiError::new(
                "INVALID_REQUEST",
                "Missing transaction bytes or signature",
            ));
        }

        let tx_bytes = hex::decode(&req.transaction_bytes)
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Invalid hex in transaction_bytes"))?;
        let mut tx: amun_transactions::Transaction = serde_json::from_slice(&tx_bytes)
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Invalid transaction format"))?;

        let sig: Vec<u8> = hex::decode(&req.signature)
            .map_err(|_| ApiError::new("INVALID_REQUEST", "Invalid hex in signature"))?;
        tx.signature = sig;

        if !tx.verify() {
            return Err(ApiError::new(
                "INVALID_REQUEST",
                "Invalid transaction signature",
            ));
        }

        // TODO: Submit to real RPC endpoint when available
        let full_hash = hex::encode(tx.tx_hash());
        let short_hash = &full_hash[..16.min(full_hash.len())];
        let hash = format!("tx_{}", short_hash);

        Ok(Json(SubmitTransactionResponse { hash }))
    }

    pub fn get_transaction(hash: &str) -> ApiResult<TransactionStatusResponse> {
        if hash.is_empty() {
            return Err(ApiError::new(
                "INVALID_REQUEST",
                "Transaction hash is empty",
            ));
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
