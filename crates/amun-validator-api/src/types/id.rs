#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ValidatorId(pub [u8; 32]);

impl ValidatorId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        ValidatorId(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        PeerId(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        PublicKey(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FindingId(pub [u8; 16]);

impl FindingId {
    pub fn generate() -> Self {
        let mut id = [0u8; 16];
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let nanos = now.as_nanos().to_le_bytes();
        id[..8].copy_from_slice(&nanos);
        FindingId(id)
    }
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        FindingId(bytes)
    }
}
