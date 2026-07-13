use crate::engine::ConsensusEngine;

impl ConsensusEngine {
    /// Register a validator with its derived id and public key.
    pub fn register_validator(&mut self, validator_id: [u8; 32], public_key: [u8; 32]) {
        self.validator_ids.push(validator_id);
        self.validator_keys.insert(validator_id, public_key);
    }

    /// Register a validator with PeerId + ValidatorId + Voting Power.
    pub fn register_validator_identity(
        &mut self,
        peer_id: [u8; 32],
        validator_id: [u8; 32],
        public_key: [u8; 32],
        voting_power: u64,
    ) {
        if self.validator_powers.contains_key(&validator_id) {
            return;
        }

        self.validator_ids.push(validator_id);
        self.validator_keys
            .register_identity(peer_id, validator_id, public_key);

        self.validator_powers.insert(validator_id, voting_power);
        self.total_voting_power += voting_power;
    }
}
