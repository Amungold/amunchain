use amun_codec::HashDomain;
use amun_kernel_types::PublicHash32;

// Constitutional Merkle domain separation.
// Frozen: these values must never change.

pub enum MerkleDomain {
    Leaf,
    Internal,
    Empty,
}

impl MerkleDomain {
    pub const fn as_hash_domain(&self) -> HashDomain {
        match self {
            Self::Leaf => HashDomain::StateCommitment,
            Self::Internal => HashDomain::StateCommitment,
            Self::Empty => HashDomain::StateCommitment,
        }
    }

    pub fn hash(&self, data: &[u8]) -> PublicHash32 {
        match self {
            Self::Leaf => HashDomain::StateCommitment.hash(data),
            Self::Internal => HashDomain::StateCommitment.hash(data),
            Self::Empty => HashDomain::StateCommitment.hash(b""),
        }
    }
}
