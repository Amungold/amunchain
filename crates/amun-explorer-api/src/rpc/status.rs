use crate::error::RpcError;
use crate::rpc::client::RpcClient;
use crate::rpc::models::status::{NodeStatus, RpcStatusResponse};

impl RpcClient {
    /// Fetch node status from the remote RPC endpoint.
    ///
    /// Calls `GET /status` and converts the raw DTO into a domain model.
    pub async fn get_status(&self) -> Result<NodeStatus, RpcError> {
        let dto: RpcStatusResponse = self.get_json("status").await?;
        Ok(NodeStatus::from(dto))
    }
}
