use crate::error::RpcError;
use crate::rpc::client::RpcClient;
use crate::rpc::models::head::{ChainHead, RpcHeadResponse};

impl RpcClient {
    /// Fetch the current chain head from the remote RPC endpoint.
    ///
    /// Calls `GET /head` and converts the raw DTO into a domain model.
    pub async fn get_head(&self) -> Result<ChainHead, RpcError> {
        let dto: RpcHeadResponse = self.get_json("head").await?;
        Ok(ChainHead::from(dto))
    }
}
