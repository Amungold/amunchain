use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Status of a known peer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerStatus {
    Active,
    Inactive,
    Removed,
}

/// A record of a known peer in the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRecord {
    pub node_id: [u8; 32],
    pub address: SocketAddr,
    pub last_seen: u64,
    pub chain_height: u64,
    pub status: PeerStatus,
}

/// Local peer table: tracks all known peers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeerTable {
    peers: HashMap<[u8; 32], PeerRecord>,
    max_peers: usize,
}

impl PeerTable {
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: HashMap::new(),
            max_peers,
        }
    }

    /// Add or update a peer.
    pub fn upsert(
        &mut self,
        node_id: [u8; 32],
        address: SocketAddr,
        chain_height: u64,
    ) -> Result<(), String> {
        if self.peers.len() >= self.max_peers && !self.peers.contains_key(&node_id) {
            return Err("Peer table full".into());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.peers
            .entry(node_id)
            .and_modify(|p| {
                p.address = address;
                p.last_seen = now;
                p.chain_height = chain_height;
                p.status = PeerStatus::Active;
            })
            .or_insert(PeerRecord {
                node_id,
                address,
                last_seen: now,
                chain_height,
                status: PeerStatus::Active,
            });
        Ok(())
    }

    /// Remove a peer.
    pub fn remove(&mut self, node_id: &[u8; 32]) {
        if let Some(peer) = self.peers.get_mut(node_id) {
            peer.status = PeerStatus::Removed;
        }
    }

    /// Mark stale peers as inactive (no contact for `timeout_secs`).
    pub fn expire_stale(&mut self, timeout_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for peer in self.peers.values_mut() {
            if peer.status == PeerStatus::Active && now - peer.last_seen > timeout_secs {
                peer.status = PeerStatus::Inactive;
            }
        }
    }

    /// Get active peers.
    pub fn active_peers(&self) -> Vec<&PeerRecord> {
        self.peers
            .values()
            .filter(|p| p.status == PeerStatus::Active)
            .collect()
    }

    /// Get all peer addresses (for exchange).
    pub fn peer_addresses(&self) -> Vec<(SocketAddr, u64)> {
        self.active_peers()
            .iter()
            .map(|p| (p.address, p.chain_height))
            .collect()
    }

    /// Get a specific peer.
    pub fn get(&self, node_id: &[u8; 32]) -> Option<&PeerRecord> {
        self.peers.get(node_id)
    }

    /// Number of active peers.
    pub fn active_count(&self) -> usize {
        self.active_peers().len()
    }

    /// Total peers (including inactive).
    pub fn total_count(&self) -> usize {
        self.peers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(id: u8) -> [u8; 32] {
        let mut a = [0u8; 32];
        a[0] = id;
        a
    }

    #[test]
    fn n73_add_and_lookup_peer() {
        let mut table = PeerTable::new(10);
        let id = make_id(1);
        table
            .upsert(id, "127.0.0.1:9001".parse().unwrap(), 100)
            .unwrap();
        assert_eq!(table.active_count(), 1);
        assert!(table.get(&id).is_some());
    }

    #[test]
    fn n73_duplicate_peer_updated() {
        let mut table = PeerTable::new(10);
        let id = make_id(1);
        table
            .upsert(id, "127.0.0.1:9001".parse().unwrap(), 100)
            .unwrap();
        table
            .upsert(id, "127.0.0.1:9002".parse().unwrap(), 200)
            .unwrap();
        assert_eq!(table.active_count(), 1);
        assert_eq!(table.get(&id).unwrap().chain_height, 200);
    }

    #[test]
    fn n73_remove_peer() {
        let mut table = PeerTable::new(10);
        let id = make_id(1);
        table
            .upsert(id, "127.0.0.1:9001".parse().unwrap(), 100)
            .unwrap();
        table.remove(&id);
        assert_eq!(table.active_count(), 0);
    }

    #[test]
    fn n73_max_peers_enforced() {
        let mut table = PeerTable::new(2);
        table
            .upsert(make_id(1), "127.0.0.1:9001".parse().unwrap(), 100)
            .unwrap();
        table
            .upsert(make_id(2), "127.0.0.1:9002".parse().unwrap(), 100)
            .unwrap();
        let result = table.upsert(make_id(3), "127.0.0.1:9003".parse().unwrap(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn n73_peer_expiry() {
        let mut table = PeerTable::new(10);
        let id = make_id(1);
        table
            .upsert(id, "127.0.0.1:9001".parse().unwrap(), 100)
            .unwrap();
        // Set last_seen far in the past
        table.peers.get_mut(&id).unwrap().last_seen = 0;
        table.expire_stale(60);
        assert_eq!(table.active_count(), 0);
    }
}
