use amun_validator_api::types::id::PeerId;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub address: String,
    pub connected: bool,
    pub outbound: bool,
    pub last_seen: u64,
    pub last_ping: u64,
    pub reputation: i32,
    pub latency_ms: u64,
    pub protocol_version: u32,
    pub chain_id: Option<String>,
    pub capabilities: Vec<String>,
    pub ban_until: Option<u64>,
    pub failed_attempts: u32,
    pub successful_handshakes: u64,
    pub last_disconnect: Option<u64>,
    pub last_error: Option<String>,
}

impl PeerInfo {
    pub fn new(peer_id: PeerId, address: String, outbound: bool) -> Self {
        PeerInfo {
            peer_id,
            address,
            outbound,
            connected: false,
            last_seen: 0,
            last_ping: 0,
            reputation: 100,
            latency_ms: 0,
            protocol_version: 0,
            chain_id: None,
            capabilities: vec![],
            ban_until: None,
            failed_attempts: 0,
            successful_handshakes: 0,
            last_disconnect: None,
            last_error: None,
        }
    }
    pub fn is_banned(&self) -> bool {
        self.ban_until.is_some_and(|u| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                < u
        })
    }
}

pub struct PeerTable {
    peers: RwLock<HashMap<[u8; 32], PeerInfo>>,
}

impl Default for PeerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerTable {
    pub fn new() -> Self {
        PeerTable {
            peers: RwLock::new(HashMap::new()),
        }
    }
    pub fn add(&self, info: PeerInfo) {
        self.peers
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .insert(*info.peer_id.as_bytes(), info);
    }
    pub fn remove(&self, pid: &PeerId) {
        self.peers
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .remove(pid.as_bytes());
    }
    pub fn get(&self, pid: &PeerId) -> Option<PeerInfo> {
        self.peers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(pid.as_bytes())
            .cloned()
    }
    pub fn update(&self, pid: &PeerId, f: impl FnOnce(&mut PeerInfo)) {
        if let Some(info) = self
            .peers
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .get_mut(pid.as_bytes())
        {
            f(info);
        }
    }
    pub fn count(&self) -> usize {
        self.peers.read().unwrap_or_else(|e| e.into_inner()).len()
    }
    pub fn connected_peers(&self) -> Vec<PeerInfo> {
        self.peers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .values()
            .filter(|p| p.connected && !p.is_banned())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let t = PeerTable::new();
        t.add(PeerInfo::new(PeerId([1u8; 32]), "a".into(), true));
        assert_eq!(t.count(), 1);
    }
    #[test]
    fn test_ban() {
        let mut i = PeerInfo::new(PeerId([1u8; 32]), "a".into(), true);
        i.ban_until = Some(u64::MAX);
        assert!(i.is_banned());
    }
}
