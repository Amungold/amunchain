use crate::types::*;

// ============================================================================
// NodeStateProvider trait (no dependencies on consensus/networking)
// ============================================================================

pub trait NodeStateProvider: Send + Sync {
    fn get_status(&self) -> Result<StatusResponse, RpcError>;
    fn get_head(&self) -> Result<HeadResponse, RpcError>;
    fn get_block(&self, height: u64) -> Result<BlockResponse, RpcError>;
    fn get_block_range(&self, from: u64, to: u64) -> Result<RangeResponse, RpcError>;
    fn get_metrics(&self) -> Result<MetricsResponse, RpcError>;
}
