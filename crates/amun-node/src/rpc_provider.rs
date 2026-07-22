use std::sync::{Arc, Mutex};
use amun_rpc::node_provider::NodeStateProvider;
use amun_rpc::types::*;
use amun_networking::node::NetworkNode;

pub struct NetworkNodeProvider {
    pub node: Arc<Mutex<NetworkNode>>,
}

impl NodeStateProvider for NetworkNodeProvider {
    fn get_status(&self) -> Result<StatusResponse, RpcError> {
        let n = self.node.lock().unwrap();
        Ok(StatusResponse {
            height: n.current_height,
            qcs_formed: n.committed_blocks.len() as u64,
            blocks_finalized: n.committed_blocks.len() as u64,
            votes_received: 0,
            peer_count: 0,
        })
    }

    fn get_head(&self) -> Result<HeadResponse, RpcError> {
        let n = self.node.lock().unwrap();
        if n.current_height == 0 { return Err(RpcError::NotFound); }
        Ok(HeadResponse {
            height: n.current_height,
            block_hash: hex::encode([0u8; 32]),
            state_root: hex::encode([0u8; 32]),
            history_root: hex::encode([0u8; 32]),
            timestamp: 0,
        })
    }

    fn get_block(&self, height: u64) -> Result<BlockResponse, RpcError> {
        let n = self.node.lock().unwrap();
        if height > n.current_height || height == 0 { return Err(RpcError::NotFound); }
        Ok(BlockResponse {
            height,
            block_hash: hex::encode([height as u8; 32]),
            state_root: hex::encode([0u8; 32]),
            certificate_hash: hex::encode([0u8; 32]),
            timestamp: 0,
        })
    }

    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, RpcError> {
        let n = self.node.lock().unwrap();
        let end = std::cmp::min(to, n.current_height);
        let blocks: Vec<BlockResponse> = (from..=end)
            .map(|h| BlockResponse {
                height: h,
                block_hash: hex::encode([h as u8; 32]),
                state_root: hex::encode([0u8; 32]),
                certificate_hash: hex::encode([0u8; 32]),
                timestamp: 0,
            })
            .collect();
        Ok(RangeResponse { blocks })
    }

    fn get_metrics(&self) -> Result<MetricsResponse, RpcError> {
        let status = self.get_status()?;
        Ok(MetricsResponse {
            height: status.height,
            qcs_formed: status.qcs_formed,
            blocks_finalized: status.blocks_finalized,
            votes_received: status.votes_received,
            rounds_active: 1,
            peer_count: status.peer_count,
        })
    }
}
