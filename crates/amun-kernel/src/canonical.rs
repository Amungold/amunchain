use sha2::{Digest, Sha256};

/// Canonical encoding version
pub const CANONICAL_VERSION: u8 = 1;

/// Trait for deterministic, fixed-width encoding.
pub trait CanonicalEncode {
    fn canonical_encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_canonical(&mut out);
        out
    }

    fn encode_canonical(&self, out: &mut Vec<u8>);
}

// Allow &T to be encoded by dereferencing
impl<T: CanonicalEncode + ?Sized> CanonicalEncode for &T {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (*self).encode_canonical(out);
    }
}

// Canonical encoding for slices
impl<T: CanonicalEncode> CanonicalEncode for [T] {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (self.len() as u64).encode_canonical(out);
        for item in self {
            item.encode_canonical(out);
        }
    }
}

// Canonical encoding for vectors
impl<T: CanonicalEncode> CanonicalEncode for Vec<T> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (self.len() as u64).encode_canonical(out);
        for item in self {
            item.encode_canonical(out);
        }
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
impl CanonicalEncode for [u8; 64] {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
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
impl CanonicalEncode for bool {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.push(if *self { 1 } else { 0 });
    }
}

impl<T: CanonicalEncode> CanonicalEncode for Option<T> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        match self {
            Some(val) => {
                out.push(1);
                val.encode_canonical(out);
            }
            None => {
                out.push(0);
            }
        }
    }
}

// Framed tuple encoding
impl<A: CanonicalEncode, B: CanonicalEncode> CanonicalEncode for (A, B) {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.0.encode_canonical(out);
        self.1.encode_canonical(out);
    }
}

/// Helper for raw bytes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalBytes<'a>(pub &'a [u8]);

impl<'a> CanonicalEncode for CanonicalBytes<'a> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (self.0.len() as u64).encode_canonical(out);
        out.extend_from_slice(self.0);
    }
}

/// Compatibility helper for legacy hash-slice encoding (mutates output buffer)
pub fn encode_hash_slice(items: &[[u8; 32]], out: &mut Vec<u8>) {
    (items.len() as u64).encode_canonical(out);
    for item in items {
        item.encode_canonical(out);
    }
}

/// Canonical hashing with domain separation.
pub struct CanonicalEncoder;

impl CanonicalEncoder {
    /// Hash an already-sorted collection.
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
    pub fn hash_value<V: CanonicalEncode + ?Sized>(value: &V, domain_tag: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(domain_tag);
        let mut buf = Vec::new();
        CANONICAL_VERSION.encode_canonical(&mut buf);
        value.encode_canonical(&mut buf);
        hasher.update(&buf);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_hash() {
        let h1 = CanonicalEncoder::hash_value("hello", b"test");
        let h2 = CanonicalEncoder::hash_value("hello", b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_domain_separation() {
        let h1 = CanonicalEncoder::hash_value(&42u64, b"domain_a");
        let h2 = CanonicalEncoder::hash_value(&42u64, b"domain_b");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_vec_encoding() {
        let v: Vec<u64> = vec![1, 2, 3];
        let enc1 = v.canonical_encode();
        let enc2 = v.canonical_encode();
        assert_eq!(enc1, enc2);
    }

    #[test]
    fn test_optional_encoding() {
        let some: Option<u64> = Some(42);
        let none: Option<u64> = None;
        assert_ne!(some.canonical_encode(), none.canonical_encode());
    }
}
