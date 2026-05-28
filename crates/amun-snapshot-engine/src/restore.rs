use amun_canonical_codec::CanonicalReader;
use amun_storage_kernel::smt::node::{Node, NodeHash};
use amun_storage_kernel::SparseMerkleTree;
use std::collections::HashMap;
use std::sync::Arc;

use super::chunk::ChunkIndex;
use super::snapshot::SerializedNode;
use super::structural_verifier::StructuralVerifier;

pub struct SnapshotRestoreEngine;

impl SnapshotRestoreEngine {
    pub fn restore(chunks: &ChunkIndex) -> Result<SparseMerkleTree, RestoreError> {
        let mut nodes_map: HashMap<NodeHash, Arc<Node>> = HashMap::new();
        let mut depth_map: HashMap<NodeHash, usize> = HashMap::new();
        let mut root_hash: Option<NodeHash> = None;

        for chunk in &chunks.chunks {
            for serialized in &chunk.nodes {
                let node = Self::deserialize_node(serialized)?;
                let computed_hash = node.hash();
                let claimed_hash = NodeHash(serialized.node_hash);

                // Self-hash verification
                if computed_hash != claimed_hash {
                    return Err(RestoreError::HashMismatch {
                        claimed: serialized.node_hash,
                        computed: computed_hash.0,
                        depth: serialized.depth,
                    });
                }

                // Constitutional depth uniqueness: same node hash must have
                // exactly one depth. Conflicting depths indicate corruption.
                if let Some(&existing_depth) = depth_map.get(&claimed_hash) {
                    if existing_depth != serialized.depth {
                        return Err(RestoreError::DepthConflict {
                            node_hash: claimed_hash.0,
                            depth_a: existing_depth,
                            depth_b: serialized.depth,
                        });
                    }
                }

                // Duplicate root detection
                if serialized.depth == 0 {
                    if root_hash.is_some() && root_hash.unwrap() != claimed_hash {
                        return Err(RestoreError::DuplicateRoot {
                            first: root_hash.unwrap().0,
                            second: claimed_hash.0,
                        });
                    }
                    root_hash = Some(claimed_hash);
                }

                depth_map.insert(claimed_hash, serialized.depth);
                nodes_map.insert(claimed_hash, Arc::new(node));
            }
        }

        let root = root_hash.ok_or(RestoreError::MissingRoot)?;

        // Structural verification with depth validation
        StructuralVerifier::verify(&nodes_map, &root, &depth_map)
            .map_err(|e| RestoreError::StructuralFault(e.to_string()))?;

        let tree = SparseMerkleTree {
            nodes: Arc::new(nodes_map.into_iter().collect()),
            root,
            empty_ladder: SparseMerkleTree::empty().empty_ladder,
        };

        Ok(tree)
    }

    fn deserialize_node(serialized: &SerializedNode) -> Result<Node, RestoreError> {
        let mut r = CanonicalReader::new(&serialized.data);
        let tag = r.read_u8().ok_or(RestoreError::InvalidFormat)?;
        match tag {
            0x01 => {
                let key_hash = r.read_hash().ok_or(RestoreError::InvalidFormat)?;
                let value_hash = r.read_hash().ok_or(RestoreError::InvalidFormat)?;
                let version = r.read_u64().ok_or(RestoreError::InvalidFormat)?;
                if !r.is_finished() {
                    return Err(RestoreError::InvalidFormat);
                }
                Ok(Node::Leaf {
                    key_hash,
                    value_hash,
                    version,
                })
            }
            0x02 => {
                let left = r.read_hash().ok_or(RestoreError::InvalidFormat)?;
                let right = r.read_hash().ok_or(RestoreError::InvalidFormat)?;
                if !r.is_finished() {
                    return Err(RestoreError::InvalidFormat);
                }
                Ok(Node::Branch {
                    left: NodeHash(left),
                    right: NodeHash(right),
                })
            }
            _ => Err(RestoreError::InvalidFormat),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreError {
    HashMismatch {
        claimed: [u8; 32],
        computed: [u8; 32],
        depth: usize,
    },
    DepthConflict {
        node_hash: [u8; 32],
        depth_a: usize,
        depth_b: usize,
    },
    MissingRoot,
    DuplicateRoot {
        first: [u8; 32],
        second: [u8; 32],
    },
    InvalidFormat,
    StructuralFault(String),
}

impl std::fmt::Display for RestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestoreError::HashMismatch {
                claimed,
                computed,
                depth,
            } => {
                write!(
                    f,
                    "Hash mismatch at depth {}: claimed {:?}, computed {:?}",
                    depth,
                    &claimed[..8],
                    &computed[..8]
                )
            }
            RestoreError::DepthConflict {
                node_hash,
                depth_a,
                depth_b,
            } => {
                write!(
                    f,
                    "Depth conflict for node {:?}: claimed both depth {} and depth {}",
                    &node_hash[..8],
                    depth_a,
                    depth_b
                )
            }
            RestoreError::MissingRoot => write!(f, "No root node found in snapshot"),
            RestoreError::DuplicateRoot { first, second } => {
                write!(
                    f,
                    "Duplicate root nodes: {:?} and {:?}",
                    &first[..8],
                    &second[..8]
                )
            }
            RestoreError::InvalidFormat => write!(f, "Invalid node format in snapshot"),
            RestoreError::StructuralFault(e) => write!(f, "Structural fault: {}", e),
        }
    }
}
