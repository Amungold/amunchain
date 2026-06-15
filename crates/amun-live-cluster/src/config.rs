use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
/// A constitutional authority loaded from genesis configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisAuthority {
    pub authority_public_key: [u8; 32],
    pub authority_version: u64,
}

/// Load genesis authority from a JSON file.
/// The file must exist; there is no fallback.
pub fn load_genesis_authority(path: &str) -> GenesisAuthority {
    let json = std::fs::read_to_string(path)
        .expect("Genesis authority file missing");
    serde_json::from_str(&json).expect("Invalid genesis authority JSON")
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    pub validator_id: [u8; 32],
    pub listen_addr: SocketAddr,
    pub cluster: Vec<ClusterPeer>,
    pub data_dir: String,
    pub quorum_size: Option<usize>,
    pub authority_public_key: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterPeer {
    pub validator_id: [u8; 32],
    #[serde(default)]
    pub certificate_path: Option<String>,
    pub address: SocketAddr,
}

impl ValidatorConfig {
    pub fn localhost_cluster(validator_index: usize) -> Self {
        let base_port = 9000;
        let ids: [[u8; 32]; 4] = [[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let mut cluster: Vec<ClusterPeer> = (0..4)
            .map(|i| ClusterPeer {
                validator_id: ids[i],
                certificate_path: None,
                address: format!("127.0.0.1:{}", base_port + i).parse().unwrap(),
            })
            .collect();
        set_cert_paths(&mut cluster);
        ValidatorConfig {
            validator_id: ids[validator_index],
            listen_addr: cluster[validator_index].address,
            cluster,
            data_dir: format!("/tmp/amun-validator-{}", validator_index),
            authority_public_key: load_genesis_authority(concat!(env!("CARGO_MANIFEST_DIR"), "/genesis/genesis_authority.json")).authority_public_key,
            quorum_size: None,
        }
    }

    pub fn test_cluster(validator_index: usize, ports: &[u16; 4]) -> Self {
        let ids: [[u8; 32]; 4] = [[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let mut cluster: Vec<ClusterPeer> = (0..4)
            .map(|i| ClusterPeer {
                validator_id: ids[i],
                certificate_path: None,
                address: format!("127.0.0.1:{}", ports[i]).parse().unwrap(),
            })
            .collect();
        set_cert_paths(&mut cluster);
        ValidatorConfig {
            validator_id: ids[validator_index],
            listen_addr: cluster[validator_index].address,
            cluster,
            data_dir: format!("/tmp/amun-test-validator-{}", validator_index),
            authority_public_key: load_genesis_authority(concat!(env!("CARGO_MANIFEST_DIR"), "/genesis/genesis_authority.json")).authority_public_key,
            quorum_size: None,
        }
    }

    pub fn with_quorum(mut self, n: usize) -> Self {
        self.quorum_size = Some(n);
        self
    }

    pub fn other_peers(&self) -> Vec<&ClusterPeer> {
        self.cluster
            .iter()
            .filter(|p| p.validator_id != self.validator_id)
            .collect()
    }

    // **NEW: Returns ALL peer addresses (including self) for forced static connections**
    pub fn all_peer_addresses(&self) -> Vec<SocketAddr> {
        self.cluster.iter().map(|p| p.address).collect()
    }

    pub fn total_validators(&self) -> usize {
        self.cluster.len()
    }
}

fn set_cert_paths(cluster: &mut [ClusterPeer]) {
    let cert_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("testdata")
        .join("certs");
    for (i, peer) in cluster.iter_mut().enumerate() {
        let path = cert_dir.join(format!("validator_{}.crt", i + 1));
        peer.certificate_path = Some(path.to_str().unwrap().to_string());
    }
}
