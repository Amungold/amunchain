use std::sync::{Arc, Mutex};
use amun_rpc::client::{StatusResponse, HeadResponse, BlockResponse, RangeResponse, MetricsResponse, AccountResponse};
use amun_rpc::provider::ChainDataProvider;
use amun_networking::node::NetworkNode;

pub struct NetworkNodeChainProvider {
    pub node: Arc<Mutex<NetworkNode>>,
}

impl ChainDataProvider for NetworkNodeChainProvider {
    fn get_status(&self) -> Result<StatusResponse, String> {
        let n = self.node.lock().unwrap();
        Ok(StatusResponse {
            height: n.current_height,
            qcs_formed: n.committed_blocks.len() as u64,
            blocks_finalized: n.committed_blocks.len() as u64,
            votes_received: 0,
            peer_count: 0,
        })
    }

    fn get_head(&self) -> Result<HeadResponse, String> {
        let n = self.node.lock().unwrap();
        if n.current_height == 0 {
            return Err("No blocks yet".into());
        }
        Ok(HeadResponse {
            height: n.current_height,
            block_hash: hex::encode([0u8; 32]),
            state_root: hex::encode([0u8; 32]),
            history_root: hex::encode([0u8; 32]),
            timestamp: 0,
        })
    }

    fn get_block(&self, height: u64) -> Result<BlockResponse, String> {
        let n = self.node.lock().unwrap();
        if height > n.current_height || height == 0 {
            return Err(format!("Block {} not found", height));
        }
        Ok(BlockResponse {
            height,
            block_hash: hex::encode([height as u8; 32]),
            state_root: hex::encode([0u8; 32]),
            certificate_hash: hex::encode([0u8; 32]),
            timestamp: 0,
        })
    }

    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, String> {
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

    fn get_metrics(&self) -> Result<MetricsResponse, String> {
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

    fn submit_transaction(&self, _tx_json: &str) -> Result<String, String> {
        Err("Not supported via NetworkNode".into())
    }

    fn get_account(&self, address: &str) -> Result<AccountResponse, String> {
        Ok(AccountResponse {
            address: address.to_string(),
            balance: 0,
            nonce: 0,
        })
    }
}
