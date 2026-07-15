use crate::peer::{PeerInfo, PeerTable};
use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::PeerId;
use std::sync::Arc;

pub struct DiscoveryService {
    peer_table: Arc<PeerTable>,
    bootstrap_peers: Vec<String>,
}

impl DiscoveryService {
    pub fn new(peer_table: Arc<PeerTable>, bootstrap_peers: Vec<String>) -> Self {
        DiscoveryService {
            peer_table,
            bootstrap_peers,
        }
    }
    pub fn discover(&self) -> PlatformResult<usize> {
        for addr in &self.bootstrap_peers {
            let mut id = [0u8; 32];
            let b = addr.as_bytes();
            id[..b.len().min(32)].copy_from_slice(&b[..b.len().min(32)]);
            let pid = PeerId(id);
            if self.peer_table.get(&pid).is_none() {
                self.peer_table.add(PeerInfo::new(pid, addr.clone(), false));
            }
        }
        Ok(self.peer_table.count())
    }
}
