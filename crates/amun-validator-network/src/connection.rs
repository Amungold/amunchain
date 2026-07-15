use crate::config::NetworkConfig;
use crate::peer::{PeerInfo, PeerTable};
use amun_validator_api::error::{NetworkError, NetworkErrorCode, PlatformError, PlatformResult};
use amun_validator_api::types::id::PeerId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ConnectionManager {
    peer_table: Arc<PeerTable>,
    config: NetworkConfig,
    connections: Mutex<HashMap<[u8; 32], ConnectionState>>,
}
#[allow(dead_code)]
struct ConnectionState {
    connected_at: u64,
    bytes_sent: u64,
    bytes_received: u64,
}

impl ConnectionManager {
    pub fn new(peer_table: Arc<PeerTable>, config: NetworkConfig) -> Self {
        ConnectionManager {
            peer_table,
            config,
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn connect(&self, peer_id: &PeerId, address: &str) -> PlatformResult<()> {
        if self.peer_table.connected_peers().len() >= self.config.max_peers {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                format!("Max peers {}", self.config.max_peers),
            )));
        }
        self.peer_table
            .add(PeerInfo::new(*peer_id, address.to_string(), true));
        self.peer_table.update(peer_id, |info| {
            info.connected = true;
            info.last_seen = Self::now();
        });
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(
                *peer_id.as_bytes(),
                ConnectionState {
                    connected_at: Self::now(),
                    bytes_sent: 0,
                    bytes_received: 0,
                },
            );
        Ok(())
    }

    pub fn disconnect(&self, peer_id: &PeerId) {
        self.peer_table.update(peer_id, |info| {
            info.connected = false;
            info.last_disconnect = Some(Self::now());
        });
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(peer_id.as_bytes());
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max() {
        let t = Arc::new(PeerTable::new());
        let c = NetworkConfig { max_peers: 0, ..NetworkConfig::default() };
        assert!(ConnectionManager::new(t, c)
            .connect(&PeerId([1u8; 32]), "a")
            .is_err());
    }
}
