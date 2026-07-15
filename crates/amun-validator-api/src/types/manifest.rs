use crate::types::capabilities::ValidatorCapabilities;
use crate::types::id::ValidatorId;
use crate::types::version::PlatformVersion;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ValidatorManifest {
    pub manifest_version: u32,
    pub validator_name: String,
    pub validator_id: ValidatorId,
    pub chain_id: String,
    pub platform_version: PlatformVersion,
    pub capabilities: ValidatorCapabilities,
    pub identity: IdentityManifest,
    pub storage: StorageManifest,
    pub network: NetworkManifest,
    pub data_dir: PathBuf,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct IdentityManifest {
    pub certificate_path: PathBuf,
    pub private_key_path: PathBuf,
    pub public_key: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub authority_key: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct StorageManifest {
    pub state_db_path: PathBuf,
    pub wal_path: PathBuf,
    pub snapshot_path: PathBuf,
    pub block_db_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct NetworkManifest {
    pub listen_address: String,
    pub bootstrap_peers: Vec<String>,
    pub external_address: Option<String>,
    pub nat_enabled: bool,
}
