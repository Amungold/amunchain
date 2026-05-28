//! Typed cryptographic wrappers for consensus

use blake3;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignatureBytes(#[serde(with = "serde_bytes")] pub Vec<u8>);

impl SignatureBytes {
    pub fn from_64(bytes: [u8; 64]) -> Self { Self(bytes.to_vec()) }
    pub fn to_64(&self) -> Option<[u8; 64]> { if self.0.len() == 64 { let mut arr = [0u8; 64]; arr.copy_from_slice(&self.0); Some(arr) } else { None } }
    pub fn as_bytes(&self) -> &[u8] { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GenesisIdentity(pub [u8; 32]);

impl GenesisIdentity {
    pub fn compute(chain_id: &str, genesis_seed: [u8; 32], epoch: u64) -> Self {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"AMUN_GENESIS_V1");
        bytes.extend_from_slice(chain_id.as_bytes());
        bytes.extend_from_slice(&genesis_seed);
        bytes.extend_from_slice(&epoch.to_be_bytes());
        Self(blake3::hash(&bytes).into())
    }
    pub fn as_bytes(&self) -> [u8; 32] { self.0 }
    pub fn from_bytes(bytes: [u8; 32]) -> Self { Self(bytes) }
}

impl fmt::Display for GenesisIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Genesis({:02x}{:02x}{:02x}{:02x})", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeHash(pub [u8; 32]);
impl NodeHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self { Self(bytes) }
    pub fn as_bytes(&self) -> [u8; 32] { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConstitutionalEpoch(pub u64);
impl ConstitutionalEpoch {
    pub fn new(epoch: u64) -> Self { Self(epoch) }
    pub fn as_u64(&self) -> u64 { self.0 }
    pub fn increment(&self) -> Self { Self(self.0 + 1) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConstitutionalRound(pub u64);
impl ConstitutionalRound {
    pub fn new(round: u64) -> Self { Self(round) }
    pub fn as_u64(&self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConstitutionalHeight(pub u64);
impl ConstitutionalHeight {
    pub fn new(height: u64) -> Self { Self(height) }
    pub fn as_u64(&self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstitutionalHash(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AuthorityReference {
    Genesis(GenesisIdentity),
    Validator(u64),
}
impl AuthorityReference {
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Self::Genesis(id) => { bytes.push(0x00); bytes.extend_from_slice(&id.as_bytes()); }
            Self::Validator(v) => { bytes.push(0x01); bytes.extend_from_slice(&v.to_be_bytes()); }
        }
        bytes
    }
}
