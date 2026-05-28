//! Constitutional Hashable Object - Single Source of Truth for Hashing

use crate::canonical::{CanonicalEncoder, CanonicalSerialize};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// Sealed trait - prevents external implementation
pub(crate) mod sealed {
    pub trait Sealed {}
    
    impl Sealed for u64 {}
    impl Sealed for u32 {}
    impl Sealed for u8 {}
    impl Sealed for [u8; 32] {}
    impl Sealed for Vec<u8> {}
    impl Sealed for crate::state::AmunState {}
    impl Sealed for crate::state::TransferTransition {}
    impl Sealed for crate::constitutional::VerifiedTransitionWitness {}
    impl Sealed for crate::constitutional::ExecutionWitness {}
}

/// Constitutional Hashable Object - SEALED TRAIT
pub trait ConstitutionalHashable: sealed::Sealed + CanonicalSerialize {
    const DOMAIN_TAG: &'static [u8];
    
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut encoder = CanonicalEncoder::new();
        encoder.write_bytes(Self::DOMAIN_TAG);
        self.encode(&mut encoder);
        blake3::hash(&encoder.into_bytes()).into()
    }
}

// Core type implementations
impl ConstitutionalHashable for u64 { const DOMAIN_TAG: &'static [u8] = b"AMUN_U64_V1"; }
impl ConstitutionalHashable for u32 { const DOMAIN_TAG: &'static [u8] = b"AMUN_U32_V1"; }
impl ConstitutionalHashable for u8 { const DOMAIN_TAG: &'static [u8] = b"AMUN_U8_V1"; }
impl ConstitutionalHashable for [u8; 32] { const DOMAIN_TAG: &'static [u8] = b"AMUN_HASH_V1"; }
impl ConstitutionalHashable for Vec<u8> { const DOMAIN_TAG: &'static [u8] = b"AMUN_BYTES_V1"; }

/// Constitutional State trait
pub trait ConstitutionalState: ConstitutionalHashable + Sized {
    type Transition: ConstitutionalTransition<State = Self>;
    fn apply_transition(self, transition: &Self::Transition) -> Result<Self, &'static str>;
    fn state_hash(&self) -> [u8; 32] { self.constitutional_hash() }
}

/// Constitutional Transition trait
pub trait ConstitutionalTransition: ConstitutionalHashable {
    type State: ConstitutionalState;
    fn verify(&self, pre_state: &Self::State) -> bool;
    fn compute_post_hash(&self, pre_hash: [u8; 32]) -> [u8; 32];
}

// Sealed for state_tree types
impl Sealed for crate::state_tree::StateRoot {}
impl Sealed for crate::state_tree::Key256 {}
impl Sealed for crate::state_tree::ValueBlob {}
impl Sealed for crate::state_tree::journal::Transition {}
