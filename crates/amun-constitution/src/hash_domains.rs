#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HashDomain {
    Transaction = 0x01,
    BlockHeader = 0x02,
    StateLeaf = 0x03,
    Receipt = 0x04,
    MerkleInternal = 0x05,
    Vote = 0x06,
    QuorumCertificate = 0x07,
    Address = 0x08,
}

impl HashDomain {
    pub fn hash(&self, data: &[u8]) -> amun_kernel_types::PublicHash32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[*self as u8]);
        hasher.update(data);
        amun_kernel_types::PublicHash32::new(hasher.finalize().into())
    }
}
