use std::collections::BTreeMap;
use super::node::ImmutableLineageNode;
use crate::crypto::types::NodeHash;
#[derive(Debug, Clone)]
pub struct LineageGraph { nodes: BTreeMap<NodeHash, ImmutableLineageNode> }
impl LineageGraph {
    pub fn new() -> Self { Self { nodes: BTreeMap::new() } }
    pub fn add_node(&mut self, node: ImmutableLineageNode) -> Result<NodeHash, String> {
        let h = node.hash();
        if self.nodes.contains_key(&h) { return Err("exists".into()); }
        self.nodes.insert(h, node); Ok(h)
    }
    pub fn get_node(&self, h: &NodeHash) -> Option<&ImmutableLineageNode> { self.nodes.get(h) }
    pub fn len(&self) -> usize { self.nodes.len() }
}
impl Default for LineageGraph { fn default() -> Self { Self::new() } }
