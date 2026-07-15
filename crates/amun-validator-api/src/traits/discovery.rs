use crate::error::PlatformResult;
use crate::types::id::PeerId;

pub trait DiscoveryProvider: Send + Sync {
    fn known_peers(&self) -> PlatformResult<Vec<PeerId>>;
    fn discover_peers(&self) -> PlatformResult<Vec<PeerId>>;
    fn peer_reputation(&self, peer_id: &PeerId) -> PlatformResult<Option<u32>>;
}
