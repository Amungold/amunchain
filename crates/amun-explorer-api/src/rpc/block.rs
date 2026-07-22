use crate::error::RpcError;
use crate::rpc::client::RpcClient;
use crate::rpc::models::block::{Block, RpcBlockResponse};

impl RpcClient {
    /// Fetch a specific block by height from the remote RPC endpoint.
    ///
    /// Calls `GET /block/{height}` and converts the raw DTO into a domain model.
    pub async fn get_block(&self, height: u64) -> Result<Block, RpcError> {
        let path = format!("block/{}", height);
        let dto: RpcBlockResponse = self.get_json(&path).await?;
        Ok(Block::from(dto))
    }
}
