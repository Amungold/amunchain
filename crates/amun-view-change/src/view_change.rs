use amun_chain_position::ChainPosition;
use blake3::Hasher;

/// A view-change message: sent when a validator times out.
#[derive(Debug, Clone)]
pub struct ViewChange {
    pub validator_id: u64,
    pub position: ChainPosition,
    pub current_round: u64,
    pub new_round: u64,
    pub prepared_qc: Option<[u8; 32]>,
    pub signature: [u8; 64],
    pub view_change_hash: [u8; 32],
}

impl ViewChange {
    pub fn new(
        validator_id: u64,
        position: ChainPosition,
        current_round: u64,
        new_round: u64,
        prepared_qc: Option<[u8; 32]>,
        signature: [u8; 64],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_VIEW_CHANGE_V1");
        h.update(&validator_id.to_le_bytes());
        h.update(&position.hash());
        h.update(&current_round.to_le_bytes());
        h.update(&new_round.to_le_bytes());
        if let Some(qc) = &prepared_qc {
            h.update(qc);
        }
        h.update(&signature);
        let mut view_change_hash = [0u8; 32];
        view_change_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self {
            validator_id,
            position,
            current_round,
            new_round,
            prepared_qc,
            signature,
            view_change_hash,
        }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_VIEW_CHANGE_V1");
        h.update(&self.validator_id.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.current_round.to_le_bytes());
        h.update(&self.new_round.to_le_bytes());
        if let Some(qc) = &self.prepared_qc {
            h.update(qc);
        }
        h.update(&self.signature);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.view_change_hash
    }
}
