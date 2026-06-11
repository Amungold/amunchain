use blake3::Hasher;

/// Aggregates all constitutional evidence into a single verifiable root.
///
/// The EvidenceRoot binds together:
/// - State root (economic state)
/// - Commit hash (state transition record)
/// - Replay certificate (replay verification proof)
/// - Audit record (constitutional audit trail entry)
/// - Previous evidence root (chain continuity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRoot {
    pub root: [u8; 32],
    pub state_root: [u8; 32],
    pub commit_hash: [u8; 32],
    pub replay_certificate: [u8; 32],
    pub audit_record: [u8; 32],
    pub previous_root: [u8; 32],
    pub height: u64,
}

impl EvidenceRoot {
    pub fn compute(
        state_root: [u8; 32],
        commit_hash: [u8; 32],
        replay_certificate: [u8; 32],
        audit_record: [u8; 32],
        previous_root: [u8; 32],
        height: u64,
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_ROOT_V1");
        hasher.update(&state_root);
        hasher.update(&commit_hash);
        hasher.update(&replay_certificate);
        hasher.update(&audit_record);
        hasher.update(&previous_root);
        hasher.update(&height.to_le_bytes());
        let root = hasher.finalize().into();
        Self {
            root,
            state_root,
            commit_hash,
            replay_certificate,
            audit_record,
            previous_root,
            height,
        }
    }

    /// Genesis evidence root with zeroed fields.
    pub fn genesis() -> Self {
        Self::compute([0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 0)
    }

    pub fn verify(&self) -> bool {
        let recomputed = Self::compute(
            self.state_root,
            self.commit_hash,
            self.replay_certificate,
            self.audit_record,
            self.previous_root,
            self.height,
        );
        self.root == recomputed.root
    }
}

/// A chain of evidence roots forming a verifiable history.
#[derive(Debug, Clone, Default)]
pub struct EvidenceChain {
    pub roots: Vec<EvidenceRoot>,
}

impl EvidenceChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(
        &mut self,
        state_root: [u8; 32],
        commit_hash: [u8; 32],
        replay_certificate: [u8; 32],
        audit_record: [u8; 32],
        height: u64,
    ) -> &EvidenceRoot {
        let previous = self.roots.last().map(|r| r.root).unwrap_or([0u8; 32]);
        self.roots.push(EvidenceRoot::compute(
            state_root,
            commit_hash,
            replay_certificate,
            audit_record,
            previous,
            height,
        ));
        self.roots.last().unwrap()
    }

    pub fn verify(&self) -> bool {
        for i in 1..self.roots.len() {
            if self.roots[i].previous_root != self.roots[i - 1].root {
                return false;
            }
            if !self.roots[i].verify() {
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.roots.len()
    }
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n39_genesis_evidence_root() {
        let genesis = EvidenceRoot::genesis();
        assert!(genesis.verify());
        assert_ne!(genesis.root, [0u8; 32]);
    }

    #[test]
    fn n39_evidence_root_deterministic() {
        let r1 = EvidenceRoot::compute([1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [0u8; 32], 1);
        let r2 = EvidenceRoot::compute([1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [0u8; 32], 1);
        assert_eq!(r1.root, r2.root);
    }

    #[test]
    fn n39_different_state_different_root() {
        let r1 = EvidenceRoot::compute([1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [0u8; 32], 1);
        let r2 = EvidenceRoot::compute([9u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [0u8; 32], 1);
        assert_ne!(r1.root, r2.root);
    }

    #[test]
    fn n39_evidence_chain_continuity() {
        let mut chain = EvidenceChain::new();
        chain.append([1u8; 32], [1u8; 32], [1u8; 32], [1u8; 32], 1);
        chain.append([2u8; 32], [2u8; 32], [2u8; 32], [2u8; 32], 2);
        chain.append([3u8; 32], [3u8; 32], [3u8; 32], [3u8; 32], 3);
        assert!(chain.verify());
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn n39_broken_chain_detected() {
        let mut chain = EvidenceChain::new();
        chain.append([1u8; 32], [1u8; 32], [1u8; 32], [1u8; 32], 1);
        chain.append([2u8; 32], [2u8; 32], [2u8; 32], [2u8; 32], 2);
        chain.append([3u8; 32], [3u8; 32], [3u8; 32], [3u8; 32], 3);
        chain.roots[1].state_root = [0xFF; 32];
        assert!(!chain.verify());
    }
}
