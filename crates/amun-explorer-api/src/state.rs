use std::sync::Arc;

use crate::config::ExplorerConfig;
use crate::rpc::client::RpcClient;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ExplorerConfig>,
    pub rpc: Arc<RpcClient>,
}
