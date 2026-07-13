use std::net::SocketAddr;
use std::str::FromStr;

use amun_live_cluster::config::{ClusterPeer, ValidatorConfig};

use crate::bootstrap::BootstrapContext;

/// Builds a LiveValidator configuration from BootstrapContext.
pub struct ClusterBuilder;

impl ClusterBuilder {
    pub fn build(ctx: &BootstrapContext) -> Result<ValidatorConfig, String> {
        let validator = ctx
            .config
            .validator
            .as_ref()
            .ok_or("Missing [validator] section in config.toml")?;

        let listen_addr = SocketAddr::from_str(&format!(
            "{}:{}",
            ctx.config.node.listen_host, ctx.config.node.listen_port
        ))
        .map_err(|e| e.to_string())?;

        let mut cluster = Vec::new();

        for peer in &ctx.config.cluster {
            let addr = SocketAddr::from_str(&peer.address).map_err(|e| e.to_string())?;

            cluster.push(ClusterPeer {
                validator_id: peer.validator_id,
                certificate_path: Some(peer.certificate_path.clone()),
                address: addr,
            });
        }

        Ok(ValidatorConfig {
            validator_id: validator.validator_id,
            listen_addr,
            cluster,
            data_dir: validator.data_dir.clone(),
            quorum_size: None,
            authority_public_key: validator.authority_public_key,
        })
    }
}
