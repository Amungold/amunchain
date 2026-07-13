use amun_storage_kernel::smt::node::{Node, NodeHash};
use amun_storage_kernel::SparseMerkleTree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedNode {
    pub depth: usize,
    pub node_type: u8,
    pub node_hash: [u8; 32],
    pub data: Vec<u8>,
}

pub struct CanonicalTraversal {
    nodes: Vec<SerializedNode>,
}

impl CanonicalTraversal {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn traverse(&mut self, tree: &SparseMerkleTree) {
        self.visit_node(0, tree.root_node(), tree);
    }

    fn visit_node(&mut self, depth: usize, node_hash: NodeHash, tree: &SparseMerkleTree) {
        if depth >= 256 {
            return;
        }
        if node_hash == tree.empty_ladder[depth] {
            return;
        }

        if let Some(node) = tree.nodes.get(&node_hash) {
            let node_type = match &**node {
                Node::Leaf { .. } => 0u8,
                Node::Branch { .. } => 1u8,
            };

            let data = match &**node {
                Node::Leaf {
                    key_hash,
                    value_hash,
                    version,
                } => {
                    let mut w = amun_canonical_codec::CanonicalWriter::new();
                    w.write_u8(0x01);
                    w.write_hash(key_hash);
                    w.write_hash(value_hash);
                    w.write_u64(*version);
                    w.into_bytes()
                }
                Node::Branch { left, right } => {
                    let mut w = amun_canonical_codec::CanonicalWriter::new();
                    w.write_u8(0x02);
                    w.write_hash(&left.0);
                    w.write_hash(&right.0);
                    w.into_bytes()
                }
            };

            self.nodes.push(SerializedNode {
                depth,
                node_type,
                node_hash: node_hash.0,
                data,
            });

            if let Node::Branch { left, right } = &**node {
                self.visit_node(depth + 1, *left, tree);
                self.visit_node(depth + 1, *right, tree);
            }
        }
    }

    pub fn into_nodes(self) -> Vec<SerializedNode> {
        self.nodes
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SerializedNode> {
        self.nodes.iter()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

pub struct StreamingTraversal<'a> {
    tree: &'a SparseMerkleTree,
    stack: Vec<(usize, NodeHash)>,
    visited_count: usize,
}

impl<'a> StreamingTraversal<'a> {
    pub fn new(tree: &'a SparseMerkleTree) -> Self {
        let stack = vec![(0, tree.root_node())];
        Self {
            tree,
            stack,
            visited_count: 0,
        }
    }

    pub fn node_count(&self) -> usize {
        self.visited_count
    }
}

impl<'a> Iterator for StreamingTraversal<'a> {
    type Item = SerializedNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((depth, node_hash)) = self.stack.pop() {
            if depth > 256 {
                continue;
            }
            if node_hash == self.tree.empty_ladder[depth] {
                continue;
            }

            if let Some(node) = self.tree.nodes.get(&node_hash) {
                let node_type = match &**node {
                    Node::Leaf { .. } => 0u8,
                    Node::Branch { .. } => 1u8,
                };

                let data = match &**node {
                    Node::Leaf {
                        key_hash,
                        value_hash,
                        version,
                    } => {
                        let mut w = amun_canonical_codec::CanonicalWriter::new();
                        w.write_u8(0x01);
                        w.write_hash(key_hash);
                        w.write_hash(value_hash);
                        w.write_u64(*version);
                        w.into_bytes()
                    }
                    Node::Branch { left, right } => {
                        let mut w = amun_canonical_codec::CanonicalWriter::new();
                        w.write_u8(0x02);
                        w.write_hash(&left.0);
                        w.write_hash(&right.0);
                        w.into_bytes()
                    }
                };

                if let Node::Branch { left, right } = &**node {
                    self.stack.push((depth + 1, *right));
                    self.stack.push((depth + 1, *left));
                }

                self.visited_count += 1;
                return Some(SerializedNode {
                    depth,
                    node_type,
                    node_hash: node_hash.0,
                    data,
                });
            }
        }
        None
    }
}
