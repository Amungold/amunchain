use crate::sync_transport::SyncTransport;
use amun_resource_core::ResourceRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejoinResult {
    Rejoined {
        height: u64,
        resources_imported: usize,
    },
    Failed {
        reason: String,
    },
}

pub struct RejoinProtocol;

impl RejoinProtocol {
    /// Execute the rejoin protocol.
    ///
    /// `peer_registry` — the source node's state.
    /// `peer_claimed_history_root` — what history root the peer claims.
    /// `my_trusted_history_root` — what the rejoining node actually trusts.
    ///
    /// If the peer's claimed history root does not match the node's trusted
    /// root, the rejoin is rejected.
    pub fn rejoin(
        peer_registry: &ResourceRegistry,
        height: u64,
        block_hash: [u8; 32],
        peer_claimed_history_root: [u8; 32],
        my_trusted_history_root: [u8; 32],
    ) -> RejoinResult {
        // The rejoining node first checks that the peer's claimed root
        // matches its own trusted root.
        if peer_claimed_history_root != my_trusted_history_root {
            return RejoinResult::Failed {
                reason: format!(
                    "History root mismatch: peer claims {}, I trust {}",
                    hex::encode(peer_claimed_history_root),
                    hex::encode(my_trusted_history_root),
                ),
            };
        }

        // Export snapshot using the agreed-upon history root
        let package = SyncTransport::export_snapshot(
            peer_registry,
            height,
            block_hash,
            my_trusted_history_root,
            "rejoin-protocol".into(),
        );

        match SyncTransport::import_snapshot(&package, my_trusted_history_root) {
            Ok(registry) => RejoinResult::Rejoined {
                height,
                resources_imported: registry.total(),
            },
            Err(reason) => RejoinResult::Failed { reason },
        }
    }
}
