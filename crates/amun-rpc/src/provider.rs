use crate::client::{RpcClient, StatusResponse, HeadResponse, BlockResponse, RangeResponse, MetricsResponse};

pub trait ChainDataProvider: Send + Sync {
    fn get_status(&self) -> Result<StatusResponse, String>;
    fn get_head(&self) -> Result<HeadResponse, String>;
    fn get_block(&self, height: u64) -> Result<BlockResponse, String>;
    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, String>;
    fn get_metrics(&self) -> Result<MetricsResponse, String>;
}

pub struct LiveRpcProvider {
    client: RpcClient,
}

impl LiveRpcProvider {
    pub fn new(host: &str, port: u16) -> Self {
        LiveRpcProvider { client: RpcClient::new(host, port) }
    }
}

impl ChainDataProvider for LiveRpcProvider {
    fn get_status(&self) -> Result<StatusResponse, String> { self.client.get_status() }
    fn get_head(&self) -> Result<HeadResponse, String> { self.client.get_head() }
    fn get_block(&self, height: u64) -> Result<BlockResponse, String> { self.client.get_block(height) }
    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, String> { self.client.get_block_range(from, to) }
    fn get_metrics(&self) -> Result<MetricsResponse, String> { self.client.get_metrics() }
}

pub struct MockProvider;

impl ChainDataProvider for MockProvider {
    fn get_status(&self) -> Result<StatusResponse, String> {
        Ok(StatusResponse { height: 847, qcs_formed: 847, blocks_finalized: 847, votes_received: 1, peer_count: 4 })
    }
    fn get_head(&self) -> Result<HeadResponse, String> {
        Ok(HeadResponse { height: 847, block_hash: "0xdeadbeef".into(), state_root: "0xstate".into(), history_root: "0xhistory".into(), timestamp: 1700000000 })
    }
    fn get_block(&self, height: u64) -> Result<BlockResponse, String> {
        Ok(BlockResponse { height, block_hash: format!("0xblock{:08x}", height), state_root: "0xstate".into(), certificate_hash: "0xcert".into(), timestamp: 1700000000 + height })
    }
    fn get_block_range(&self, _from: u64, _to: u64) -> Result<RangeResponse, String> {
        Ok(RangeResponse { blocks: vec![] })
    }
    fn get_metrics(&self) -> Result<MetricsResponse, String> {
        Ok(MetricsResponse { height: 847, qcs_formed: 847, blocks_finalized: 847, votes_received: 1, rounds_active: 1, peer_count: 4 })
    }
}
