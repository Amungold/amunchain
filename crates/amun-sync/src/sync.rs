use amun_chain_store::store::ChainStore;
use std::net::SocketAddr;

pub struct SyncState {
    pub is_synced: bool,
    pub sync_progress: f64,
    pub target_height: u64,
}

pub fn check_sync_status(store: &ChainStore, peers: &[SocketAddr]) -> Result<SyncState, String> {
    let local_height = store.latest_height();

    let peer = crate::peer_discovery::discover_peer_tip(peers);
    match peer {
        None => Ok(SyncState {
            is_synced: true,
            sync_progress: 1.0,
            target_height: local_height,
        }),
        Some(peer) => {
            if peer.tip_height <= local_height + 5 {
                Ok(SyncState {
                    is_synced: true,
                    sync_progress: 1.0,
                    target_height: peer.tip_height,
                })
            } else {
                Ok(SyncState {
                    is_synced: false,
                    sync_progress: local_height as f64 / peer.tip_height as f64,
                    target_height: peer.tip_height,
                })
            }
        }
    }
}
