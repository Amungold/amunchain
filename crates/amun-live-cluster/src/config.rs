use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    pub validator_id: [u8; 32],
    pub listen_addr: SocketAddr,
    pub cluster: Vec<ClusterPeer>,
    pub data_dir: String,
    pub quorum_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterPeer {
    pub validator_id: [u8; 32],
    pub address: SocketAddr,
}

impl ValidatorConfig {
    pub fn localhost_cluster(validator_index: usize) -> Self {
        let base_port = 9000;
        let ids: [[u8; 32]; 4] = [[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let cluster: Vec<ClusterPeer> = (0..4)
            .map(|i| ClusterPeer {
                validator_id: ids[i],
                address: format!("127.0.0.1:{}", base_port + i).parse().unwrap(),
            })
            .collect();
        ValidatorConfig {
            validator_id: ids[validator_index],
            listen_addr: cluster[validator_index].address,
            cluster,
            data_dir: format!("/tmp/amun-validator-{}", validator_index),
            quorum_size: None,
        }
    }

    pub fn test_cluster(validator_index: usize, ports: &[u16; 4]) -> Self {
        let ids: [[u8; 32]; 4] = [[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let cluster: Vec<ClusterPeer> = (0..4)
            .map(|i| ClusterPeer {
                validator_id: ids[i],
                address: format!("127.0.0.1:{}", ports[i]).parse().unwrap(),
            })
            .collect();
        ValidatorConfig {
            validator_id: ids[validator_index],
            listen_addr: cluster[validator_index].address,
            cluster,
            data_dir: format!("/tmp/amun-test-validator-{}", validator_index),
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
