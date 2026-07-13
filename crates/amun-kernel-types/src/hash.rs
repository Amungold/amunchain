use core::marker::PhantomData;
use zeroize::Zeroize;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PublicHash32(pub [u8; 32]);

impl PublicHash32 {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PublicHash32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for PublicHash32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PublicHash32({:02x}{:02x}..)", self.0[0], self.0[1])
    }
}

#[derive(Clone, PartialEq, Eq, Zeroize)]
#[zeroize(drop)]
#[derive(Default)]
pub struct SecretHash32(pub [u8; 32]);

impl SecretHash32 {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommitmentHash32(pub [u8; 32]);

impl CommitmentHash32 {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
impl AsRef<[u8]> for CommitmentHash32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash<Domain> {
    bytes: [u8; 32],
    _domain: PhantomData<Domain>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransactionDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoteDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuorumCertificateDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateCommitmentDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JournalEntryDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatorSetDomain;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochBoundaryDomain;

impl<D> Hash<D> {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _domain: PhantomData,
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<D> core::fmt::Debug for Hash<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Hash({:02x}{:02x}..)", self.bytes[0], self.bytes[1])
    }
}

impl<D> Default for Hash<D> {
    fn default() -> Self {
        Self {
            bytes: [0u8; 32],
            _domain: PhantomData,
        }
    }
}

pub type BlockHash = Hash<BlockDomain>;
pub type TxHash = Hash<TransactionDomain>;
pub type VoteHash = Hash<VoteDomain>;
pub type QcHash = Hash<QuorumCertificateDomain>;
pub type StateCommitment = Hash<StateCommitmentDomain>;
