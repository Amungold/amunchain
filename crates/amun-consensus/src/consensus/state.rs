use super::qc_store::{QCStore, QC, QCHash};

#[derive(Debug, Clone, Default)]
pub struct ConsensusState {
    pub current_round: u64,
    pub current_epoch: u64,
    pub locked_qc_hash: Option<QCHash>,
    pub justified_qc_hash: Option<QCHash>,
    pub finalized_qc_hash: Option<QCHash>,
    pub store: QCStore,
}

impl ConsensusState {
    pub fn new() -> Self { Self::default() }
    pub fn get_locked(&self) -> Option<&QC> { self.locked_qc_hash.and_then(|h| self.store.get(&h)) }
    pub fn get_justified(&self) -> Option<&QC> { self.justified_qc_hash.and_then(|h| self.store.get(&h)) }
    pub fn get_finalized(&self) -> Option<&QC> { self.finalized_qc_hash.and_then(|h| self.store.get(&h)) }
    pub fn locked_height(&self) -> Option<u64> { self.get_locked().map(|q| q.block_height) }

    pub fn update_qc(&mut self, qc: QC) -> bool {
        let hash = qc.hash;
        let is_justified = qc.is_justified();
        let is_locked = qc.is_locked();
        let is_finalized = qc.is_finalized();
        let epoch = qc.epoch;
        let round = qc.round;

        self.store.insert(qc);

        if is_justified { self.justified_qc_hash = Some(hash); }
        if is_locked { self.locked_qc_hash = Some(hash); }
        if is_finalized { self.finalized_qc_hash = Some(hash); }
        self.current_round = round;
        self.current_epoch = epoch;
        is_finalized
    }
    pub fn advance_round(&mut self) { self.current_round += 1; }
    pub fn advance_epoch(&mut self) { self.current_epoch += 1; self.current_round = 0; }
}
