use blake3::Hasher;

/// A record of a state transition committed to the chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateCommit {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub previous_root: [u8; 32],
    pub new_root: [u8; 32],
    pub tx_count: usize,
    pub timestamp: u64,
}

impl StateCommit {
    pub fn commit_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_COMMIT_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.previous_root);
        hasher.update(&self.new_root);
        hasher.update(&self.tx_count.to_le_bytes());
        hasher.finalize().into()
    }
}

/// An append-only log of all state transitions.
#[derive(Debug, Clone, Default)]
pub struct CommitLog {
    pub commits: Vec<StateCommit>,
}

impl CommitLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new state commit.
    pub fn record(
        &mut self,
        height: u64,
        block_hash: [u8; 32],
        previous_root: [u8; 32],
        new_root: [u8; 32],
        tx_count: usize,
        timestamp: u64,
    ) -> &StateCommit {
        let commit = StateCommit {
            height,
            block_hash,
            previous_root,
            new_root,
            tx_count,
            timestamp,
        };
        self.commits.push(commit);
        self.commits
            .last()
            .expect("commit_log: invariant violated — empty after push")
    }

    /// Get the latest state root.
    pub fn latest_root(&self) -> Option<[u8; 32]> {
        self.commits.last().map(|c| c.new_root)
    }

    /// Number of commits.
    pub fn is_empty(&self) -> bool {
        self.commits.is_empty()
    }
    pub fn len(&self) -> usize {
        self.commits.len()
    }

    /// Get a commit by height.
    pub fn get(&self, height: u64) -> Option<&StateCommit> {
        self.commits.get(height as usize - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n34_record_commit() {
        let mut log = CommitLog::new();
        let commit = log.record(1, [1u8; 32], [0u8; 32], [10u8; 32], 3, 1000);
        assert_eq!(commit.height, 1);
        assert_eq!(log.len(), 1);
        assert_eq!(log.latest_root(), Some([10u8; 32]));
    }

    #[test]
    fn n34_commit_chain_continuity() {
        let mut log = CommitLog::new();
        let root_a = [10u8; 32];
        let root_b = [20u8; 32];
        let root_c = [30u8; 32];
        log.record(1, [1u8; 32], [0u8; 32], root_a, 2, 1000);
        log.record(2, [2u8; 32], root_a, root_b, 1, 2000);
        log.record(3, [3u8; 32], root_b, root_c, 4, 3000);
        assert_eq!(log.len(), 3);
        assert_eq!(log.latest_root(), Some(root_c));
        assert_eq!(log.get(1).unwrap().new_root, root_a);
        assert_eq!(log.get(2).unwrap().previous_root, root_a);
        assert_eq!(log.get(3).unwrap().previous_root, root_b);
    }

    #[test]
    fn n34_commit_hash_deterministic() {
        let mut log = CommitLog::new();
        let c = log.record(1, [1u8; 32], [0u8; 32], [10u8; 32], 3, 1000);
        assert_eq!(c.commit_hash(), c.commit_hash());
    }

    #[test]
    fn n34_different_commits_different_hash() {
        let mut log = CommitLog::new();
        let c1 = log
            .record(1, [1u8; 32], [0u8; 32], [10u8; 32], 1, 1000)
            .clone();
        let c2 = log.record(2, [2u8; 32], [10u8; 32], [20u8; 32], 2, 2000);
        assert_ne!(c1.commit_hash(), c2.commit_hash());
    }
}
