//! Constitutional Canonical Serialization Traits

use super::CanonicalEncoder;
use super::CanonicalDecoder;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub trait CanonicalSerialize {
    fn encode(&self, encoder: &mut CanonicalEncoder);
    fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = CanonicalEncoder::new();
        self.encode(&mut encoder);
        encoder.into_bytes()
    }
}

pub trait CanonicalDeserialize: Sized {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self>;
    fn from_canonical_bytes(bytes: &[u8]) -> Option<Self> {
        let mut decoder = CanonicalDecoder::new(bytes);
        let result = Self::decode(&mut decoder);
        if decoder.is_exhausted() { result } else { None }
    }
}
