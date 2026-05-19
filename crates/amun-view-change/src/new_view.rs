use amun_chain_position::ChainPosition;
use crate::view_change::ViewChange;
use blake3::Hasher;

/// NewView: the new proposer aggregates view-change messages.
#[derive(Debug, Clone)]
pub struct NewView {
    pub position: ChainPosition,
    pub new_round: u64,
    pub view_changes: Vec<ViewChange>,
    pub new_view_hash: [u8; 32],
}

impl NewView {
    pub fn new(position: ChainPosition, new_round: u64, view_changes: Vec<ViewChange>) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_NEW_VIEW_V1");
        h.update(&position.hash());
        h.update(&new_round.to_le_bytes());
        for vc in &view_changes {
            h.update(&vc.view_change_hash);
        }
        let mut new_view_hash = [0u8; 32];
        new_view_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { position, new_round, view_changes, new_view_hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_NEW_VIEW_V1");
        h.update(&self.position.hash());
        h.update(&self.new_round.to_le_bytes());
        for vc in &self.view_changes {
            h.update(&vc.view_change_hash);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.new_view_hash
    }
}
