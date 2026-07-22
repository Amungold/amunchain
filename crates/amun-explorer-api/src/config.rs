#[derive(Clone, Debug)]
pub struct ExplorerConfig {
    pub rpc_base_url: String,
    pub listen_addr: String,
}

impl ExplorerConfig {
    pub fn from_env() -> Self {
        Self {
            rpc_base_url: std::env::var("AMUN_RPC_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:9070".to_string()),
            listen_addr: std::env::var("AMUN_EXPLORER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:9080".to_string()),
        }
    }
}
