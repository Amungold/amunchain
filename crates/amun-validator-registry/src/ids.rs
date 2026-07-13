#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidatorId(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicKey(pub [u8; 32]);

impl ValidatorId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl PeerId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl PublicKey {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
