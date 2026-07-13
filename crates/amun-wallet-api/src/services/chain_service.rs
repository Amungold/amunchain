use axum::Json;

use crate::errors::{ApiError, ApiResult};
use crate::types::{BlockResponse, ChainHeadResponse};

pub struct ChainService;

impl ChainService {
    pub fn get_head() -> ApiResult<ChainHeadResponse> {
        Ok(Json(ChainHeadResponse {
            height: 0,
            hash: "0x0000".to_string(),
            state_root: "0x0000".to_string(),
            timestamp: 0,
        }))
    }

    pub fn get_block_by_height(height: u64) -> ApiResult<BlockResponse> {
        Ok(Json(BlockResponse {
            height,
            hash: format!("0x{:016x}", height),
            previous_hash: "0x0000".to_string(),
            state_root: "0x0000".to_string(),
            timestamp: 0,
            transaction_count: 0,
        }))
    }

    pub fn get_block_by_hash(hash: &str) -> ApiResult<BlockResponse> {
        if hash.is_empty() {
            return Err(ApiError::new("INVALID_REQUEST", "Block hash is empty"));
        }
        Self::get_block_by_height(0)
    }
}
