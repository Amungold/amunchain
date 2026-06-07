/// Header gossip session — simplified for N55 compatibility.
pub struct GossipSession;

impl GossipSession {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GossipSession {
    fn default() -> Self {
        Self::new()
    }
}
