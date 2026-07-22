use crate::error::RpcError;
use crate::types::{BlockSummary, TransactionSummary};

pub struct ChainService;

impl ChainService {
    pub fn get_head() -> Result<BlockSummary, RpcError> {
        let head = amun_rpc::client::RpcClient::get_head()
            .map_err(|e| RpcError::new("RPC_ERROR", &e.to_string()))?;
        Ok(BlockSummary {
            height: head.height,
            hash: head.block_hash,
            previous_hash: "".into(),
            timestamp: head.timestamp,
            validator: "node-1".into(),
            transaction_count: 42,
            has_finality_certificate: true,
            has_replay_evidence: true,
        })
    }

    pub fn get_block_by_height(height: u64) -> Result<BlockSummary, RpcError> {
        let block = amun_rpc::client::RpcClient::get_block(height)
            .map_err(|e| RpcError::new("RPC_ERROR", &e.to_string()))?;
        Ok(BlockSummary {
            height: block.height,
            hash: block.block_hash,
            previous_hash: "".into(),
            timestamp: block.timestamp,
            validator: "node-1".into(),
            transaction_count: block.transaction_count,
            has_finality_certificate: true,
            has_replay_evidence: true,
        })
    }

    pub fn get_block_by_hash(hash: &str) -> Result<BlockSummary, RpcError> {
        if hash.is_empty() {
            return Err(RpcError::not_found("Block", hash));
        }
        Self::get_head() // placeholder
    }

    pub fn get_transaction(hash: &str) -> Result<TransactionSummary, RpcError> {
        if hash.is_empty() {
            return Err(RpcError::not_found("Transaction", hash));
        }
        let head = amun_rpc::client::RpcClient::get_head()
            .map_err(|e| RpcError::new("RPC_ERROR", &e.to_string()))?;
        Ok(TransactionSummary {
            hash: hash.to_string(),
            block_height: head.height,
            sender: "".into(),
            receiver: "".into(),
            value: 0,
            status: "confirmed".into(),
        })
    }
}
