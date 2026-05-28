//! Content-addressed node interner.

use std::collections::HashMap;
use std::sync::Arc;
use crate::node::Node;
use crate::hash::Hash;
use crate::error::SmtError;

/// Immutable content-addressed node store.
pub struct Context {
    nodes: HashMap<Hash, Arc<Node>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Intern a node. Returns existing `Arc` if the hash already exists.
    pub fn intern(&mut self, node: Node) -> Result<Arc<Node>, SmtError> {
        let hash = node.hash();
        if let Some(existing) = self.nodes.get(&hash) {
            return Ok(existing.clone());
        }
        let arc = Arc::new(node);
        self.nodes.insert(hash, arc.clone());
        Ok(arc)
    }

    /// Look up a node by hash.
    pub fn get_node(&self, hash: &Hash) -> Result<Arc<Node>, SmtError> {
        self.nodes.get(hash).cloned().ok_or(SmtError::NodeNotFound)
    }
}
