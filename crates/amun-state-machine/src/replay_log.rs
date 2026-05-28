use super::transitions::Transition;
use amun_canonical_codec::CanonicalHasher;
use std::collections::{HashMap, HashSet};

/// A node in the constitutional replay DAG.
/// Supports multiple parents (forks/mergers) and branching history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayLogEntry {
    pub sequence: u64,
    pub transition_hash: [u8; 32],
    /// Multiple parents for DAG structure (mergers, causal convergence)
    pub parent_entries: Vec<[u8; 32]>,
    pub epoch: u64,
    pub generation: u64,
    pub entry_hash: [u8; 32],
}

impl ReplayLogEntry {
    pub fn new(sequence: u64, transition: &Transition, parent_entries: Vec<[u8; 32]>) -> Self {
        let mut e = Self {
            sequence,
            transition_hash: transition.transition_id,
            parent_entries,
            epoch: transition.epoch,
            generation: transition.generation,
            entry_hash: [0u8; 32],
        };
        e.entry_hash = e.compute_hash();
        e
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_REPLAY_LOG_V1");
        h.update(&self.sequence.to_le_bytes());
        h.update(&self.transition_hash);
        h.update(&(self.parent_entries.len() as u64).to_le_bytes());
        for p in &self.parent_entries {
            h.update(p);
        }
        h.update(&self.epoch.to_le_bytes());
        h.update(&self.generation.to_le_bytes());
        h.finalize()
    }
}

/// Constitutional Replay DAG - branching constitutional history.
#[derive(Debug, Clone)]
pub struct ConstitutionalReplayDAG {
    pub entries: HashMap<[u8; 32], ReplayLogEntry>,
    pub heads: Vec<[u8; 32]>,
    pub next_sequence: u64,
}

impl Default for ConstitutionalReplayDAG {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalReplayDAG {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            heads: Vec::new(),
            next_sequence: 0,
        }
    }

    pub fn append(&mut self, transition: &Transition, parents: Vec<[u8; 32]>) -> ReplayLogEntry {
        let entry = ReplayLogEntry::new(self.next_sequence, transition, parents.clone());
        // Remove parents from heads, add new entry as head
        for p in &parents {
            self.heads.retain(|h| h != p);
        }
        self.heads.push(entry.entry_hash);
        self.entries.insert(entry.entry_hash, entry.clone());
        self.next_sequence += 1;
        entry
    }

    /// Verify the entire DAG is acyclic and all entries are valid.
    pub fn verify_dag(&self) -> bool {
        // Check all parent references exist
        for entry in self.entries.values() {
            for p in &entry.parent_entries {
                if !self.entries.contains_key(p) && *p != [0u8; 32] {
                    return false;
                }
            }
        }
        // Cycle detection via topological sort
        self.is_acyclic()
    }

    fn is_acyclic(&self) -> bool {
        let mut visited: HashSet<[u8; 32]> = HashSet::new();
        let mut visiting: HashSet<[u8; 32]> = HashSet::new();
        for head in &self.heads {
            if !self.dfs_acyclic(head, &mut visited, &mut visiting) {
                return false;
            }
        }
        true
    }

    fn dfs_acyclic(
        &self,
        node: &[u8; 32],
        visited: &mut HashSet<[u8; 32]>,
        visiting: &mut HashSet<[u8; 32]>,
    ) -> bool {
        if visited.contains(node) {
            return true;
        }
        if visiting.contains(node) {
            return false;
        }
        visiting.insert(*node);
        if let Some(entry) = self.entries.get(node) {
            for p in &entry.parent_entries {
                if *p != [0u8; 32] && !self.dfs_acyclic(p, visited, visiting) {
                    return false;
                }
            }
        }
        visiting.remove(node);
        visited.insert(*node);
        true
    }
}
