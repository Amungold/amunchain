use crate::validator::LiveValidator;
use amun_rpc::client::{
    AccountResponse, BlockResponse, HeadResponse, MetricsResponse, RangeResponse, StatusResponse,
};
use amun_rpc::provider::ChainDataProvider;
use std::sync::Arc;

pub struct LiveValidatorProvider {
    pub validator: Arc<LiveValidator>,
}

impl ChainDataProvider for LiveValidatorProvider {
    fn get_status(&self) -> Result<StatusResponse, String> {
        let h = self.validator.current_height();
        let engine = self.validator.engine.lock().unwrap();
        Ok(StatusResponse {
            height: h,
            qcs_formed: engine.metrics.qcs_formed,
            blocks_finalized: engine.metrics.blocks_finalized,
            votes_received: engine.metrics.votes_received,
            peer_count: engine.total_validators,
        })
    }

    fn get_head(&self) -> Result<HeadResponse, String> {
        let store = self.validator.store.lock().unwrap();
        store
            .load_tip()
            .map(|r| HeadResponse {
                height: r.height,
                block_hash: hex::encode(r.block_hash),
                state_root: hex::encode(r.state_root),
                history_root: hex::encode(r.history_root),
                timestamp: r.timestamp,
            })
            .ok_or_else(|| "No blocks yet".into())
    }

    fn get_block(&self, height: u64) -> Result<BlockResponse, String> {
        let store = self.validator.store.lock().unwrap();
        store
            .load_height(height)
            .map(|r| BlockResponse {
                height: r.height,
                block_hash: hex::encode(r.block_hash),
                state_root: hex::encode(r.state_root),
                certificate_hash: hex::encode(r.certificate_hash),
                timestamp: r.timestamp,
            })
            .ok_or_else(|| format!("Block {} not found", height))
    }

    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, String> {
        let store = self.validator.store.lock().unwrap();
        let end = std::cmp::min(to, store.latest_height());
        let blocks: Vec<BlockResponse> = (from..=end)
            .filter_map(|h| store.load_height(h))
            .map(|r| BlockResponse {
                height: r.height,
                block_hash: hex::encode(r.block_hash),
                state_root: hex::encode(r.state_root),
                certificate_hash: hex::encode(r.certificate_hash),
                timestamp: r.timestamp,
            })
            .collect();
        Ok(RangeResponse { blocks })
    }

    fn get_metrics(&self) -> Result<MetricsResponse, String> {
        let status = self.get_status()?;
        let engine = self.validator.engine.lock().unwrap();
        Ok(MetricsResponse {
            height: status.height,
            qcs_formed: status.qcs_formed,
            blocks_finalized: status.blocks_finalized,
            votes_received: status.votes_received,
            rounds_active: engine.rounds.len(),
            peer_count: status.peer_count,
        })
    }

    fn submit_transaction(&self, _tx_json: &str) -> Result<String, String> {
        Err("submit_transaction not yet implemented for LiveValidator".into())
    }

    fn get_account(&self, address: &str) -> Result<AccountResponse, String> {
        Ok(AccountResponse {
            address: address.to_string(),
            balance: 0,
            nonce: 0,
        })
    }
}
