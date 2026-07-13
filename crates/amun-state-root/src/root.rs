use amun_kernel::canonical::{CanonicalEncode, CanonicalEncoder};
use amun_kernel::hashing::domain_tags;
use std::collections::BTreeMap;

/// A single leaf in the state trie.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateLeaf {
    pub key: String,
    pub value: Vec<u8>,
}

impl CanonicalEncode for StateLeaf {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.key.encode_canonical(out);
        self.value.encode_canonical(out);
    }
}

/// Engine for computing the global state root.
pub struct StateRootEngine;

impl StateRootEngine {
    /// Compute a domain root from leaves.
    /// Duplicate keys are **rejected** with an error.
    pub fn domain_root(leaves: &[StateLeaf]) -> Result<[u8; 32], String> {
        let mut map: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for leaf in leaves {
            if map.insert(leaf.key.clone(), leaf.value.clone()).is_some() {
                return Err(format!("Duplicate canonical key: {}", leaf.key));
            }
        }
        Ok(CanonicalEncoder::hash_sorted(
            map.iter(),
            domain_tags::STATE_ROOT,
        ))
    }

    /// Global state root from per‑domain roots.
    pub fn global_root(domain_roots: &[[u8; 32]]) -> [u8; 32] {
        CanonicalEncoder::hash_sorted(domain_roots.iter(), domain_tags::STATE_ROOT)
    }
}
