//! Canonical Serialization — deterministic byte representation.
//!
//! INVARIANT: Identical constitutional semantics MUST produce
//! identical canonical bytes across all platforms and runtimes.
//!
//! Rules:
//!   - Fixed endianness (little-endian for u16/u32/u64)
//!   - Fixed field ordering (declaration order)
//!   - No optional ambiguity (presence bits for Option types)
//!   - No platform-dependent encoding

use crate::prelude::*;

/// Trait for objects that can be canonically serialized.
pub trait CanonicalEncode {
    /// Encode this object into canonical bytes.
    fn canonical_encode(&self) -> Vec<u8>;
}

/// Trait for objects that can be decoded from canonical bytes.
pub trait CanonicalDecode: Sized {
    /// Decode from canonical bytes.
    fn canonical_decode(bytes: &[u8]) -> Option<Self>;
}

/// Write a u64 in little-endian canonical form.
pub fn write_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// Write a u32 in little-endian canonical form.
pub fn write_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// Write a u16 in little-endian canonical form.
pub fn write_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// Write a u8.
pub fn write_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}

/// Write a byte slice with length prefix (u32 LE).
pub fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    write_u32(buf, bytes.len() as u32);
    buf.extend_from_slice(bytes);
}

/// Write a fixed-size hash.
pub fn write_hash(buf: &mut Vec<u8>, hash: &[u8; 32]) {
    buf.extend_from_slice(hash);
}

/// Write an optional hash with presence bit (0x00 = None, 0x01 = Some).
pub fn write_optional_hash(buf: &mut Vec<u8>, hash: Option<&[u8; 32]>) {
    match hash {
        None => write_u8(buf, 0x00),
        Some(h) => {
            write_u8(buf, 0x01);
            buf.extend_from_slice(h);
        }
    }
}

/// Read a u64 from canonical bytes.
pub fn read_u64(bytes: &[u8], pos: &mut usize) -> Option<u64> {
    if *pos + 8 > bytes.len() {
        return None;
    }
    let v = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().ok()?);
    *pos += 8;
    Some(v)
}

/// Read a u32 from canonical bytes.
pub fn read_u32(bytes: &[u8], pos: &mut usize) -> Option<u32> {
    if *pos + 4 > bytes.len() {
        return None;
    }
    let v = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?);
    *pos += 4;
    Some(v)
}

/// Read a u16 from canonical bytes.
pub fn read_u16(bytes: &[u8], pos: &mut usize) -> Option<u16> {
    if *pos + 2 > bytes.len() {
        return None;
    }
    let v = u16::from_le_bytes(bytes[*pos..*pos + 2].try_into().ok()?);
    *pos += 2;
    Some(v)
}

/// Read a u8 from canonical bytes.
pub fn read_u8(bytes: &[u8], pos: &mut usize) -> Option<u8> {
    if *pos >= bytes.len() {
        return None;
    }
    let v = bytes[*pos];
    *pos += 1;
    Some(v)
}

/// Read a byte slice with length prefix.
pub fn read_bytes(bytes: &[u8], pos: &mut usize) -> Option<Vec<u8>> {
    let len = read_u32(bytes, pos)? as usize;
    if *pos + len > bytes.len() {
        return None;
    }
    let v = bytes[*pos..*pos + len].to_vec();
    *pos += len;
    Some(v)
}

/// Read a fixed-size hash.
pub fn read_hash(bytes: &[u8], pos: &mut usize) -> Option<[u8; 32]> {
    if *pos + 32 > bytes.len() {
        return None;
    }
    let mut h = [0u8; 32];
    h.copy_from_slice(&bytes[*pos..*pos + 32]);
    *pos += 32;
    Some(h)
}

/// Read an optional hash with presence bit.
pub fn read_optional_hash(bytes: &[u8], pos: &mut usize) -> Option<Option<[u8; 32]>> {
    let presence = read_u8(bytes, pos)?;
    match presence {
        0x00 => Some(None),
        0x01 => read_hash(bytes, pos).map(Some),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_roundtrip() {
        let mut buf = Vec::new();
        write_u64(&mut buf, 0xDEAD_BEEF_CAFE_BABE);
        let mut pos = 0;
        assert_eq!(read_u64(&buf, &mut pos), Some(0xDEAD_BEEF_CAFE_BABE));
    }

    #[test]
    fn test_bytes_roundtrip() {
        let mut buf = Vec::new();
        write_bytes(&mut buf, b"hello canonical");
        let mut pos = 0;
        assert_eq!(
            read_bytes(&buf, &mut pos),
            Some(b"hello canonical".to_vec())
        );
    }

    #[test]
    fn test_optional_hash_some() {
        let mut buf = Vec::new();
        write_optional_hash(&mut buf, Some(&[0xAA; 32]));
        let mut pos = 0;
        assert_eq!(read_optional_hash(&buf, &mut pos), Some(Some([0xAA; 32])));
    }

    #[test]
    fn test_optional_hash_none() {
        let mut buf = Vec::new();
        write_optional_hash(&mut buf, None);
        let mut pos = 0;
        assert_eq!(read_optional_hash(&buf, &mut pos), Some(None));
    }

    #[test]
    fn test_deterministic_output() {
        let mut b1 = Vec::new();
        write_u64(&mut b1, 42);
        write_u32(&mut b1, 100);
        let mut b2 = Vec::new();
        write_u64(&mut b2, 42);
        write_u32(&mut b2, 100);
        assert_eq!(b1, b2);
    }
}
