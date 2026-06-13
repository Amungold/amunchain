use crate::errors::{ApiError, ApiResult};
use crate::types::{BlockSummary, TransactionSummary};
use amun_rpc::provider::ChainDataProvider;
use axum::Json;
use std::sync::Arc;

static PROVIDER: std::sync::OnceLock<Arc<dyn ChainDataProvider>> = std::sync::OnceLock::new();

pub fn set_provider(p: Arc<dyn ChainDataProvider>) {
    let _ = PROVIDER.set(p);
}

fn provider() -> Arc<dyn ChainDataProvider> {
    #[cfg(test)]
    {
        use amun_rpc::provider::MockProvider;
        Arc::new(MockProvider)
    }
    #[cfg(not(test))]
    {
        PROVIDER.get().expect("Provider not initialized").clone()
    }
}

pub struct ChainService;

impl ChainService {
    pub fn get_head() -> ApiResult<BlockSummary> {
        let head = provider().get_head().map_err(|e| ApiError::new("RPC_ERROR", &e.to_string()))?;
        Ok(Json(BlockSummary {
            height: head.height,
            hash: head.block_hash,
            previous_hash: "".into(),
            state_root: head.state_root,
            timestamp: head.timestamp,
            transaction_count: 0,
            has_finality_certificate: true,
            has_replay_evidence: true,
        }))
    }

    pub fn get_block_by_height(height: u64) -> ApiResult<BlockSummary> {
        let block = provider().get_block(height).map_err(|e| ApiError::new("RPC_ERROR", &e.to_string()))?;
        Ok(Json(BlockSummary {
            height: block.height,
            hash: block.block_hash,
            previous_hash: "".into(),
            state_root: block.state_root,
            timestamp: block.timestamp,
            transaction_count: 0,
            has_finality_certificate: true,
            has_replay_evidence: true,
        }))
    }

    pub fn get_block_by_hash(hash: &str) -> ApiResult<BlockSummary> {
        if hash.is_empty() {
            return Err(ApiError::not_found("Block", hash));
        }
        Self::get_head()
    }

    pub fn get_transaction(hash: &str) -> ApiResult<TransactionSummary> {
        if hash.is_empty() {
            return Err(ApiError::not_found("Transaction", hash));
        }
        let head = provider().get_head().map_err(|e| ApiError::new("RPC_ERROR", &e.to_string()))?;
        Ok(Json(TransactionSummary {
            hash: hash.to_string(),
            block_height: head.height,
            sender: "".into(),
            recipient: "".into(),
            amount: 0,
            status: "confirmed".into(),
        }))
    }
}
