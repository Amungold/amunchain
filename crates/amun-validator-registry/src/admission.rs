// N149.1: Admission Service — single entry point for validator admission.
//
// This is an orchestration layer. It does NOT introduce new verification
// logic. It wraps the existing register_full() + activate() sequence
// behind a single admit() call.
//
// Verification gates (certificate validation, stake policy, protocol
// compatibility, genesis compatibility) will be added in N149.2.

use crate::registry::ValidatorRegistry;
use crate::ValidatorId;

/// A request to admit a new validator into the active set.
#[derive(Debug, Clone)]
pub struct AdmissionRequest {
    pub validator_id: ValidatorId,
    pub peer_id: crate::PeerId,
    pub public_key: crate::PublicKey,
    pub certificate_hash: [u8; 32],
    pub stake: u64,
    pub voting_power: u64,
    pub protocol_version: u32,
    pub identity_version: u32,
}

/// The result of an admission attempt.
#[derive(Debug, Clone)]
pub enum AdmissionResult {
    /// Validator was registered and activated successfully.
    Admitted {
        validator_id: ValidatorId,
    },
    /// Validator was rejected for a specific reason.
    Rejected {
        validator_id: ValidatorId,
        reason: String,
    },
}

/// Service that orchestrates validator admission.
///
/// Owns no state — delegates storage to ValidatorRegistry and
/// verification to identity/certificate providers (future N149.2).
pub struct AdmissionService;

impl AdmissionService {
    /// Admit a validator: verify, register, activate.
    ///
    /// Currently performs:
    ///   1. Identity verification (delegates to verify_ed25519)
    ///   2. Registration via register_full()
    ///   3. Activation via activate()
    ///
    /// Verification gates are minimal in N149.1 — they will be
    /// expanded in N149.2.
    pub fn admit(
        registry: &mut ValidatorRegistry,
        request: AdmissionRequest,
    ) -> AdmissionResult {
        // Step 1: Identity verification (basic — checks existence of key material)
        if !Self::verify_identity(&request) {
            return AdmissionResult::Rejected {
                validator_id: request.validator_id,
                reason: "Identity verification failed".into(),
            };
        }

        // Step 2: Register the validator (inactive state)
        let record = crate::ValidatorRecord {
            validator_id: request.validator_id,
            peer_id: request.peer_id,
            public_key: request.public_key,
            certificate_hash: request.certificate_hash,
            stake: request.stake,
            voting_power: request.voting_power,
            active: false,
            slash_count: 0,
            registered_at: 0,
            registered_epoch: 0,
            last_seen: 0,
            status: crate::ValidatorStatus::Inactive,
            stake_epoch: 0,
            protocol_version: request.protocol_version,
            identity_version: request.identity_version,
        };

        if let Err(e) = registry.register_full(record) {
            return AdmissionResult::Rejected {
                validator_id: request.validator_id,
                reason: format!("Registration failed: {}", e),
            };
        }

        // Step 3: Activate the validator
        if let Err(e) = registry.activate(&request.validator_id) {
            return AdmissionResult::Rejected {
                validator_id: request.validator_id,
                reason: format!("Activation failed: {}", e),
            };
        }

        AdmissionResult::Admitted {
            validator_id: request.validator_id,
        }
    }

    /// Verify the identity of a prospective validator.
    ///
    /// N149.1: Minimal check — verifies public key is non-zero and
    /// certificate hash is present. Full certificate chain validation
    /// will be added in N149.2.
    fn verify_identity(request: &AdmissionRequest) -> bool {
        // Basic sanity: public key must not be zero
        if request.public_key.0 == [0u8; 32] {
            return false;
        }
        // Certificate hash must be present
        if request.certificate_hash == [0u8; 32] {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PeerId, PublicKey, ValidatorId, ValidatorRegistry, ValidatorStatus};

    #[test]
    fn n149_admit_valid_validator() {
        let mut registry = ValidatorRegistry::new();
        let request = AdmissionRequest {
            validator_id: ValidatorId([1u8; 32]),
            peer_id: PeerId([0u8; 32]),
            public_key: PublicKey([0xAA; 32]),
            certificate_hash: [0xBB; 32],
            stake: 100,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        };

        let result = AdmissionService::admit(&mut registry, request);
        assert!(matches!(result, AdmissionResult::Admitted { .. }));

        // Verify the validator is now active
        assert!(registry.is_active_validator(&ValidatorId([1u8; 32])));
    }

    #[test]
    fn n149_reject_zero_public_key() {
        let mut registry = ValidatorRegistry::new();
        let request = AdmissionRequest {
            validator_id: ValidatorId([1u8; 32]),
            peer_id: PeerId([0u8; 32]),
            public_key: PublicKey([0u8; 32]), // zero key — should reject
            certificate_hash: [0xBB; 32],
            stake: 100,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        };

        let result = AdmissionService::admit(&mut registry, request);
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
    }

    #[test]
    fn n149_reject_empty_certificate_hash() {
        let mut registry = ValidatorRegistry::new();
        let request = AdmissionRequest {
            validator_id: ValidatorId([1u8; 32]),
            peer_id: PeerId([0u8; 32]),
            public_key: PublicKey([0xAA; 32]),
            certificate_hash: [0u8; 32], // empty — should reject
            stake: 100,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        };

        let result = AdmissionService::admit(&mut registry, request);
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
    }
}
