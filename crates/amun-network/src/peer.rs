use amun_kernel_types::PublicKey;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeerState {
    Connected,
    Disconnected,
    Stale,
    Banned,
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PublicKey,
    pub address: heapless::String<128>,
    pub state: PeerState,
    pub last_seen_ms: u64,
    pub connection_count: u32,
}

impl Peer {
    pub fn new(id: PublicKey, address: heapless::String<128>) -> Self {
        Self { id, address, state: PeerState::Disconnected, last_seen_ms: 0, connection_count: 0 }
    }
    pub fn is_active(&self) -> bool {
        matches!(self.state, PeerState::Connected)
    }
    pub fn mark_connected(&mut self, now_ms: u64) {
        self.state = PeerState::Connected;
        self.last_seen_ms = now_ms;
        self.connection_count = self.connection_count.saturating_add(1);
    }
    pub fn mark_disconnected(&mut self) {
        self.state = PeerState::Disconnected;
    }
    pub fn mark_stale(&mut self) {
        self.state = PeerState::Stale;
    }
}
