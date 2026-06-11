use crate::errors::{ApiError, ApiResult};
use crate::types::{BlockSummary, TransactionSummary};
use axum::Json;

pub struct ChainService;

impl ChainService {
    pub fn get_head() -> ApiResult<BlockSummary> {
        Ok(Json(BlockSummary {
            height: 847,
            hash: "0xdeadbeef".into(),
            previous_hash: "0xprevious".into(),
            state_root: "0xstate".into(),
            timestamp: 1700000000,
            transaction_count: 42,
            has_finality_certificate: true,
            has_replay_evidence: true,
        }))
    }

    pub fn get_block_by_height(height: u64) -> ApiResult<BlockSummary> {
        Ok(Json(BlockSummary {
            height,
            hash: format!("0xblock{:08x}", height),
            previous_hash: format!("0xblock{:08x}", height.saturating_sub(1)),
            state_root: "0xstate".into(),
            timestamp: 1700000000 + height,
            transaction_count: (height % 50) as usize,
            has_finality_certificate: true,
            has_replay_evidence: true,
        }))
    }

    pub fn get_block_by_hash(hash: &str) -> ApiResult<BlockSummary> {
        if hash.is_empty() {
            return Err(ApiError::not_found("Block", hash));
        }
        Self::get_block_by_height(847)
    }

    pub fn get_transaction(hash: &str) -> ApiResult<TransactionSummary> {
        if hash.is_empty() {
            return Err(ApiError::not_found("Transaction", hash));
        }
        Ok(Json(TransactionSummary {
            hash: hash.to_string(),
            block_height: 847,
            sender: "0xalice".into(),
            recipient: "0xbob".into(),
            amount: 1000,
            status: "confirmed".into(),
        }))
    }
}
