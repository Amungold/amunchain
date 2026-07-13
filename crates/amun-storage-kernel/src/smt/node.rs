use crate::canonical::Encoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeHash(pub [u8; 32]);
impl NodeHash {
    pub const ZERO: Self = Self([0u8; 32]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Leaf {
        key_hash: [u8; 32],
        value_hash: [u8; 32],
        version: u64,
    },
    Branch {
        left: NodeHash,
        right: NodeHash,
    },
}

impl Node {
    pub fn hash(&self) -> NodeHash {
        let mut e = Encoder::new();
        match self {
            Node::Leaf {
                key_hash,
                value_hash,
                version,
            } => {
                // AMUN_LEAF_V1 || key_hash(32) || value_hash(32) || version(u64)
                e.write_bytes(b"AMUN_LEAF_V1");
                e.write_hash(key_hash);
                e.write_hash(value_hash);
                e.write_u64(*version);
            }
            Node::Branch { left, right } => {
                // AMUN_BRANCH_V1 || left(32) || right(32)
                e.write_bytes(b"AMUN_BRANCH_V1");
                e.write_hash(&left.0);
                e.write_hash(&right.0);
            }
        }
        NodeHash(blake3::hash(&e.into_bytes()).into())
    }
}
