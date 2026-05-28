//! Canonical implementations for fixed-size arrays

use super::{CanonicalEncoder, CanonicalDecoder, CanonicalSerialize, CanonicalDeserialize};

impl CanonicalSerialize for [u8; 32] {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_fixed_hash(self);
    }
}

impl CanonicalDeserialize for [u8; 32] {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        decoder.read_fixed_hash()
    }
}
