use crate::store::ChainStore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotManifest {
    pub snapshot_height: u64,
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    pub validator_set_hash: [u8; 32],
    pub snapshot_hash: [u8; 32],
}

/// Create a snapshot of the current chain state at the latest finalized height.
/// Writes manifest.json and a binary state dump into `output_dir`.
pub fn create_snapshot(
    store: &ChainStore,
    output_dir: &Path,
) -> Result<SnapshotManifest, String> {
    let tip = store
        .load_tip()
        .ok_or_else(|| "Empty store – cannot snapshot".to_string())?;

    let height = tip.height;
    let state_root = tip.state_root;
    let history_root = tip.history_root;
    let validator_set_hash = [0u8; 32]; // placeholder until validator set is stored

    // Build manifest (snapshot hash is a placeholder hash of the serialised manifest)
    let mut manifest = SnapshotManifest {
        snapshot_height: height,
        state_root,
        history_root,
        validator_set_hash,
        snapshot_hash: [0u8; 32],
    };

    // Serialise the manifest (without snapshot_hash) to compute its hash
    let pre_hash = postcard::to_stdvec(&(
        manifest.snapshot_height,
        manifest.state_root,
        manifest.history_root,
        manifest.validator_set_hash,
    ))
    .map_err(|e| e.to_string())?;
    let hash = blake3::hash(&pre_hash);
    manifest.snapshot_hash = hash.as_bytes().to_owned();

    // Write manifest.json
    fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
    let manifest_path = output_dir.join("manifest.json");
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(&manifest_path, json).map_err(|e| e.to_string())?;

    // Write state.bin (currently just the block height and state_root)
    let state_path = output_dir.join("state.bin");
    let mut state_file = fs::File::create(&state_path).map_err(|e| e.to_string())?;
    state_file
        .write_all(&height.to_le_bytes())
        .map_err(|e| e.to_string())?;
    state_file
        .write_all(&state_root)
        .map_err(|e| e.to_string())?;
    state_file.flush().map_err(|e| e.to_string())?;

    Ok(manifest)
}
