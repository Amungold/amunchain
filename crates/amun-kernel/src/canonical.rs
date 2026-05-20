use sha2::{Digest, Sha256};

/// Canonical encoding version
pub const CANONICAL_VERSION: u8 = 1;

/// Trait for deterministic, fixed‑width encoding.
pub trait CanonicalEncode {
    fn encode_canonical(&self, out: &mut Vec<u8>);
}

// Allow &T to be encoded by dereferencing
impl<T: CanonicalEncode + ?Sized> CanonicalEncode for &T {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (*self).encode_canonical(out);
    }
}

// Primitives
impl CanonicalEncode for u64 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl CanonicalEncode for u32 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl CanonicalEncode for u8 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }
}
impl CanonicalEncode for [u8; 32] {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
}

// Variable-length items with length prefix
impl CanonicalEncode for Vec<u8> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (self.len() as u64).encode_canonical(out);
        out.extend_from_slice(self);
    }
}
impl CanonicalEncode for str {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        let len = self.len() as u64;
        len.encode_canonical(out);
        out.extend_from_slice(self.as_bytes());
    }
}
impl CanonicalEncode for String {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.as_str().encode_canonical(out);
    }
}

// Framed tuple encoding
impl<A: CanonicalEncode, B: CanonicalEncode> CanonicalEncode for (A, B) {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        let mut tmp = Vec::new();
        self.0.encode_canonical(&mut tmp);
        (tmp.len() as u64).encode_canonical(out);
        out.extend_from_slice(&tmp);
        tmp.clear();
        self.1.encode_canonical(&mut tmp);
        (tmp.len() as u64).encode_canonical(out);
        out.extend_from_slice(&tmp);
    }
}

/// Helper to encode a length-prefixed slice of hash arrays.
pub fn encode_hash_slice(hashes: &[[u8; 32]], out: &mut Vec<u8>) {
    (hashes.len() as u64).encode_canonical(out);
    for h in hashes {
        out.extend_from_slice(h);
    }
}

/// Canonical hashing with domain separation.
pub struct CanonicalEncoder;

impl CanonicalEncoder {
    /// Hash an **already sorted** collection.
    /// # Panics
    /// Panics if the items are not in ascending order.
    pub fn hash_sorted<I>(items: I, domain_tag: &[u8]) -> [u8; 32]
    where
        I: IntoIterator,
        I::Item: CanonicalEncode + Ord,
    {
        let mut hasher = Sha256::new();
        hasher.update(domain_tag);
        let mut buf = Vec::new();
        let mut iter = items.into_iter();
        if let Some(first) = iter.next() {
            let mut prev = first;
            prev.encode_canonical(&mut buf);
            hasher.update(&buf);
            buf.clear();
            for item in iter {
                // Constitutional invariant: must be sorted
                assert!(prev <= item, "hash_sorted: items not sorted");
                item.encode_canonical(&mut buf);
                hasher.update(&buf);
                buf.clear();
                prev = item;
            }
        }
        hasher.finalize().into()
    }

    /// Sorts and then hashes.
    pub fn hash_unsorted_canonicalized<T>(items: &[T], domain_tag: &[u8]) -> [u8; 32]
    where
        T: CanonicalEncode + Ord + Clone,
    {
        let mut sorted = items.to_vec();
        sorted.sort();
        Self::hash_sorted(sorted.iter(), domain_tag)
    }

    /// Hash a single value with domain tag and version prefix.
    pub fn hash_value<V: CanonicalEncode>(value: &V, domain_tag: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(domain_tag);
        let mut buf = Vec::new();
        CANONICAL_VERSION.encode_canonical(&mut buf);
        value.encode_canonical(&mut buf);
        hasher.update(&buf);
        hasher.finalize().into()
    }
}
