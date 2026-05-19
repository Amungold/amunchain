use crate::peer::Peer;
use crate::constants::MAX_CONNECTIONS_PER_IP;
use hashbrown::HashMap;

pub struct Connection {
    connections: HashMap<[u8; 48], Peer>,
    ip_counts: HashMap<[u8; 16], u32>,
}

impl Connection {
    pub fn new() -> Self {
        Self { connections: HashMap::new(), ip_counts: HashMap::new() }
    }
    pub fn connect(&mut self, peer: Peer, ip: [u8; 16]) -> Result<(), &'static str> {
        let count = self.ip_counts.get(&ip).copied().unwrap_or(0);
        if count >= MAX_CONNECTIONS_PER_IP as u32 { return Err("ip connection limit"); }
        self.connections.insert(peer.id.0, peer);
        self.ip_counts.insert(ip, count.saturating_add(1));
        Ok(())
    }
    pub fn disconnect(&mut self, peer_id: &[u8; 48]) {
        self.connections.remove(peer_id);
    }
    pub fn is_connected(&self, peer_id: &[u8; 48]) -> bool {
        self.connections.contains_key(peer_id)
    }
    pub fn connection_count(&self) -> usize { self.connections.len() }
}
