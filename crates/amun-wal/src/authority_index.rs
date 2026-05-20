use std::collections::BTreeMap;

/// Authority index for O(1) replay lookup
#[derive(Debug, Clone, Default)]
pub struct AuthorityIndex {
    /// sequence -> chain_hash (O(1) reverse lookup)
    pub sequence_to_chain_hash: BTreeMap<u64, String>,
    /// chain_hash -> sequence (O(1) forward lookup)
    pub chain_hash_to_sequence: BTreeMap<String, u64>,
    /// segment -> first_sequence
    pub segment_first_sequence: BTreeMap<u64, u64>,
    /// Highest indexed sequence
    pub max_sequence: u64,
}

impl AuthorityIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build index from WAL entries
    pub fn build_from_entries(entries: &[super::WALEntry]) -> Self {
        let mut index = Self::new();
        for entry in entries {
            index.max_sequence = index.max_sequence.max(entry.sequence);
            index
                .sequence_to_chain_hash
                .insert(entry.sequence, entry.chain_hash.clone());
            index
                .chain_hash_to_sequence
                .insert(entry.chain_hash.clone(), entry.sequence);
        }
        index
    }

    /// O(1) chain hash lookup by sequence
    pub fn chain_hash_for(&self, seq: u64) -> Option<&String> {
        self.sequence_to_chain_hash.get(&seq)
    }

    /// O(1) sequence lookup by chain hash
    pub fn sequence_for_hash(&self, hash: &str) -> Option<u64> {
        self.chain_hash_to_sequence.get(hash).copied()
    }

    /// Verify snapshot anchoring with O(1) lookups
    pub fn verify_snapshot_anchor(
        &self,
        snapshot_sequence: u64,
        snapshot_chain_hash: &str,
    ) -> bool {
        self.sequence_to_chain_hash
            .get(&snapshot_sequence)
            .map(|h| h == snapshot_chain_hash)
            .unwrap_or(false)
            && self
                .chain_hash_to_sequence
                .get(snapshot_chain_hash)
                .map(|&s| s == snapshot_sequence)
                .unwrap_or(false)
    }
}
