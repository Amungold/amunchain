// Replay Continuity Engine
// THEOREM 11: state -> snapshot -> restore -> WAL replay -> final_root
// This is the constitutional heart of sovereign state transfer.

use super::chunk::ChunkIndex;
use super::restore::SnapshotRestoreEngine;
use amun_storage_kernel::persistence::wal::WalIterator;
use amun_storage_kernel::Key256;

pub struct ReplayContinuityEngine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContinuityResult {
    /// Full continuity verified: snapshot + WAL == live state
    Verified {
        final_root: [u8; 32],
        replayed_frames: u64,
    },
    /// Continuity broken at specific frame
    Broken {
        at_frame: u64,
        expected_root: [u8; 32],
        actual_root: [u8; 32],
    },
}

impl ReplayContinuityEngine {
    /// THEOREM 11: Verify that restoring a snapshot and replaying the
    /// remaining WAL produces the same final root as the claimed live state.
    pub fn verify_continuity(
        snapshot_chunks: &ChunkIndex,
        wal_path: &str,
        claimed_live_root: [u8; 32],
    ) -> Result<ContinuityResult, String> {
        // Step 1: Restore state from snapshot
        let mut tree = SnapshotRestoreEngine::restore(snapshot_chunks)
            .map_err(|e| format!("Snapshot restore failed: {}", e))?;

        let _snapshot_root = tree.root().0;

        // Step 2: Replay WAL from snapshot checkpoint forward
        let iter = WalIterator::new(wal_path);
        let mut replayed_frames: u64 = 0;

        for entry in iter {
            if entry.op_type == 0x05 {
                let mut key_bytes = [0u8; 32];
                let mut val_bytes = [0u8; 32];
                if entry.key_hash.len() != 32 || entry.value_hash.len() != 32 {
                    return Err(format!("Invalid WAL entry at frame {}", entry.sequence));
                }
                key_bytes.copy_from_slice(&entry.key_hash);
                val_bytes.copy_from_slice(&entry.value_hash);

                tree = tree.insert(&Key256(key_bytes), &val_bytes, entry.version);
                let actual_root = tree.root().0;

                if actual_root != entry.state_root {
                    return Ok(ContinuityResult::Broken {
                        at_frame: entry.sequence,
                        expected_root: entry.state_root,
                        actual_root,
                    });
                }
                replayed_frames += 1;
            }
        }

        let final_root = tree.root().0;

        // Step 3: Verify final root matches claimed live root
        if final_root != claimed_live_root {
            return Ok(ContinuityResult::Broken {
                at_frame: replayed_frames,
                expected_root: claimed_live_root,
                actual_root: final_root,
            });
        }

        Ok(ContinuityResult::Verified {
            final_root,
            replayed_frames,
        })
    }

    /// Verify just the roundtrip: snapshot restore produces expected root
    pub fn verify_roundtrip(chunks: &ChunkIndex, expected_root: [u8; 32]) -> Result<bool, String> {
        let tree =
            SnapshotRestoreEngine::restore(chunks).map_err(|e| format!("Restore failed: {}", e))?;
        Ok(tree.root().0 == expected_root)
    }
}
