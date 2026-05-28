use amun_storage_kernel::smt::node::{Node, NodeHash};
use amun_storage_kernel::SparseMerkleTree;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralError {
    DuplicateRoot {
        first: [u8; 32],
        second: [u8; 32],
    },
    MissingChild {
        parent: [u8; 32],
        missing_child: [u8; 32],
        depth: usize,
    },
    DepthMismatch {
        parent: [u8; 32],
        parent_depth: usize,
        child: [u8; 32],
        child_depth: usize,
    },
    OrphanNode {
        node_hash: [u8; 32],
        depth: usize,
    },
    UnreachableNode {
        node_hash: [u8; 32],
    },
    EmptyGraph,
}

impl std::fmt::Display for StructuralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructuralError::DuplicateRoot { first, second } => {
                write!(
                    f,
                    "Duplicate root nodes: {:?} and {:?}",
                    &first[..8],
                    &second[..8]
                )
            }
            StructuralError::MissingChild {
                parent,
                missing_child,
                depth,
            } => {
                write!(
                    f,
                    "Missing child {:?} referenced by parent {:?} at depth {}",
                    &missing_child[..8],
                    &parent[..8],
                    depth
                )
            }
            StructuralError::DepthMismatch {
                parent,
                parent_depth,
                child,
                child_depth,
            } => {
                write!(f, "Depth mismatch: parent {:?} at depth {} references child {:?} at depth {} (expected child at depth {})", &parent[..8], parent_depth, &child[..8], child_depth, parent_depth + 1)
            }
            StructuralError::OrphanNode { node_hash, depth } => {
                write!(f, "Orphan node {:?} at depth {}", &node_hash[..8], depth)
            }
            StructuralError::UnreachableNode { node_hash } => {
                write!(f, "Unreachable node {:?}", &node_hash[..8])
            }
            StructuralError::EmptyGraph => write!(f, "Empty snapshot graph"),
        }
    }
}

pub struct StructuralVerifier;

impl StructuralVerifier {
    /// Verify structural integrity including depth correctness.
    /// Every child must be at parent_depth + 1.
    pub fn verify(
        nodes: &HashMap<NodeHash, Arc<Node>>,
        root: &NodeHash,
        depth_map: &HashMap<NodeHash, usize>,
    ) -> Result<(), StructuralError> {
        if nodes.is_empty() {
            return Err(StructuralError::EmptyGraph);
        }

        let empty_ladder = &SparseMerkleTree::empty().empty_ladder;

        // Phase 1: Verify child existence and depth correctness
        for (parent_hash, node) in nodes {
            let parent_depth = depth_map.get(parent_hash).copied().unwrap_or(0);
            if let Node::Branch { left, right } = &**node {
                // Check left child
                if !nodes.contains_key(left) && !Self::is_empty_ladder_hash(left, empty_ladder) {
                    return Err(StructuralError::MissingChild {
                        parent: parent_hash.0,
                        missing_child: left.0,
                        depth: parent_depth,
                    });
                }
                if let Some(&child_depth) = depth_map.get(left) {
                    if child_depth != parent_depth + 1 {
                        return Err(StructuralError::DepthMismatch {
                            parent: parent_hash.0,
                            parent_depth,
                            child: left.0,
                            child_depth,
                        });
                    }
                }
                // Check right child
                if !nodes.contains_key(right) && !Self::is_empty_ladder_hash(right, empty_ladder) {
                    return Err(StructuralError::MissingChild {
                        parent: parent_hash.0,
                        missing_child: right.0,
                        depth: parent_depth,
                    });
                }
                if let Some(&child_depth) = depth_map.get(right) {
                    if child_depth != parent_depth + 1 {
                        return Err(StructuralError::DepthMismatch {
                            parent: parent_hash.0,
                            parent_depth,
                            child: right.0,
                            child_depth,
                        });
                    }
                }
            }
        }

        // Phase 2: Verify all nodes are reachable from root
        let mut visited = HashSet::new();
        Self::dfs_reachable(root, nodes, empty_ladder, &mut visited);

        if visited.len() != nodes.len() {
            for (hash, _) in nodes {
                if !visited.contains(hash) {
                    return Err(StructuralError::UnreachableNode { node_hash: hash.0 });
                }
            }
        }

        Ok(())
    }

    fn is_empty_ladder_hash(hash: &NodeHash, empty_ladder: &[NodeHash]) -> bool {
        empty_ladder.iter().any(|eh| eh == hash)
    }

    fn dfs_reachable(
        current: &NodeHash,
        nodes: &HashMap<NodeHash, Arc<Node>>,
        empty_ladder: &[NodeHash],
        visited: &mut HashSet<NodeHash>,
    ) {
        if visited.contains(current) {
            return;
        }
        if Self::is_empty_ladder_hash(current, empty_ladder) {
            return;
        }
        visited.insert(*current);
        if let Some(node) = nodes.get(current) {
            if let Node::Branch { left, right } = &**node {
                Self::dfs_reachable(left, nodes, empty_ladder, visited);
                Self::dfs_reachable(right, nodes, empty_ladder, visited);
            }
        }
    }

    pub fn verify_unique_root(
        nodes: &[(usize, NodeHash)],
        root: &NodeHash,
    ) -> Result<(), StructuralError> {
        let root_count = nodes.iter().filter(|(d, _)| *d == 0).count();
        if root_count > 1 {
            let first_root = nodes
                .iter()
                .find(|(d, _)| *d == 0)
                .map(|(_, h)| h.0)
                .unwrap_or([0u8; 32]);
            return Err(StructuralError::DuplicateRoot {
                first: first_root,
                second: root.0,
            });
        }
        Ok(())
    }
}
