use std::collections::{HashSet, VecDeque};
use crate::state_tree::{NodeHash, Node};
use crate::storage::persistent_node_store::PersistentNodeStore;

pub struct MarkSweepGC;

impl MarkSweepGC {
    pub fn mark(root_hash: NodeHash, node_store: &PersistentNodeStore) -> HashSet<NodeHash> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root_hash);
        reachable.insert(root_hash);
        while let Some(hash) = queue.pop_front() {
            if let Some(node) = node_store.get(&hash) {
                match node {
                    Node::Branch { left, right } => {
                        if !reachable.contains(&left) {
                            reachable.insert(left);
                            queue.push_back(left);
                        }
                        if !reachable.contains(&right) {
                            reachable.insert(right);
                            queue.push_back(right);
                        }
                    }
                    _ => {}
                }
            }
        }
        reachable
    }
    
    pub fn sweep(reachable: &HashSet<NodeHash>, node_store: &PersistentNodeStore) {
        // In a real implementation: iterate all node files, delete if not in reachable.
        // Placeholder for brevity.
    }
}
