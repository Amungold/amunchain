//! Domain-separated hashing for Amun SMT V1.
//!
//! # Key Hashing Semantics
//!
//! `Key256` is a pre-image. The tree internally hashes it with BLAKE3
//! to produce the 256-bit trie key. External implementations MUST
//! replicate this exactly.

pub mod domain {
    pub const LEAF:   &[u8] = b"AMUN_LEAF_V1";
    pub const BRANCH: &[u8] = b"AMUN_BRANCH_V1";
    pub const EMPTY:  &[u8] = b"AMUN_EMPTY_V1";
    pub const ROOT:   &[u8] = b"AMUN_ROOT_V1";
}

/// 256-bit hash output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub const ZERO: Self = Self([0u8; 32]);
}

/// Canonical hash of an empty subtree: H(AMUN_EMPTY_V1).
pub fn empty_node_hash() -> Hash {
    Hash(blake3::hash(domain::EMPTY).into())
}

/// Hash a leaf: H(AMUN_LEAF_V1 || key_hash || value_hash || version).
pub fn hash_leaf(key_hash: &[u8; 32], value_hash: &[u8; 32], version: u64) -> Hash {
    let mut h = blake3::Hasher::new();
    h.update(domain::LEAF);
    h.update(key_hash);
    h.update(value_hash);
    h.update(&version.to_le_bytes());
    Hash(h.finalize().into())
}

/// Hash a branch (structural only).
/// `prefix` trailing bits MUST already be zeroed via `canonicalize_prefix`.
pub fn hash_branch(skip_len: u8, prefix: &[u8; 32], left: &Hash, right: &Hash) -> Hash {
    let byte_len = (skip_len as usize + 7) / 8;
    let mut h = blake3::Hasher::new();
    h.update(domain::BRANCH);
    h.update(&[skip_len]);
    h.update(&prefix[..byte_len]);
    h.update(&left.0);
    h.update(&right.0);
    Hash(h.finalize().into())
}

/// Hash the state root: H(AMUN_ROOT_V1 || internal_root_hash).
pub fn hash_root(internal_root: &Hash) -> Hash {
    let mut h = blake3::Hasher::new();
    h.update(domain::ROOT);
    h.update(&internal_root.0);
    Hash(h.finalize().into())
}

/// Extract bit `pos` (0..255) from a 32-byte hash.
/// Bits within each byte are big-endian (MSB first).
#[inline]
pub fn bit(hash: &[u8; 32], pos: usize) -> u8 {
    (hash[pos / 8] >> (7 - (pos % 8))) & 1
}

/// First bit index where two hashes differ (0..256).
pub fn find_divergence(a: &[u8; 32], b: &[u8; 32]) -> usize {
    for i in 0..256 {
        if bit(a, i) != bit(b, i) {
            return i;
        }
    }
    256
}

/// Maximum allowed skip_len at a given depth.
/// Guarantees: `depth + skip_len < 256` (a decision bit always exists).
#[inline]
pub fn max_skip_len(depth: usize) -> u8 {
    255u8.saturating_sub(depth as u8)
}

/// Zero trailing unused bits in a prefix byte array (canonical encoding).
pub fn canonicalize_prefix(prefix: &mut [u8; 32], skip_len: u8) {
    if skip_len == 0 {
        *prefix = [0u8; 32];
        return;
    }
    let byte_len = (skip_len as usize + 7) / 8;
    let rem = skip_len % 8;
    if rem != 0 && byte_len > 0 {
        let mask = 0xFFu8.wrapping_shl(8 - rem as u32);
        prefix[byte_len - 1] &= mask;
    }
    for b in prefix.iter_mut().skip(byte_len) {
        *b = 0;
    }
}

/// Pack prefix bits into (bytes, bit_len) for proofs.
/// Trailing unused bits are zeroed.
pub fn prefix_bytes(prefix: &[u8; 32], skip_len: u8) -> (Vec<u8>, u8) {
    let byte_len = (skip_len as usize + 7) / 8;
    let mut bytes = prefix[..byte_len].to_vec();
    let rem = skip_len % 8;
    if rem != 0 {
        if let Some(last) = bytes.last_mut() {
            *last &= 0xFFu8.wrapping_shl(8 - rem as u32);
        }
    }
    (bytes, skip_len)
}
