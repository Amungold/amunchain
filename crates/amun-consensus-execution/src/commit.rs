use amun_quorum_certificate::QuorumCertificate;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
pub struct BlockRecord {
    pub block_hash: [u8; 32],
    pub parent_hash: [u8; 32],
    pub height: u64,
    pub round: u64,
}

#[derive(Debug, Clone)]
pub struct CommitRule {
    pub last_qc: Option<QuorumCertificate>,
    pub locked_qc: Option<QuorumCertificate>,
    pub block_records: BTreeMap<[u8; 32], BlockRecord>,
    pub committed_blocks: Vec<(u64, [u8; 32])>,
    pub finalized_height: u64,
    pub commit_index: u64,
}

#[derive(Debug, Clone)]
pub struct CommitCheckpoint {
    pub commit_index: u64,
    pub finalized_height: u64,
    pub last_committed_height: u64,
    pub locked_qc_block: Option<[u8; 32]>,
}

impl CommitRule {
    pub fn new() -> Self {
        Self {
            last_qc: None,
            locked_qc: None,
            block_records: BTreeMap::new(),
            committed_blocks: Vec::new(),
            finalized_height: 0,
            commit_index: 0,
        }
    }

    pub fn register_block(
        &mut self,
        block_hash: [u8; 32],
        parent_hash: [u8; 32],
        height: u64,
        round: u64,
    ) {
        self.block_records.insert(
            block_hash,
            BlockRecord {
                block_hash,
                parent_hash,
                height,
                round,
            },
        );
    }

    pub fn is_descendant(&self, child: &[u8; 32], ancestor: &[u8; 32]) -> bool {
        if child == ancestor {
            return false;
        }
        let mut current = self.block_records.get(child).map(|r| r.parent_hash);
        let mut visited = HashSet::new();
        visited.insert(*child);
        let mut depth = 0;
        const MAX_DEPTH: usize = 50_000;
        while let Some(hash) = current {
            if depth >= MAX_DEPTH || !visited.insert(hash) {
                return false;
            }
            if hash == *ancestor {
                return true;
            }
            current = self.block_records.get(&hash).map(|r| r.parent_hash);
            depth += 1;
        }
        false
    }

    pub fn try_commit_2chain(&mut self, qc: &QuorumCertificate) -> Option<[u8; 32]> {
        if let Some(ref last_qc) = self.last_qc {
            if qc.position.sequence != last_qc.position.sequence + 1 {
                return None;
            }
            let child_record = self.block_records.get(&qc.block_hash)?;
            if child_record.parent_hash != last_qc.block_hash {
                return None;
            }
            if let Some(ref locked) = self.locked_qc {
                if !self.is_descendant(&qc.block_hash, &locked.block_hash) {
                    return None;
                }
            }
            let committed = last_qc.block_hash;
            self.committed_blocks
                .push((last_qc.position.sequence, committed));
            self.commit_index += 1;
            self.locked_qc = Some(last_qc.clone());
            self.last_qc = Some(qc.clone());
            return Some(committed);
        }
        self.last_qc = Some(qc.clone());
        None
    }

    pub fn try_commit_3chain(
        &mut self,
        qc3: &QuorumCertificate,
        qc2: &QuorumCertificate,
        qc1: &QuorumCertificate,
    ) -> Option<[u8; 32]> {
        if !(qc3.round > qc2.round && qc2.round > qc1.round) {
            return None;
        }
        let block3 = self.block_records.get(&qc3.block_hash)?;
        let block2 = self.block_records.get(&qc2.block_hash)?;
        if block3.parent_hash != qc2.block_hash {
            return None;
        }
        if block2.parent_hash != qc1.block_hash {
            return None;
        }
        let committed = qc1.block_hash;
        self.committed_blocks
            .push((qc1.position.sequence, committed));
        self.commit_index += 1;
        Some(committed)
    }

    pub fn is_committed(&self, block_hash: &[u8; 32]) -> bool {
        self.committed_blocks.iter().any(|(_, h)| h == block_hash)
    }

    pub fn last_committed_height(&self) -> u64 {
        self.committed_blocks.last().map(|(h, _)| *h).unwrap_or(0)
    }

    pub fn finalize(&mut self, height: u64) {
        self.finalized_height = height;
        self.block_records
            .retain(|_, r| r.height >= height || r.height == 0);
    }

    pub fn checkpoint_state(&self) -> CommitCheckpoint {
        CommitCheckpoint {
            commit_index: self.commit_index,
            finalized_height: self.finalized_height,
            last_committed_height: self.last_committed_height(),
            locked_qc_block: self.locked_qc.as_ref().map(|qc| qc.block_hash),
        }
    }

    pub fn restore_from_checkpoint(&mut self, checkpoint: &CommitCheckpoint) {
        self.commit_index = checkpoint.commit_index;
        self.finalized_height = checkpoint.finalized_height;
    }
}

impl Default for CommitRule {
    fn default() -> Self {
        Self::new()
    }
}
