// N149: Admission Service — single entry point for validator admission.
//
// N149.1: Orchestration layer (admit() wraps register_full + activate).
// N149.2: Policy framework (AdmissionGate trait, AdmissionPipeline).
//
// Verification gates are composable and independently testable.

use crate::registry::ValidatorRegistry;
use crate::ValidatorId;

// ═══════════════════════════════════════════════════════════════
// N149.2.1: AdmissionError — structured rejection reasons
// ═══════════════════════════════════════════════════════════════

/// Structured error for admission rejection.
/// Replaces String-based reasons for type-safe matching and audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionError {
    InvalidIdentity,
    InvalidCertificate,
    CertificateHashMismatch,
    DuplicateValidator,
    AlreadyRegistered,
    InsufficientStake { required: u64, provided: u64 },
    UnsupportedProtocol { expected: u32, provided: u32 },
    GenesisMismatch,
    AuthorityRejected,
    RegistryError(String),
}

impl std::fmt::Display for AdmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdmissionError::InvalidIdentity => write!(f, "Invalid validator identity"),
            AdmissionError::InvalidCertificate => write!(f, "Invalid certificate"),
            AdmissionError::CertificateHashMismatch => write!(f, "Certificate hash mismatch"),
            AdmissionError::DuplicateValidator => write!(f, "Validator already exists"),
            AdmissionError::AlreadyRegistered => write!(f, "Validator already registered"),
            AdmissionError::InsufficientStake { required, provided } => {
                write!(f, "Insufficient stake: required={} provided={}", required, provided)
            }
            AdmissionError::UnsupportedProtocol { expected, provided } => {
                write!(f, "Unsupported protocol: expected={} provided={}", expected, provided)
            }
            AdmissionError::GenesisMismatch => write!(f, "Genesis hash mismatch"),
            AdmissionError::AuthorityRejected => write!(f, "Rejected by authority"),
            AdmissionError::RegistryError(e) => write!(f, "Registry error: {}", e),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// N149.2.1: AdmissionGate trait — composable policy checks
// ═══════════════════════════════════════════════════════════════

/// A single admission policy check.
/// Each gate evaluates one aspect of a validator admission request.
pub trait AdmissionGate: Send + Sync {
    /// Evaluate this gate. Returns Ok(()) if the request passes,
    /// or Err(AdmissionError) if it fails.
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError>;
}

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
    Admitted { validator_id: ValidatorId },
    /// Validator was rejected for a specific reason.
    Rejected {
        validator_id: ValidatorId,
        error: AdmissionError,
    },
}

/// Service that orchestrates validator admission.
///
/// Owns no state — delegates storage to ValidatorRegistry and
/// verification to the configured AdmissionGates.
pub struct AdmissionService;

impl AdmissionService {
    /// Admit a validator: run all gates, then register + activate.
    pub fn admit(
        registry: &mut ValidatorRegistry,
        request: AdmissionRequest,
        gates: &[Box<dyn AdmissionGate>],
    ) -> AdmissionResult {
        // Step 1: Run all admission gates
        for gate in gates {
            if let Err(error) = gate.evaluate(&request, registry) {
                return AdmissionResult::Rejected {
                    validator_id: request.validator_id,
                    error,
                };
            }
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
                error: AdmissionError::RegistryError(e.to_string()),
            };
        }

        // Step 3: Activate the validator
        if let Err(e) = registry.activate(&request.validator_id) {
            return AdmissionResult::Rejected {
                validator_id: request.validator_id,
                error: AdmissionError::RegistryError(e.to_string()),
            };
        }

        AdmissionResult::Admitted {
            validator_id: request.validator_id,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// N149.2.2: Built-in Admission Gates
// ═══════════════════════════════════════════════════════════════

/// Gate: Rejects validators with zero public key or empty certificate hash.
pub struct IdentityGate;

impl AdmissionGate for IdentityGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        _registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        if request.public_key.0 == [0u8; 32] {
            return Err(AdmissionError::InvalidIdentity);
        }
        if request.certificate_hash == [0u8; 32] {
            return Err(AdmissionError::InvalidIdentity);
        }
        Ok(())
    }
}

/// Gate: Rejects duplicate validator IDs already in the registry.
pub struct DuplicateGate;

impl AdmissionGate for DuplicateGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        let short_id = u64::from_le_bytes(request.validator_id.0[..8].try_into().unwrap_or([0u8; 8]));
        if registry.contains(short_id) {
            return Err(AdmissionError::DuplicateValidator);
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::{PeerId, PublicKey, ValidatorId, ValidatorRegistry, ValidatorStatus};

    fn make_request() -> AdmissionRequest {
        AdmissionRequest {
            validator_id: ValidatorId([1u8; 32]),
            peer_id: PeerId([0u8; 32]),
            public_key: PublicKey([0xAA; 32]),
            certificate_hash: [0xBB; 32],
            stake: 100,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        }
    }

    fn default_gates() -> Vec<Box<dyn AdmissionGate>> {
        vec![Box::new(IdentityGate), Box::new(DuplicateGate)]
    }

    #[test]
    fn n149_admit_valid_validator() {
        let mut registry = ValidatorRegistry::new();
        let result = AdmissionService::admit(&mut registry, make_request(), &default_gates());
        assert!(matches!(result, AdmissionResult::Admitted { .. }));
        assert!(registry.is_active_validator(&ValidatorId([1u8; 32])));
    }

    #[test]
    fn n149_reject_zero_public_key() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request();
        req.public_key = PublicKey([0u8; 32]);
        let result = AdmissionService::admit(&mut registry, req, &default_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
    }

    #[test]
    fn n149_reject_empty_certificate_hash() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request();
        req.certificate_hash = [0u8; 32];
        let result = AdmissionService::admit(&mut registry, req, &default_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
    }

    #[test]
    fn n149_reject_duplicate_validator() {
        let mut registry = ValidatorRegistry::new();
        // First admission succeeds
        let result = AdmissionService::admit(&mut registry, make_request(), &default_gates());
        assert!(matches!(result, AdmissionResult::Admitted { .. }));
        // Second admission of same validator fails
        let result = AdmissionService::admit(&mut registry, make_request(), &default_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
    }

    #[test]
    fn n149_error_display() {
        let err = AdmissionError::InsufficientStake { required: 1000, provided: 500 };
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("1000"));
    }
}
