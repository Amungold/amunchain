use crate::types::{RpcRequest, RpcResponse};
use crate::auth::AuthValidator;

pub struct RpcHandler {
    auth: AuthValidator,
    chain_id: u64,
}

impl RpcHandler {
    pub fn new(chain_id: u64, auth_token: Option<&str>) -> Self {
        Self {
            auth: AuthValidator::new(auth_token),
            chain_id,
        }
    }

    pub fn handle(&self, request: &RpcRequest) -> RpcResponse {
        if self.auth.requires_auth() {
            if let serde_json::Value::Object(ref params) = request.params {
                let token = params.get("auth_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !self.auth.validate(token) {
                    return RpcResponse::error(request.id, -32001, "Unauthorized");
                }
            } else {
                return RpcResponse::error(request.id, -32001, "Unauthorized");
            }
        }

        match request.method.as_str() {
            "chain_id" => {
                RpcResponse::success(request.id, serde_json::json!({
                    "chain_id": self.chain_id
                }))
            }
            "block_height" => {
                RpcResponse::success(request.id, serde_json::json!({
                    "height": 0
                }))
            }
            "health" => {
                RpcResponse::success(request.id, serde_json::json!({
                    "status": "ok"
                }))
            }
            _ => {
                RpcResponse::error(request.id, -32601, "Method not found")
            }
        }
    }
}
