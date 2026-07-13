use amun_chain_position::ChainPosition;
use blake3::Hasher;

/// An epoch transition: seals the current epoch and begins the next.
#[derive(Debug, Clone)]
pub struct EpochTransition {
    pub previous_epoch: u64,
    pub new_epoch: u64,
    pub position: ChainPosition,
    pub state_root: [u8; 32],
    pub validator_set_hash: [u8; 32],
    pub transition_hash: [u8; 32],
}

impl EpochTransition {
    pub fn new(
        previous_epoch: u64,
        new_epoch: u64,
        position: ChainPosition,
        state_root: [u8; 32],
        validator_set_hash: [u8; 32],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_EPOCH_TRANSITION_V1");
        h.update(&previous_epoch.to_le_bytes());
        h.update(&new_epoch.to_le_bytes());
        h.update(&position.hash());
        h.update(&state_root);
        h.update(&validator_set_hash);
        let mut transition_hash = [0u8; 32];
        transition_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self {
            previous_epoch,
            new_epoch,
            position,
            state_root,
            validator_set_hash,
            transition_hash,
        }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_EPOCH_TRANSITION_V1");
        h.update(&self.previous_epoch.to_le_bytes());
        h.update(&self.new_epoch.to_le_bytes());
        h.update(&self.position.hash());
        h.update(&self.state_root);
        h.update(&self.validator_set_hash);
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.transition_hash
    }
}
