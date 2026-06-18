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
pub fn create_snapshot(store: &ChainStore, output_dir: &Path) -> Result<SnapshotManifest, String> {
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

/// Verify the integrity of a snapshot directory.
/// Returns the manifest if valid, or an error string.
pub fn verify_snapshot(snapshot_dir: &Path) -> Result<SnapshotManifest, String> {
    let manifest_path = snapshot_dir.join("manifest.json");
    let state_path = snapshot_dir.join("state.bin");

    // 1. Files must exist
    if !manifest_path.exists() {
        return Err("manifest.json not found".into());
    }
    if !state_path.exists() {
        return Err("state.bin not found".into());
    }

    // 2. Read and parse manifest
    let json = std::fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;
    let manifest: SnapshotManifest = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    // 3. Recompute hash from the fields (excluding snapshot_hash itself)
    let pre_hash = postcard::to_stdvec(&(
        manifest.snapshot_height,
        manifest.state_root,
        manifest.history_root,
        manifest.validator_set_hash,
    ))
    .map_err(|e| e.to_string())?;
    let expected_hash = blake3::hash(&pre_hash);

    if manifest.snapshot_hash != *expected_hash.as_bytes() {
        return Err(format!(
            "Hash mismatch: stored={} computed={}",
            hex::encode(manifest.snapshot_hash),
            hex::encode(expected_hash.as_bytes())
        ));
    }

    // 4. Quick check that state.bin contains the same height
    let state_bytes = std::fs::read(&state_path).map_err(|e| e.to_string())?;
    if state_bytes.len() < 8 {
        return Err("state.bin too short".into());
    }
    let height = u64::from_le_bytes(state_bytes[0..8].try_into().unwrap());
    if height != manifest.snapshot_height {
        return Err(format!(
            "Height mismatch: state={} manifest={}",
            height, manifest.snapshot_height
        ));
    }

    Ok(manifest)
}

/// Restore a chain store from a verified snapshot.
/// Creates a new store at `store_dir` and populates it with the tip record.
pub fn restore_snapshot(snapshot_dir: &Path, store_dir: &Path) -> Result<SnapshotManifest, String> {
    // Verify first
    let manifest = verify_snapshot(snapshot_dir)?;

    // Read state.bin
    let state_path = snapshot_dir.join("state.bin");
    let state_bytes = std::fs::read(&state_path).map_err(|e| e.to_string())?;
    let state_root: [u8; 32] = state_bytes[8..40]
        .try_into()
        .map_err(|e| format!("{:?}", e))?;

    // Build the tip record
    let record = crate::record::FinalizedChainRecord {
        height: manifest.snapshot_height,
        block_hash: [manifest.snapshot_height as u8; 32],
        state_root,
        history_root: manifest.history_root,
        certificate_hash: [0u8; 32],
        slashing_root: [0u8; 32],
        timestamp: 0,
    };

    // Create/open store
    let mut store = crate::store::ChainStore::open(store_dir.to_str().unwrap())
        .map_err(|e| format!("open store: {}", e))?;
    store
        .append(record)
        .map_err(|e| format!("append record: {}", e))?;

    Ok(manifest)
}
