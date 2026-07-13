use crate::config::{ClusterPeer, ValidatorConfig};
use amun_networking::validator_certificate::ValidatorCertificate;

pub struct ProductionConfigBuilder;

impl ProductionConfigBuilder {
    pub fn build(
        validator_id: [u8;32],
        listen_addr: std::net::SocketAddr,
        authority_public_key: [u8;32],
        cluster: Vec<ClusterPeer>,
        data_dir: String,
    ) -> ValidatorConfig {

        ValidatorConfig {
            validator_id,
            listen_addr,
            cluster,
            data_dir,
            authority_public_key,
            quorum_size: None,
        }
    }
}
