use amun_quorum_certificate::QuorumCertificate;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct QCStore {
    pub by_round: BTreeMap<u64, Vec<QuorumCertificate>>,
    pub by_block: BTreeMap<[u8; 32], QuorumCertificate>,
}

impl QCStore {
    pub fn new() -> Self {
        Self {
            by_round: BTreeMap::new(),
            by_block: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, qc: QuorumCertificate) {
        let round = qc.round;
        let block = qc.block_hash;

        self.by_round
            .entry(round)
            .or_insert_with(Vec::new)
            .push(qc.clone());

        self.by_block.insert(block, qc);
    }

    pub fn get_by_block(&self, hash: &[u8; 32]) -> Option<&QuorumCertificate> {
        self.by_block.get(hash)
    }

    pub fn get_by_round(&self, round: u64) -> &[QuorumCertificate] {
        self.by_round
            .get(&round)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn highest_round(&self) -> u64 {
        self.by_round.keys().last().copied().unwrap_or(0)
    }

    pub fn prune_below(&mut self, round: u64) {
        self.by_round = self.by_round.split_off(&round);
        self.by_block.retain(|_, qc| qc.round >= round);
    }
}

impl Default for QCStore {
    fn default() -> Self {
        Self::new()
    }
}
