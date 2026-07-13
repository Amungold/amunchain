//! Canonical Collections — deterministic, replay-safe container types.
//! Layer 1 — Deterministic Infrastructure (alloc-based, no_std compatible).
//!
//! Constitutional guarantees:
//!   - DETERMINISTIC: same operations → same iteration order always
//!   - CANONICAL: encoding is deterministic and domain-separated
//!   - BOUNDED: capacity limits enforced by default (1M entries)
//!   - REPLAY-SAFE: canonical_root() produces verifiable commitments

extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec::Vec;
use amun_kernel::canonical::CanonicalEncode;
use sha2::{Sha256, Digest};

// ─── Constitutional Traits ─────────────────────────────────
pub trait DeterministicCollection { fn is_deterministic(&self) -> bool { true } }
pub trait ReplaySafe: DeterministicCollection + CanonicalEncode {
    fn canonical_root(&self) -> [u8; 32];
    fn verify_root(&self, expected: &[u8; 32]) -> bool { self.canonical_root() == *expected }
    fn is_replay_stable(&self) -> bool { true }
}
pub trait BoundedCollection {
    fn capacity(&self) -> usize;
    fn remaining(&self) -> usize;
    fn is_full(&self) -> bool { self.remaining() == 0 }
}

// ─── Constants ─────────────────────────────────────────────
const DEFAULT_MAX_CAPACITY: usize = 1_048_576;
const SET_DOMAIN: &[u8] = b"AMUN|CSET|V1";
const MAP_DOMAIN: &[u8] = b"AMUN|CMAP|V1";
const DEQUE_DOMAIN: &[u8] = b"AMUN|CDEQ|V1";

// ─── Error ─────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionError {
    CapacityExceeded,
    EncodingTooLarge { len: usize, max: usize },
    InvalidCapacity { requested: usize },
}

// ─── CanonicalSet<T> ───────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalSet<T: Ord> { inner: BTreeSet<T>, max_capacity: usize }

impl<T: Ord> CanonicalSet<T> {
    pub fn new() -> Self { Self { inner: BTreeSet::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
        if max == 0 { return Err(CollectionError::InvalidCapacity { requested: max }); }
        Ok(Self { inner: BTreeSet::new(), max_capacity: max })
    }
    #[cfg(test)] pub fn with_capacity(max: usize) -> Self { Self::try_with_capacity(max).expect("capacity") }
    pub fn insert(&mut self, value: T) -> Result<bool, CollectionError> {
        if self.inner.len() >= self.max_capacity && !self.inner.contains(&value) { return Err(CollectionError::CapacityExceeded); }
        Ok(self.inner.insert(value))
    }
    pub fn contains(&self, value: &T) -> bool { self.inner.contains(value) }
    pub fn remove(&mut self, value: &T) -> bool { self.inner.remove(value) }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.inner.iter() }
    pub fn retain(&mut self, f: impl Fn(&T) -> bool) { self.inner.retain(f); }
    pub fn clear(&mut self) { self.inner.clear(); }
}

impl<T: Ord> DeterministicCollection for CanonicalSet<T> {}
impl<T: Ord> BoundedCollection for CanonicalSet<T> {
    fn capacity(&self) -> usize { self.max_capacity }
    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
}
impl<T: Ord + CanonicalEncode> ReplaySafe for CanonicalSet<T> {
    fn canonical_root(&self) -> [u8; 32] {
        let mut h = Sha256::new(); h.update(SET_DOMAIN); h.update((self.len() as u64).to_le_bytes());
        for item in &self.inner { h.update(item.canonical_encode()); }
        h.finalize().into()
    }
}
impl<T: Ord + CanonicalEncode> CanonicalEncode for CanonicalSet<T> {
    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for item in &self.inner { item.encode_canonical(out); } }
}
impl<T: Ord> Default for CanonicalSet<T> { fn default() -> Self { Self::new() } }
impl<T: Ord> IntoIterator for CanonicalSet<T> {
    type Item = T; type IntoIter = alloc::collections::btree_set::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}

// ─── CanonicalMap<K,V> ─────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalMap<K: Ord, V> { inner: BTreeMap<K, V>, max_capacity: usize }

impl<K: Ord, V> CanonicalMap<K, V> {
    pub fn new() -> Self { Self { inner: BTreeMap::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
        if max == 0 { return Err(CollectionError::InvalidCapacity { requested: max }); }
        Ok(Self { inner: BTreeMap::new(), max_capacity: max })
    }
    #[cfg(test)] pub fn with_capacity(max: usize) -> Self { Self::try_with_capacity(max).expect("capacity") }
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, CollectionError> {
        if self.inner.len() >= self.max_capacity && !self.inner.contains_key(&key) { return Err(CollectionError::CapacityExceeded); }
        Ok(self.inner.insert(key, value))
    }
    pub fn get(&self, key: &K) -> Option<&V> { self.inner.get(key) }
    pub fn contains_key(&self, key: &K) -> bool { self.inner.contains_key(key) }
    pub fn remove(&mut self, key: &K) -> Option<V> { self.inner.remove(key) }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> { self.inner.iter() }
    pub fn keys(&self) -> impl Iterator<Item = &K> { self.inner.keys() }
    pub fn values(&self) -> impl Iterator<Item = &V> { self.inner.values() }
    pub fn retain(&mut self, f: impl Fn(&K, &mut V) -> bool) { self.inner.retain(f); }
    pub fn clear(&mut self) { self.inner.clear(); }
}

impl<K: Ord, V> DeterministicCollection for CanonicalMap<K, V> {}
impl<K: Ord, V> BoundedCollection for CanonicalMap<K, V> {
    fn capacity(&self) -> usize { self.max_capacity }
    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
}
impl<K: Ord + CanonicalEncode, V: CanonicalEncode> ReplaySafe for CanonicalMap<K, V> {
    fn canonical_root(&self) -> [u8; 32] {
        let mut h = Sha256::new(); h.update(MAP_DOMAIN); h.update((self.len() as u64).to_le_bytes());
        for (k, v) in &self.inner { h.update(k.canonical_encode()); h.update(v.canonical_encode()); }
        h.finalize().into()
    }
}
impl<K: Ord + CanonicalEncode, V: CanonicalEncode> CanonicalEncode for CanonicalMap<K, V> {
    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for (k, v) in &self.inner { k.encode_canonical(out); v.encode_canonical(out); } }
}
impl<K: Ord, V> Default for CanonicalMap<K, V> { fn default() -> Self { Self::new() } }

// ─── CanonicalDeque<T> ─────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDeque<T> { inner: VecDeque<T>, max_capacity: usize }

impl<T> CanonicalDeque<T> {
    pub fn new() -> Self { Self { inner: VecDeque::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
        if max == 0 { return Err(CollectionError::InvalidCapacity { requested: max }); }
        Ok(Self { inner: VecDeque::new(), max_capacity: max })
    }
    #[cfg(test)] pub fn with_capacity(max: usize) -> Self { Self::try_with_capacity(max).expect("capacity") }
    pub fn push_back(&mut self, value: T) -> Result<(), CollectionError> { if self.inner.len() >= self.max_capacity { return Err(CollectionError::CapacityExceeded); } self.inner.push_back(value); Ok(()) }
    pub fn push_front(&mut self, value: T) -> Result<(), CollectionError> { if self.inner.len() >= self.max_capacity { return Err(CollectionError::CapacityExceeded); } self.inner.push_front(value); Ok(()) }
    pub fn pop_front(&mut self) -> Option<T> { self.inner.pop_front() }
    pub fn pop_back(&mut self) -> Option<T> { self.inner.pop_back() }
    pub fn front(&self) -> Option<&T> { self.inner.front() }
    pub fn back(&self) -> Option<&T> { self.inner.back() }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.inner.iter() }
    pub fn clear(&mut self) { self.inner.clear(); }
}

impl<T> DeterministicCollection for CanonicalDeque<T> {}
impl<T> BoundedCollection for CanonicalDeque<T> {
    fn capacity(&self) -> usize { self.max_capacity }
    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
}
impl<T: CanonicalEncode> ReplaySafe for CanonicalDeque<T> {
    fn canonical_root(&self) -> [u8; 32] {
        let mut h = Sha256::new(); h.update(DEQUE_DOMAIN); h.update((self.len() as u64).to_le_bytes());
        for item in &self.inner { h.update(item.canonical_encode()); }
        h.finalize().into()
    }
}
impl<T: CanonicalEncode> CanonicalEncode for CanonicalDeque<T> {
    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for item in &self.inner { item.encode_canonical(out); } }
}
impl<T> Default for CanonicalDeque<T> { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_set_deterministic() { let mut s = CanonicalSet::new(); s.insert(3u64).unwrap(); s.insert(1u64).unwrap(); s.insert(2u64).unwrap(); assert_eq!(s.iter().copied().collect::<Vec<_>>(), vec![1,2,3]); }
    #[test] fn test_set_capacity() { let mut s = CanonicalSet::with_capacity(2); s.insert(1u64).unwrap(); s.insert(2u64).unwrap(); assert!(s.insert(3u64).is_err()); }
    #[test] fn test_set_default_not_unbounded() { assert_eq!(CanonicalSet::<u64>::new().capacity(), 1_048_576); }
    #[test] fn test_rejects_zero_capacity() { assert!(CanonicalSet::<u64>::try_with_capacity(0).is_err()); }
    #[test] fn test_replay_root_deterministic() { let mut a = CanonicalSet::new(); let mut b = CanonicalSet::new(); a.insert(1u64).unwrap(); a.insert(2u64).unwrap(); b.insert(2u64).unwrap(); b.insert(1u64).unwrap(); assert_eq!(a.canonical_root(), b.canonical_root()); }
    #[test] fn test_map_deterministic() { let mut m = CanonicalMap::new(); m.insert("c",3).unwrap(); m.insert("a",1).unwrap(); m.insert("b",2).unwrap(); assert_eq!(m.keys().copied().collect::<Vec<_>>(), vec!["a","b","c"]); }
    #[test] fn test_deque_fifo() { let mut d = CanonicalDeque::new(); d.push_back(1).unwrap(); d.push_back(2).unwrap(); d.push_back(3).unwrap(); assert_eq!(d.pop_front(), Some(1)); assert_eq!(d.pop_front(), Some(2)); assert_eq!(d.pop_front(), Some(3)); }
    #[test] fn test_domain_separation() { let mut s = CanonicalSet::new(); s.insert(1u64).unwrap(); let mut m = CanonicalMap::new(); m.insert(1u64,1u64).unwrap(); assert_ne!(s.canonical_root(), m.canonical_root()); }
    #[test] fn test_full_detection() { let mut s = CanonicalSet::with_capacity(1); s.insert(1u64).unwrap(); assert!(s.is_full()); }
}
