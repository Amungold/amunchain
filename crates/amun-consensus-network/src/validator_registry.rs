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
        // N150: Use AdmissionService as single entry point
        if let Some(ref mutex) = self.registry_mut {
            if let Ok(mut reg) = mutex.lock() {
                use amun_validator_registry::{AdmissionRequest, AdmissionService, PeerId, PublicKey, ValidatorId};
                let request = AdmissionRequest {
                    validator_id: ValidatorId(validator_id),
                    peer_id: PeerId(peer_id),
                    public_key: PublicKey(public_key),
                    certificate_hash: [0u8; 32],
                    stake: voting_power,
                    voting_power,
                    protocol_version: 1,
                    identity_version: 1,
                };
                let gates = AdmissionService::mainnet_gates();
                let _ = AdmissionService::admit(&mut reg, request, &gates);
            }
        }

        // Legacy fallback: keep internal maps in sync
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

impl ConsensusEngine {
    /// Get voting power for a validator, preferring the canonical registry.
    pub fn get_validator_voting_power(&self, validator_id: &[u8; 32]) -> u64 {
        if let Some(ref reg) = self.validator_registry {
            let id = amun_validator_registry::ValidatorId(*validator_id);
            return reg.get_voting_power(&id);
        }
        self.validator_powers
            .get(validator_id)
            .copied()
            .unwrap_or(0)
    }

    /// Get total voting power, preferring the canonical registry.
    pub fn get_total_voting_power(&self) -> u64 {
        if let Some(ref reg) = self.validator_registry {
            return reg.total_voting_power();
        }
        self.total_voting_power
    }

    /// Check if a validator is active, preferring the canonical registry.
    pub fn is_validator_active(&self, validator_id: &[u8; 32]) -> bool {
        if let Some(ref reg) = self.validator_registry {
            let id = amun_validator_registry::ValidatorId(*validator_id);
            return reg.is_active(&id);
        }
        self.validator_powers.contains_key(validator_id)
    }

    /// Get public key for a validator, preferring the canonical registry.
    pub fn get_validator_public_key(&self, validator_id: &[u8; 32]) -> Option<[u8; 32]> {
        if let Some(ref reg) = self.validator_registry {
            let id = amun_validator_registry::ValidatorId(*validator_id);
            return reg.get_public_key(&id);
        }
        self.validator_keys.get(validator_id)
    }
}
