use crate::error::PlatformResult;
use crate::types::id::PeerId;

/// Single Source of Truth for all network operations.
/// Every network implementation in the system MUST implement this trait.
pub trait NetworkProvider: Send + Sync {
    // ── Lifecycle ──────────────────────────────────────────
    fn start(&self) -> PlatformResult<()>;
    fn stop(&self) -> PlatformResult<()>;

    // ── Status ─────────────────────────────────────────────
    fn peer_count(&self) -> PlatformResult<usize>;
    fn is_connected(&self) -> PlatformResult<bool>;
    fn listen_address(&self) -> PlatformResult<String>;
    fn check_ports(&self) -> PlatformResult<()>;

    // ── Connectivity ───────────────────────────────────────
    fn connect_to_peer(&self, address: &str) -> PlatformResult<PeerId>;
    fn disconnect_peer(&self, peer: &PeerId) -> PlatformResult<()>;

    // ── Messaging ──────────────────────────────────────────
    fn broadcast(&self, message: &[u8]) -> PlatformResult<()>;
    fn send_to_peer(&self, peer: &PeerId, message: &[u8]) -> PlatformResult<()>;
}
