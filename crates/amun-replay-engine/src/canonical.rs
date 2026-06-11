// ============================================================================
// CANONICAL BYTE CONTRACT — LAYER ZERO OF REPLAY PHYSICS
// ============================================================================
//
// This file is the constitutional foundation of the entire replay system.
// Without a stable, deterministic byte representation, replay equivalence
// cannot be proven.
//
// RULES (immutable):
//   1. Big-endian for all multi-byte integers. Always.
//   2. Length-prefixed for all variable-length data. No exceptions.
//   3. No floating point. No system time. No pointers. No RNG.
//   4. If two values are semantically equal, their canonical bytes
//      MUST be bit-for-bit identical.
//
// DEPENDENCY DIRECTION:
//   canonical.rs is depended upon by everything above it.
//   canonical.rs depends on NOTHING except alloc and sha2.

extern crate alloc;
use alloc::vec::Vec;

use amun_constitutional::ConstitutionalHash;

// ────────────────────────────────────────────────────────────────────────────
// TRAIT: CanonicalEncode
// ────────────────────────────────────────────────────────────────────────────

pub trait CanonicalEncode {
    fn canonical_encode(&self) -> Vec<u8>;
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter);
}

// ────────────────────────────────────────────────────────────────────────────
// STRUCT: CanonicalWriter
// ────────────────────────────────────────────────────────────────────────────

pub struct CanonicalWriter {
    buffer: Vec<u8>,
}

impl Default for CanonicalWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_u64(&mut self, v: u64) {
        self.buffer.extend_from_slice(&v.to_be_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buffer.extend_from_slice(&v.to_be_bytes());
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buffer.push(v);
    }

    pub fn write_bool(&mut self, v: bool) {
        self.buffer.push(if v { 1 } else { 0 });
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u64(bytes.len() as u64);
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_fixed_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_hash(&mut self, hash: &ConstitutionalHash) {
        self.write_fixed_bytes(hash);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

// ────────────────────────────────────────────────────────────────────────────
// STRUCT: CanonicalReader
// ────────────────────────────────────────────────────────────────────────────

pub struct CanonicalReader<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> CanonicalReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len().saturating_sub(self.position)
    }

    pub fn read_u64(&mut self) -> Result<u64, CanonicalError> {
        if self.remaining() < 8 {
            return Err(CanonicalError::UnexpectedEnd);
        }
        let slice = &self.buffer[self.position..self.position + 8];
        let bytes: [u8; 8] = slice
            .try_into()
            .expect("canonical: fixed-size conversion failed");
        self.position += 8;
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn read_hash(&mut self) -> Result<ConstitutionalHash, CanonicalError> {
        if self.remaining() < 32 {
            return Err(CanonicalError::UnexpectedEnd);
        }
        let slice = &self.buffer[self.position..self.position + 32];
        let hash: ConstitutionalHash = slice
            .try_into()
            .expect("canonical: fixed-size conversion failed");
        self.position += 32;
        Ok(hash)
    }

    pub fn read_bytes(&mut self) -> Result<&'a [u8], CanonicalError> {
        let len = self.read_u64()? as usize;
        if self.remaining() < len {
            return Err(CanonicalError::UnexpectedEnd);
        }
        let bytes = &self.buffer[self.position..self.position + len];
        self.position += len;
        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    UnexpectedEnd,
    InvalidFormat,
}

// ────────────────────────────────────────────────────────────────────────────
// STRUCT: CanonicalHasher
// ────────────────────────────────────────────────────────────────────────────

pub struct CanonicalHasher {
    buffer: Vec<u8>,
}

impl Default for CanonicalHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalHasher {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn update<T: CanonicalEncode>(&mut self, item: &T) {
        let mut writer = CanonicalWriter::new();
        item.canonical_encode_into(&mut writer);
        self.buffer.extend_from_slice(writer.as_bytes());
    }

    pub fn finalize(&self) -> ConstitutionalHash {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&self.buffer);
        let result: [u8; 32] = hasher.finalize().into();
        result
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TRAIT IMPLEMENTATIONS
// ────────────────────────────────────────────────────────────────────────────
//
// NOTE: ConstitutionalHash IS [u8; 32], so we implement ONLY for [u8; 32].
// No duplicate impl. The type alias resolves to the same concrete type.

impl CanonicalEncode for u64 {
    fn canonical_encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
        buf.write_u64(*self);
    }
}

impl CanonicalEncode for u32 {
    fn canonical_encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
        buf.write_u32(*self);
    }
}

impl CanonicalEncode for u8 {
    fn canonical_encode(&self) -> Vec<u8> {
        vec![*self]
    }
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
        buf.write_u8(*self);
    }
}

impl CanonicalEncode for bool {
    fn canonical_encode(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
        buf.write_bool(*self);
    }
}

// SINGLE implementation for [u8; 32] (which IS ConstitutionalHash)
impl CanonicalEncode for [u8; 32] {
    fn canonical_encode(&self) -> Vec<u8> {
        self.to_vec()
    }
    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
        buf.write_fixed_bytes(self);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TESTS
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_roundtrip() {
        let v: u64 = 0xDEAD_BEEF_CAFE_BABE;
        let encoded = v.canonical_encode();
        let mut reader = CanonicalReader::new(&encoded);
        assert_eq!(reader.read_u64().unwrap(), v);
    }

    #[test]
    fn hash_roundtrip() {
        let hash: ConstitutionalHash = [0xAB; 32];
        let encoded = hash.canonical_encode();
        let mut reader = CanonicalReader::new(&encoded);
        assert_eq!(reader.read_hash().unwrap(), hash);
    }

    #[test]
    fn hasher_is_deterministic() {
        let mut h1 = CanonicalHasher::new();
        h1.update(&42u64);
        h1.update(&true);
        let r1 = h1.finalize();

        let mut h2 = CanonicalHasher::new();
        h2.update(&42u64);
        h2.update(&true);
        let r2 = h2.finalize();

        assert_eq!(r1, r2);
    }

    #[test]
    fn hasher_is_order_sensitive() {
        let mut h1 = CanonicalHasher::new();
        h1.update(&42u64);
        h1.update(&99u64);

        let mut h2 = CanonicalHasher::new();
        h2.update(&99u64);
        h2.update(&42u64);
    }
}
