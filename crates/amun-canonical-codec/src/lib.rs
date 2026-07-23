#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]
pub mod constants;
pub mod hasher;
pub mod reader;
pub mod writer;

pub use constants::*;
pub use hasher::CanonicalHasher;
pub use reader::CanonicalReader;
pub use writer::CanonicalWriter;

/// Trait for types that can be canonically encoded (P0.1).
pub trait CanonicalEncode {
    fn canonical_encode(&self) -> Vec<u8>;
}

/// Trait for types that can be canonically decoded (P0.1).
pub trait CanonicalDecode: Sized {
    fn canonical_decode(data: &[u8]) -> Result<Self, String>;
}


pub const CANONICAL_CODEC_VERSION: u16 = 1;
pub const MAX_CANONICAL_ALLOCATION: u64 = 64 * 1024 * 1024; // 64MB constitutional limit

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_vector_u32() {
        let val = 42u32;
        let expected = vec![42, 0, 0, 0];
        let mut w = CanonicalWriter::new();
        w.write_u32(val);
        assert_eq!(w.into_bytes(), expected);
    }

    #[test]
    fn test_roundtrip_bool() {
        let mut w = CanonicalWriter::new();
        w.write_bool(true);
        w.write_bool(false);
        let bytes = w.into_bytes();
        let mut r = CanonicalReader::new(&bytes);
        assert_eq!(r.read_bool(), Some(true));
        assert_eq!(r.read_bool(), Some(false));
    }
}
