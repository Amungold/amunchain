#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub bootstrap_peers: Vec<String>,
    pub protocol_version: u32,
    pub chain_id: String,
    pub network_id: String,
    pub genesis_hash: [u8; 32],
    pub handshake_timeout_ms: u64,
    pub connect_timeout_ms: u64,
    pub heartbeat_interval_ms: u64,
    pub max_peers: usize,
    pub max_message_size: usize,
    pub max_frame_size: usize,
    pub idle_timeout_ms: u64,
    pub reconnect_backoff_ms: u64,
    pub max_reconnect_attempts: u32,
    pub ping_interval_ms: u64,
    pub pong_timeout_ms: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            listen_host: "0.0.0.0".into(),
            listen_port: 9000,
            bootstrap_peers: vec![],
            protocol_version: 1,
            chain_id: "amun-testnet-1".into(),
            network_id: "amun".into(),
            genesis_hash: [0u8; 32],
            handshake_timeout_ms: 5000,
            connect_timeout_ms: 10000,
            heartbeat_interval_ms: 30000,
            max_peers: 50,
            max_message_size: 1_048_576,
            max_frame_size: 1_100_000,
            idle_timeout_ms: 300000,
            reconnect_backoff_ms: 5000,
            max_reconnect_attempts: 5,
            ping_interval_ms: 15000,
            pong_timeout_ms: 5000,
        }
    }
}

impl NetworkConfig {
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.listen_host, self.listen_port)
    }
}
