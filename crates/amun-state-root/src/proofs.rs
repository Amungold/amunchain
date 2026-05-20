use amun_kernel::hashing::domain_tags;
use sha2::{Digest, Sha256};

pub struct MerkleProof {
    pub leaf: Vec<u8>,
    pub path: Vec<(bool, [u8; 32])>,
}

impl MerkleProof {
    pub fn verify(&self, expected_root: &[u8; 32]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(domain_tags::MERKLE_LEAF);
        hasher.update(&self.leaf);
        let mut hash = hasher.finalize();

        for (is_left, sibling) in &self.path {
            let mut hasher = Sha256::new();
            hasher.update(domain_tags::MERKLE_NODE);
            if *is_left {
                hasher.update(hash);
                hasher.update(sibling);
            } else {
                hasher.update(sibling);
                hasher.update(hash);
            }
            hash = hasher.finalize();
        }
        hash.as_slice() == expected_root.as_slice()
    }
}
