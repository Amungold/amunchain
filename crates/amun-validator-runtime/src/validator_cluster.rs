use amun_resource_core::ResourceId;
use crate::validator_node::ValidatorNode;

pub struct ValidatorCluster {
    pub nodes: Vec<ValidatorNode>,
}

impl ValidatorCluster {
    pub fn new(base_dir: &str) -> Result<Self, String> {
        let mut nodes = Vec::new();
        for i in 0..4 {
            let node_id = ResourceId([i as u8; 32]);
            let dir = format!("{}/node{}", base_dir, i);
            let node = ValidatorNode::new(node_id, &dir)?;
            nodes.push(node);
        }
        Ok(Self { nodes })
    }

    pub fn run_blocks(&mut self, count: u64) -> Result<(), String> {
        for height in 1..=count {
            let leader_idx = ((height - 1) % self.nodes.len() as u64) as usize;
            let state_root = self.nodes[leader_idx].propose_block(height)?;

            for i in 0..self.nodes.len() {
                if i != leader_idx {
                    self.nodes[i].import_block(height, state_root)?;
                }
            }
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        for node in &mut self.nodes {
            node.shutdown()?;
        }
        Ok(())
    }

    pub fn current_height(&self) -> u64 {
        self.nodes[0].current_height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n64_cluster_create_and_run() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        let mut cluster = ValidatorCluster::new(base).unwrap();
        cluster.run_blocks(10).unwrap();
        assert_eq!(cluster.current_height(), 10);

        for node in &cluster.nodes {
            assert_eq!(node.current_height(), 10, "All nodes must be at height 10");
        }
        cluster.shutdown().unwrap();
    }

    #[test]
    fn n64_cluster_restart_preserves_state() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();

        let mut cluster = ValidatorCluster::new(base).unwrap();
        cluster.run_blocks(10).unwrap();
        cluster.shutdown().unwrap();
        drop(cluster);

        let cluster2 = ValidatorCluster::new(base).unwrap();
        assert_eq!(cluster2.current_height(), 10);

        for node in &cluster2.nodes {
            assert_eq!(node.current_height(), 10);
        }
    }

    #[test]
    fn n64_cluster_round_robin_distribution() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        let mut cluster = ValidatorCluster::new(base).unwrap();
        cluster.run_blocks(12).unwrap();

        for node in &cluster.nodes {
            assert_eq!(node.current_height(), 12);
        }
    }

    #[test]
    fn n64_cluster_state_root_equality() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        let mut cluster = ValidatorCluster::new(base).unwrap();
        cluster.run_blocks(10).unwrap();

        // ALL nodes must have identical state roots after syncing
        let root = cluster.nodes[0].state_root();
        for node in &cluster.nodes {
            assert_eq!(node.state_root(), root,
                "All nodes must have identical state roots after block import");
            assert_eq!(node.current_height(), 10);
        }
    }

    #[test]
    fn n64_cluster_tampered_state_root_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        let mut cluster = ValidatorCluster::new(base).unwrap();

        // Node 0 proposes block 1
        let state_root = cluster.nodes[0].propose_block(1).unwrap();

        // Tamper the state root before importing to node 1
        let mut tampered = state_root;
        tampered[0] ^= 0xFF;

        let result = cluster.nodes[1].import_block(1, tampered);
        assert!(result.is_err(), "Tampered state root must be rejected");
    }
}

    #[test]
    fn n64_cluster_stress_1000_blocks() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        let mut cluster = ValidatorCluster::new(base).unwrap();
        
        cluster.run_blocks(1000).unwrap();
        
        // All 4 nodes must be at height 1000 with identical state roots
        let root = cluster.nodes[0].state_root();
        for (i, node) in cluster.nodes.iter().enumerate() {
            assert_eq!(node.current_height(), 1000, 
                "Node {} must be at height 1000", i);
            assert_eq!(node.state_root(), root,
                "Node {} must have the same state root after 1000 blocks", i);
        }
        
        // Verify block distribution: each validator proposed ~250 blocks
        // (1000 blocks / 4 validators = 250 each)
        for (i, node) in cluster.nodes.iter().enumerate() {
            assert!(node.metrics.blocks_produced >= 245 && node.metrics.blocks_produced <= 255,
                "Node {} produced {} blocks, expected ~250", i, node.metrics.blocks_produced);
        }
        
        cluster.shutdown().unwrap();
        
        // Restart and verify state persists
        drop(cluster);
        let cluster2 = ValidatorCluster::new(base).unwrap();
        let root2 = cluster2.nodes[0].state_root();
        assert_eq!(root2, root, "State root must persist after restart");
        for node in &cluster2.nodes {
            assert_eq!(node.current_height(), 1000);
            assert_eq!(node.state_root(), root);
        }
    }
