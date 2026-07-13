use amun_orchestrator_core::types::{PeerId, PublicKey, ValidatorId};
use serde::Deserialize;
use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub name: String,
    pub validator_id: ValidatorId,
    pub peer_id: PeerId,
    pub public_key: PublicKey,
    pub address: SocketAddr,
    pub certificate_path: PathBuf,
    pub config_path: PathBuf,
    pub data_dir: PathBuf,
    pub cluster: Vec<ClusterCfg>,
}

#[derive(Debug, Clone)]
pub struct ValidatorInventory {
    pub validators: Vec<ValidatorInfo>,
}

#[derive(Debug, Deserialize)]
struct NodeCfg {
    name: String,
    listen_port: u16,
}

#[derive(Debug, Deserialize)]
struct IdentityCfg {
    peer_id: String,
    validator_id: String,
    public_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterCfg {
    validator_id: String,
    address: String,
    certificate_path: String,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    node: NodeCfg,
    identity: IdentityCfg,
    #[serde(default)]
    cluster: Vec<ClusterCfg>,
}

impl ValidatorInventory {
    pub fn scan(root: impl AsRef<Path>) -> Result<Self, String> {
        let mut validators = Vec::new();

        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;

            if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                continue;
            }

            let dir = entry.path();

            let cfg_path = dir.join("config.toml");
            let crt_path = dir.join("validator.crt");

            if !cfg_path.exists() {
                continue;
            }

            let text = fs::read_to_string(&cfg_path).map_err(|e| e.to_string())?;

            let cfg: ConfigFile = toml::from_str(&text).map_err(|e| e.to_string())?;

            let peer = hex::decode(cfg.identity.peer_id).map_err(|e| e.to_string())?;
            let vid = hex::decode(cfg.identity.validator_id).map_err(|e| e.to_string())?;
            let pk = hex::decode(cfg.identity.public_key).map_err(|e| e.to_string())?;

            if peer.len() != 32 || vid.len() != 32 || pk.len() != 32 {
                return Err(format!("Invalid identity in {}", cfg_path.display()));
            }

            let mut pk_arr = [0u8; 32];
            pk_arr.copy_from_slice(&pk);

            let canonical_validator = amun_validator_identity::derive_validator_id(&pk_arr);

            let canonical_peer = pk_arr;

            validators.push(ValidatorInfo {
                name: cfg.node.name,
                peer_id: PeerId(canonical_peer),
                validator_id: ValidatorId(canonical_validator),
                public_key: PublicKey(pk_arr),
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), cfg.node.listen_port),
                certificate_path: crt_path,
                config_path: cfg_path,
                data_dir: dir,
                cluster: cfg.cluster,
            });
        }

        println!("========== INVENTORY ==========");
        for v in &validators {
            println!(
                "{}  cfg={}  crt_exists={}",
                v.name,
                v.config_path.display(),
                v.certificate_path.exists(),
            );
        }
        println!("===============================");

        validators.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self { validators })
    }
}
