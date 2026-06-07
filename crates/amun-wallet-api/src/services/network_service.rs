use axum::Json;

use crate::errors::ApiResult;
use crate::types::NetworkInfoResponse;

pub struct NetworkService;

impl NetworkService {
    pub fn get_info() -> ApiResult<NetworkInfoResponse> {
        Ok(Json(NetworkInfoResponse {
            node_id: "amun-node-001".to_string(),
            chain_id: "amun-chain-1".to_string(),
            peer_count: 0,
            syncing: false,
        }))
    }
}
