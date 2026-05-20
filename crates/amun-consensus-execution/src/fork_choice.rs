use crate::block_dag::{BlockDAG, BlockNode, MAX_DAG_DEPTH};
use amun_quorum_certificate::QuorumCertificate;
use std::collections::{BTreeMap, HashSet};

/// Fork-choice rule with lock history for monotonicity verification
#[derive(Debug, Clone)]
pub struct ForkChoice {
    pub high_qcs: BTreeMap<u64, QuorumCertificate>,
    pub locked_qc: Option<QuorumCertificate>,
    pub validated_qc: Option<QuorumCertificate>,
    /// Committed QCs (from 3-chain finality)
    pub committed_qcs: Vec<QuorumCertificate>,
    /// Lock history for monotonicity verification: (round, block_hash)
    pub lock_history: Vec<(u64, [u8; 32])>,
}

impl ForkChoice {
    pub fn new() -> Self {
        Self {
            high_qcs: BTreeMap::new(),
            locked_qc: None,
            validated_qc: None,
            committed_qcs: Vec::new(),
            lock_history: Vec::new(),
        }
    }

    /// Update with a new QC
    pub fn update_qc(&mut self, qc: QuorumCertificate, dag: &BlockDAG) {
        let round = qc.round;

        self.high_qcs
            .entry(round)
            .and_modify(|existing| {
                if qc.position.sequence > existing.position.sequence {
                    *existing = qc.clone();
                }
            })
            .or_insert_with(|| qc.clone());

        self.try_advance_lock_and_commit(&qc, dag);
        self.update_validated_qc(qc);
    }

    /// Get the highest QC across all rounds
    pub fn highest_qc(&self) -> Option<&QuorumCertificate> {
        self.high_qcs
            .values()
            .max_by_key(|qc| (qc.round, qc.position.sequence))
    }

    /// Try 2-chain lock and 3-chain commit with round monotonicity
    fn try_advance_lock_and_commit(&mut self, qc3: &QuorumCertificate, dag: &BlockDAG) {
        let block3 = match dag.get_block(&qc3.block_hash) {
            Some(b) => b,
            None => return,
        };

        // Get qc2 (parent's justify QC)
        let parent_hash = match block3.parent_hash {
            Some(h) => h,
            None => return,
        };
        let block2 = match dag.get_block(&parent_hash) {
            Some(b) => b,
            None => return,
        };
        let qc2 = match &block2.justify_qc {
            Some(qc) => qc,
            None => return,
        };

        // 2-chain lock: verify round monotonicity
        if qc3.round > qc2.round {
            let should_lock = match &self.locked_qc {
                Some(current) => qc2.round > current.round,
                None => true,
            };
            if should_lock {
                self.locked_qc = Some(qc2.clone());
                self.lock_history.push((qc2.round, qc2.block_hash));
            }
        }

        // 3-chain commit: verify strict round monotonicity
        let grandparent_hash = match block2.parent_hash {
            Some(h) => h,
            None => return,
        };
        let block1 = match dag.get_block(&grandparent_hash) {
            Some(b) => b,
            None => return,
        };
        let qc1 = match &block1.justify_qc {
            Some(qc) => qc,
            None => return,
        };

        if qc3.round > qc2.round && qc2.round > qc1.round {
            // 3-chain commit rule: commit block1
            if !self
                .committed_qcs
                .iter()
                .any(|c| c.block_hash == qc1.block_hash)
            {
                self.committed_qcs.push(qc1.clone());
            }
        }
    }

    fn update_validated_qc(&mut self, qc: QuorumCertificate) {
        match &self.validated_qc {
            Some(current) => {
                if qc.round > current.round {
                    self.validated_qc = Some(qc);
                }
            }
            None => self.validated_qc = Some(qc),
        }
    }

    /// Get canonical chain tip using children_index (not round+1 assumption)
    pub fn canonical_tip(&self, dag: &BlockDAG) -> Option<[u8; 32]> {
        let mut current_hash = self.highest_qc()?.block_hash;
        let mut visited = HashSet::new();
        let mut depth = 0;

        loop {
            if depth > MAX_DAG_DEPTH || !visited.insert(current_hash) {
                return Some(current_hash);
            }

            // Use children_index for direct child traversal
            let children = dag.get_children(&current_hash);

            if children.is_empty() {
                return Some(current_hash);
            }

            // Pick child with highest QC round, tiebreak by block_hash
            let mut best_hash = children[0].block_hash;
            let mut best_round = 0u64;

            for child in &children {
                let child_round = child.justify_qc.as_ref().map(|qc| qc.round).unwrap_or(0);
                if child_round > best_round {
                    best_round = child_round;
                    best_hash = child.block_hash;
                } else if child_round == best_round && child.block_hash > best_hash {
                    best_hash = child.block_hash;
                }
            }

            current_hash = best_hash;
            depth += 1;
        }
    }

    /// Verify lock monotonicity: lock rounds must be strictly increasing
    pub fn verify_lock_monotonicity(&self) -> bool {
        if self.lock_history.len() < 2 {
            return true;
        }
        for i in 1..self.lock_history.len() {
            if self.lock_history[i].0 <= self.lock_history[i - 1].0 {
                return false;
            }
        }
        true
    }

    /// SafeNode predicate: proposal is safe if:
    /// 1. Extends locked QC
    /// 2. Justify QC justifies the parent block
    /// 3. Justify QC round >= locked QC round
    pub fn is_safe_proposal(&self, block: &BlockNode, dag: &BlockDAG) -> bool {
        // Rule 1: Must extend locked QC
        if let Some(ref locked) = self.locked_qc {
            if !block.is_descendant_of(&locked.block_hash, dag) {
                return false;
            }
        }

        // Rule 2: Justify QC must justify the parent
        if let Some(ref justify_qc) = block.justify_qc {
            if let Some(parent_hash) = block.parent_hash {
                if justify_qc.block_hash != parent_hash {
                    return false;
                }
            } else {
                return false;
            }

            // Rule 3: Justify QC round must be >= locked QC round
            if let Some(ref locked) = self.locked_qc {
                if justify_qc.round < locked.round {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for ForkChoice {
    fn default() -> Self {
        Self::new()
    }
}
