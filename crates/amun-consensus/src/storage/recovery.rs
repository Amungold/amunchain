use crate::state_tree::StateRoot;
use crate::storage::root_persistence::RootPersistence;
use crate::storage::wal::replay::WALReplayIterator;
use crate::storage::wal::codec::WALOp;
use crate::storage::version_manifest::VersionManifest;

pub struct RecoveryCoordinator;

impl RecoveryCoordinator {
    pub fn recover() -> (StateRoot, u64) {
        // 1. Load root from root file (highest precedence)
        if let Ok(root) = RootPersistence::load() {
            // Verify against latest manifest
            let manifest = VersionManifest::new("manifest.dat");
            if let Some(entry) = manifest.latest() {
                if entry.state_root.0 == root.0 {
                    return (root, entry.version);
                }
            }
            return (root, 0);
        }
        // 2. Otherwise replay WAL
        let mut iter = match WALReplayIterator::new("wal.log") {
            Ok(i) => i,
            Err(_) => return (StateRoot::EMPTY, 0),
        };
        let mut last_root = StateRoot::EMPTY;
        let mut last_version = 0;
        while let Some(frame) = iter.next() {
            if let WALOp::Checkpoint { state_root, version } = frame.op {
                last_root = StateRoot(state_root);
                last_version = version;
            }
        }
        if iter.corruption_detected {
            // Partial recovery, use last known good checkpoint
        }
        (last_root, last_version)
    }
}
