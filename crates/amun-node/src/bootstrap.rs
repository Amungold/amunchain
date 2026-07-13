use std::fs;
use std::path::{Path, PathBuf};

use amun_networking::handshake::ConstitutionInfo;
use amun_networking::node::NetworkNode;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::validator_certificate::ValidatorCertificate;

use crate::config::Config;
use crate::error::{io_err, NodeError};
use crate::genesis::Genesis;

#[allow(dead_code)]
pub struct BootstrapContext {
    pub transport: TcpTransport,
    pub node: NetworkNode,
    pub cert: ValidatorCertificate,
    pub genesis: Genesis,
    pub genesis_hash: [u8; 32],
    pub peer_id_bytes: [u8; 32],
    pub config: Config,
    pub config_dir: PathBuf,
}

pub fn initialize(config_path: &str) -> Result<BootstrapContext, NodeError> {
    let config = Config::load(config_path)?;
    let config_dir = Path::new(config_path).parent().unwrap_or(Path::new("."));
    tracing::info!(node = %config.node.name, "Node configured");

    // Identity
    let key_path = config_dir.join(&config.identity.key_file);
    let keypair = crate::identity::load_or_create_keypair(key_path.to_str().unwrap())?;
    let peer_id = keypair.peer_id();
    tracing::info!(peer_id = %hex::encode(peer_id.0), "Identity loaded");

    // Genesis
    let genesis_path = config_dir.join(&config.genesis.file);
    let genesis_str = fs::read_to_string(&genesis_path).map_err(|e| io_err(&genesis_path, e))?;
    let genesis: Genesis = serde_json::from_str(&genesis_str).map_err(|e| NodeError::Json {
        path: genesis_path.clone(),
        source: e,
    })?;

    if !amun_orchestrator_genesis::validator::validate(&genesis) {
        return Err(NodeError::Genesis("Genesis validation failed".into()));
    }

    let genesis_hash = {
        let mut gen = genesis.clone();
        amun_orchestrator_genesis::generator::recompute_hash(&mut gen);
        hex::decode(&gen.genesis_hash)
            .map_err(|e| NodeError::Genesis(format!("Invalid genesis hash: {}", e)))?
            .try_into()
            .map_err(|_| NodeError::Genesis("Genesis hash must be 32 bytes".into()))?
    };

    tracing::info!(
        path = %genesis_path.display(),
        hash = %hex::encode(genesis_hash),
        "Genesis loaded and validated"
    );

    // Certificate
    let cert_path = config_dir.join("validator.crt");
    let cert = crate::certificate_loader::load_validator_certificate(
        cert_path.to_str().unwrap(),
        &genesis,
    )
    .map_err(NodeError::Certificate)?;
    tracing::info!("Validator certificate verified");

    // Extract SigningKey from the loaded keypair
    // Use the existing seed() method and SigningKey::from_bytes()
    let seed = keypair.to_seed();
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&seed);

    // Network ID: SHA-256 of chain_id
    let network_id = {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(genesis.chain_id.as_bytes());
        hasher.finalize().into()
    };

    // Constitution from genesis
    let constitution = ConstitutionInfo {
        version: 1,
        hash: genesis_hash,
        proof_system_version: 1,
        state_commitment_algorithm: "MerklePatricia".to_string(),
        accepted_features: vec!["sync".into(), "vote".into(), "block_range".into()],
    };

    // REAL listen address from config
    let addr: std::net::SocketAddr =
        format!("{}:{}", config.node.listen_host, config.node.listen_port)
            .parse()
            .map_err(|_| {
                NodeError::InvalidAddress(format!(
                    "{}:{}",
                    config.node.listen_host, config.node.listen_port
                ))
            })?;

    // NodeNetwork with REAL identity
    let network = crate::network::NodeNetwork::new(
        addr,
        signing_key,
        peer_id.0,
        network_id,
        genesis_hash,
        constitution,
    )
    .map_err(NodeError::InvalidAddress)?;

    let mut node = NetworkNode::new(peer_id.0);
    node.keypair = Some(keypair);

    Ok(BootstrapContext {
        transport: network.transport,
        node,
        cert,
        genesis,
        genesis_hash,
        peer_id_bytes: peer_id.0,
        config,
        config_dir: config_dir.to_path_buf(),
    })
}
