use std::collections::HashMap;
use std::net::SocketAddr;

use parking_lot::RwLock;

/// Authenticated session for a peer.
#[derive(Debug, Clone)]
pub struct Session {
    pub peer_addr: SocketAddr,
    pub session_id: [u8; 32],
    pub authenticated_at: u64,
    pub last_active: u64,
}

/// SessionManager (Constitution Part II)
/// Authenticates peers only. Never validates consensus state.
pub struct SessionManager {
    sessions: RwLock<HashMap<SocketAddr, Session>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Register an authenticated session.
    pub fn register_session(&self, addr: SocketAddr, session_id: [u8; 32], timestamp: u64) {
        let session = Session {
            peer_addr: addr,
            session_id,
            authenticated_at: timestamp,
            last_active: timestamp,
        };
        self.sessions.write().insert(addr, session);
    }

    /// Check if a peer has an active session.
    pub fn is_authenticated(&self, addr: &SocketAddr) -> bool {
        self.sessions.read().contains_key(addr)
    }

    /// Get session for a peer.
    pub fn get_session(&self, addr: &SocketAddr) -> Option<Session> {
        self.sessions.read().get(addr).cloned()
    }

    /// Remove a session (disconnect).
    pub fn remove_session(&self, addr: &SocketAddr) {
        self.sessions.write().remove(addr);
    }

    /// Update last active timestamp.
    pub fn touch(&self, addr: &SocketAddr, timestamp: u64) {
        if let Some(session) = self.sessions.write().get_mut(addr) {
            session.last_active = timestamp;
        }
    }

    /// Count of active sessions.
    /// Register a session using the session_id from handshake confirmation.
    /// This is the single source of truth for session identity.
    pub fn register_from_handshake_with_id(&self, addr: SocketAddr, session_id: [u8; 32]) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.register_session(addr, session_id, now);
    }

    pub fn session_count(&self) -> usize {
        self.sessions.read().len()
    }
}

impl SessionManager {
    /// Register a session from handshake data.
    /// Called after successful handshake completion.
    pub fn register_from_handshake(&self, addr: SocketAddr) -> [u8; 32] {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let sid: [u8; 32] = rand::random();
        self.register_session(addr, sid, now);
        sid
    }

    /// Check session and update last_active timestamp.
    pub fn verify_and_touch(&self, addr: &SocketAddr) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if self.is_authenticated(addr) {
            self.touch(addr, now);
            true
        } else {
            false
        }
    }
}
