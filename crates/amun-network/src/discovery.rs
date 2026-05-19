use crate::peer::Peer;
use crate::constants::MAX_PEERS;
use hashbrown::HashSet;
use heapless::Vec;

pub struct Discovery {
    peers: Vec<Peer, MAX_PEERS>,
    seed_ids: HashSet<[u8; 48]>,
}

impl Discovery {
    pub fn new() -> Self {
        Self { peers: Vec::new(), seed_ids: HashSet::new() }
    }
    pub fn add_seed(&mut self, peer: Peer) -> Result<(), &'static str> {
        if self.peers.is_full() { return Err("peer table full"); }
        self.seed_ids.insert(peer.id.0);
        self.peers.push(peer).map_err(|_| "push failed")
    }
    pub fn discover(&mut self) -> Vec<Peer, MAX_PEERS> {
        { let mut v = Vec::new(); for p in self.peers.iter().filter(|p| p.is_active()).cloned() { let _ = v.push(p); } v }
    }
    pub fn add_peer(&mut self, peer: Peer) -> Result<(), &'static str> {
        if self.peers.is_full() { return Err("peer table full"); }
        if self.peers.iter().any(|p| p.id == peer.id) { return Ok(()); }
        self.peers.push(peer).map_err(|_| "push failed")
    }
    pub fn peer_count(&self) -> usize { self.peers.len() }
    pub fn active_count(&self) -> usize { self.peers.iter().filter(|p| p.is_active()).count() }
}
