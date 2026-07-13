use super::{Node, NodeHash};
use std::collections::HashSet;

pub struct CycleDetector;
impl CycleDetector {
    pub fn detect<F>(root_hash: NodeHash, get_node: F) -> Result<(), &'static str>
    where
        F: Fn(&NodeHash) -> Option<Node>,
    {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        Self::dfs(&root_hash, &get_node, &mut visiting, &mut visited)
    }
    fn dfs<F>(
        hash: &NodeHash,
        get_node: &F,
        visiting: &mut HashSet<NodeHash>,
        visited: &mut HashSet<NodeHash>,
    ) -> Result<(), &'static str>
    where
        F: Fn(&NodeHash) -> Option<Node>,
    {
        if visited.contains(hash) {
            return Ok(());
        }
        if visiting.contains(hash) {
            return Err("cycle detected");
        }
        visiting.insert(*hash);
        if let Some(Node::Branch { left, right }) = get_node(hash) {
            Self::dfs(&left, get_node, visiting, visited)?;
            Self::dfs(&right, get_node, visiting, visited)?;
        }
        visiting.remove(hash);
        visited.insert(*hash);
        Ok(())
    }
}
