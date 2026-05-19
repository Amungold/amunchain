use amun_kernel_types::PublicHash32;
use heapless::Vec;

// Constitutional Merkle tree capacity.
// Must be >= CONSTITUTIONAL_MAX_TX_COUNT (500) from amun-block.
pub const MAX_MERKLE_LEAVES: usize = 512;

pub struct MerkleTree;

impl MerkleTree {
    pub fn empty_root() -> PublicHash32 {
        crate::domain::MerkleDomain::Empty.hash(b"")
    }

    pub fn leaf_hash(data: &[u8]) -> PublicHash32 {
        crate::domain::MerkleDomain::Leaf.hash(data)
    }

    pub fn internal_hash(left: &PublicHash32, right: &PublicHash32) -> PublicHash32 {
        let mut buf = [0u8; 64];
        buf[..32].copy_from_slice(left.as_bytes());
        buf[32..].copy_from_slice(right.as_bytes());
        crate::domain::MerkleDomain::Internal.hash(&buf)
    }

    pub fn compute_root(leaves: &[PublicHash32]) -> PublicHash32 {
        if leaves.is_empty() {
            return Self::empty_root();
        }
        if leaves.len() == 1 {
            return leaves[0];
        }
        let mut current: Vec<PublicHash32, MAX_MERKLE_LEAVES> = Vec::new();
        for leaf in leaves {
            current.push(*leaf).ok();
        }
        while current.len() > 1 {
            let mut next: Vec<PublicHash32, MAX_MERKLE_LEAVES> = Vec::new();
            for chunk in current.chunks(2) {
                if chunk.len() == 2 {
                    next.push(Self::internal_hash(&chunk[0], &chunk[1])).ok();
                } else {
                    next.push(Self::internal_hash(&chunk[0], &chunk[0])).ok();
                }
            }
            current = next;
        }
        current[0]
    }
}
