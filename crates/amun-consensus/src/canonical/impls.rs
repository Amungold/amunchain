//! Canonical implementations for built-in types
//! SINGLE SOURCE - no duplicates

use super::{CanonicalEncoder, CanonicalDecoder, CanonicalSerialize, CanonicalDeserialize};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

impl CanonicalSerialize for u64 {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_u64(*self);
    }
}

impl CanonicalDeserialize for u64 {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        decoder.read_u64()
    }
}

impl CanonicalSerialize for u32 {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_u32(*self);
    }
}

impl CanonicalDeserialize for u32 {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        decoder.read_u32()
    }
}

impl CanonicalSerialize for u8 {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_u8(*self);
    }
}

impl CanonicalDeserialize for u8 {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        decoder.read_u8()
    }
}

impl CanonicalSerialize for Vec<u8> {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_bytes(self);
    }
}

impl CanonicalDeserialize for Vec<u8> {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        decoder.read_bytes()
    }
}
