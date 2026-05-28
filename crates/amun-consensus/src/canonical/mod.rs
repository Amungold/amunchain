//! Canonical Serialization for Deterministic Replay
//! Little-endian forever. Fixed format.

mod traits;
pub use traits::{CanonicalSerialize, CanonicalDeserialize};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::convert::TryInto;

// Constitutional memory bounds
const MAX_CANONICAL_VEC_BYTES: usize = 16 * 1024 * 1024;  // 16 MB

#[derive(Debug, Clone, Default)]
pub struct CanonicalEncoder {
    buf: Vec<u8>,
}

impl CanonicalEncoder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub(crate) fn raw_buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u64(bytes.len() as u64);
        self.buf.extend_from_slice(bytes);
    }

    /// Write a fixed 32-byte hash (NO length prefix)
    pub fn write_fixed_hash(&mut self, hash: &[u8; 32]) {
        self.buf.extend_from_slice(hash);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

#[derive(Debug)]
pub struct CanonicalDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CanonicalDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 2 > self.data.len() {
            return None;
        }
        let bytes: [u8; 2] = self.data[self.pos..self.pos+2].try_into().ok()?;
        self.pos += 2;
        Some(u16::from_le_bytes(bytes))
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if self.pos + 8 > self.data.len() {
            return None;
        }
        let bytes: [u8; 8] = self.data[self.pos..self.pos+8].try_into().ok()?;
        self.pos += 8;
        Some(u64::from_le_bytes(bytes))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 > self.data.len() {
            return None;
        }
        let bytes: [u8; 4] = self.data[self.pos..self.pos+4].try_into().ok()?;
        self.pos += 4;
        Some(u32::from_le_bytes(bytes))
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() {
            return None;
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Some(v)
    }

    /// Read a fixed 32-byte hash
    pub fn read_fixed_hash(&mut self) -> Option<[u8; 32]> {
        if self.pos + 32 > self.data.len() {
            return None;
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&self.data[self.pos..self.pos+32]);
        self.pos += 32;
        Some(hash)
    }

    pub fn read_bytes(&mut self) -> Option<Vec<u8>> {
        let len_u64 = self.read_u64()?;
        if len_u64 > usize::MAX as u64 {
            return None;
        }
        let len = len_u64 as usize;
        if len > MAX_CANONICAL_VEC_BYTES {
            return None;
        }
        let new_pos = self.pos.checked_add(len)?;
        if new_pos > self.data.len() {
            return None;
        }
        let bytes = self.data[self.pos..new_pos].to_vec();
        self.pos = new_pos;
        Some(bytes)
    }

    pub fn is_exhausted(&self) -> bool {
        self.pos >= self.data.len()
    }
}

// Include all implementations
mod impls;
mod array_impl;
