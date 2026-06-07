use amun_resource_core::ResourceId;
use crate::validator_node::ValidatorNode;

/// Wraps a ValidatorNode with its tempdir to keep the directory alive.
pub struct NodeInstance {
    pub node: ValidatorNode,
    _dir: tempfile::TempDir,
}

pub struct ValidatorCluster {
    pub nodes: Vec<NodeInstance>,
    pub block_height: u64,
}

impl ValidatorCluster {
    pub fn new(count: usize) -> Result<Self, String> {
        let mut nodes = Vec::new();
        for i in 0..count {
            let node_id = {
                let mut h = [0u8; 32];
                h[0..8].copy_from_slice(&(i as u64).to_le_bytes());
                ResourceId(h)
            };
            let dir = tempfile::tempdir().map_err(|e| format!("tempdir: {}", e))?;
            let node = ValidatorNode::new(node_id, dir.path().to_str().unwrap())?;
            nodes.push(NodeInstance { node, _dir: dir });
        }
        Ok(Self { nodes, block_height: 0 })
    }

    pub fn produce_blocks(&mut self, count: u64) -> Result<(), String> {
        for _ in 0..count {
            self.block_height += 1;
            for instance in &mut self.nodes {
                instance.node.propose_block(self.block_height)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n64_cluster_4_nodes_10_blocks() {
        let mut cluster = ValidatorCluster::new(4).unwrap();
        cluster.produce_blocks(10).unwrap();
        assert_eq!(cluster.block_height, 10);
        for instance in &cluster.nodes {
            assert!(instance.node.health.is_synced);
            assert_eq!(instance.node.metrics.blocks_produced, 10);
        }
    }

    #[test]
    fn n64_cluster_restart_preserves_state() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();
        let node_id = ResourceId([1u8; 32]);

        {
            let mut node = ValidatorNode::new(node_id, dir_str).unwrap();
            node.propose_block(1).unwrap();
            node.propose_block(2).unwrap();
        }

        {
            let node = ValidatorNode::new(node_id, dir_str).unwrap();
            assert_eq!(node.store.current_height(), 2);
        }
    }
}
