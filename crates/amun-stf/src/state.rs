use amun_kernel_types::PublicHash32;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_constitution::hash_domains::HashDomain;
use heapless::Vec;

pub trait StateStore {
    fn get(&self, key: &[u8]) -> AmunResult<Option<Vec<u8, 256>>>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> AmunResult<()>;
    fn delete(&mut self, key: &[u8]) -> AmunResult<()>;
    fn root(&self) -> AmunResult<PublicHash32>;
}

// Constitutional state leaf hash — frozen domain-separated.
fn state_leaf_hash(key: &[u8], value: &[u8]) -> PublicHash32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&[HashDomain::StateLeaf as u8]);
    hasher.update(&(key.len() as u32).to_le_bytes());
    hasher.update(key);
    hasher.update(&(value.len() as u32).to_le_bytes());
    hasher.update(value);
    PublicHash32::new(hasher.finalize().into())
}

pub struct InMemoryState {
    entries: Vec<(Vec<u8, 64>, Vec<u8, 256>), 256>,
}

impl InMemoryState {
    pub fn new() -> Self { Self { entries: Vec::new() } }

    fn insert_sorted(&mut self, key: Vec<u8, 64>, value: Vec<u8, 256>) -> Result<(), FailureContext> {
        let exists = self.entries.iter().any(|(k, _)| k.as_slice() == key.as_slice());
        if !exists && self.entries.is_full() {
            return Err(FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0008, 0x0003));
        }
        self.delete_internal(key.as_slice())?;
        let pos = self.entries.iter()
            .position(|(k, _)| k.as_slice() > key.as_slice())
            .unwrap_or(self.entries.len());
        self.entries.insert(pos, (key, value)).map_err(|_|
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0008, 0x0003))?;
        Ok(())
    }

    fn delete_internal(&mut self, key: &[u8]) -> Result<(), FailureContext> {
        let mut i = 0;
        while i < self.entries.len() {
            if self.entries[i].0.as_slice() == key { self.entries.remove(i); }
            else { i += 1; }
        }
        Ok(())
    }

    fn assert_canonical_order(&self) {
        for i in 1..self.entries.len() {
            debug_assert!(self.entries[i-1].0.as_slice() <= self.entries[i].0.as_slice());
        }
    }
}

impl StateStore for InMemoryState {
    fn get(&self, key: &[u8]) -> AmunResult<Option<Vec<u8, 256>>> {
        for (k, v) in &self.entries {
            if k.as_slice() == key { return Ok(Some(v.clone())); }
        }
        Ok(None)
    }
    fn set(&mut self, key: &[u8], value: &[u8]) -> AmunResult<()> {
        let mut k = Vec::new(); k.extend_from_slice(key).map_err(|_|
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0008, 0x0001))?;
        let mut v = Vec::new(); v.extend_from_slice(value).map_err(|_|
            FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0008, 0x0002))?;
        self.insert_sorted(k, v)
    }
    fn delete(&mut self, key: &[u8]) -> AmunResult<()> { self.delete_internal(key) }
    fn root(&self) -> AmunResult<PublicHash32> {
        self.assert_canonical_order();
        let mut leaves: Vec<PublicHash32, 256> = Vec::new();
        for (k, v) in &self.entries {
            let leaf = state_leaf_hash(k, v);
            leaves.push(leaf).map_err(|_|
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0008, 0x0006))?;
        }
        Ok(amun_merkle::MerkleTree::compute_root(&leaves))
    }
}
