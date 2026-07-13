use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    Sync,
    Vote,
    BlockRange,
    Governance,
    Evidence,
    StateSnapshot,
}

impl Capability {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "sync" => Some(Capability::Sync),
            "vote" => Some(Capability::Vote),
            "block_range" => Some(Capability::BlockRange),
            "governance" => Some(Capability::Governance),
            "evidence" => Some(Capability::Evidence),
            "state_snapshot" => Some(Capability::StateSnapshot),
            _ => None,
        }
    }
}

pub struct CapabilityEnforcer {
    peer_capabilities: RwLock<HashMap<SocketAddr, HashSet<Capability>>>,
}

impl CapabilityEnforcer {
    pub fn new() -> Self {
        Self {
            peer_capabilities: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_peer(&self, addr: SocketAddr, capabilities: Vec<String>) {
        let caps: HashSet<Capability> = capabilities
            .iter()
            .filter_map(|s| Capability::from_string(s))
            .collect();

        self.peer_capabilities.write().unwrap().insert(addr, caps);
    }

    pub fn remove_peer(&self, addr: &SocketAddr) {
        self.peer_capabilities.write().unwrap().remove(addr);
    }

    pub fn can_send_to(&self, addr: &SocketAddr, required: &[Capability]) -> bool {
        if let Some(caps) = self.peer_capabilities.read().unwrap().get(addr) {
            required.iter().all(|c| caps.contains(c))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    fn test_capability_enforcement() {
        let enforcer = CapabilityEnforcer::new();
        let peer: SocketAddr = "127.0.0.1:9000".parse().unwrap();

        enforcer.register_peer(peer, vec!["sync".to_string(), "vote".to_string()]);

        assert!(enforcer.can_send_to(&peer, &[Capability::Vote]));
        assert!(enforcer.can_send_to(&peer, &[Capability::Sync]));
        assert!(!enforcer.can_send_to(&peer, &[Capability::BlockRange]));
    }
}
