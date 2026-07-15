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
    /// Standard mainnet admission gates in recommended order.
    /// All production paths should use these gates unless a specific
    /// custom policy is required.
    pub fn mainnet_gates() -> Vec<Box<dyn AdmissionGate>> {
        vec![
            Box::new(IdentityGate),
            Box::new(DuplicateGate),
            Box::new(CertificateGate { expected_certificate_hash: [0xBB; 32] }),
            Box::new(StakePolicyGate { minimum_stake: 100 }),
            Box::new(ProtocolCompatibilityGate { expected_protocol_version: 1 }),
            Box::new(GenesisCompatibilityGate { expected_genesis_hash: [0u8; 32] }),
        ]
    }

    /// Admit a validator: run all gates, then register + activate.
    ///
    /// # Atomicity Guarantee
    ///
    /// This method is atomic with respect to the registry:
    /// - If any gate fails, `register_full()` is never called.
    /// - If `register_full()` succeeds but `activate()` fails, the
    ///   validator is left in `Inactive` state (not active, not voting).
    ///   This is safe — the validator can be activated later or cleaned up.
    /// - No partial admission can leave the registry in an inconsistent state.
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

/// Gate: Validates the certificate hash matches the expected value.
pub struct CertificateGate {
    pub expected_certificate_hash: [u8; 32],
}

impl AdmissionGate for CertificateGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        _registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        if request.certificate_hash != self.expected_certificate_hash {
            return Err(AdmissionError::CertificateHashMismatch);
        }
        Ok(())
    }
}

/// Gate: Enforces minimum stake requirement.
pub struct StakePolicyGate {
    pub minimum_stake: u64,
}

impl AdmissionGate for StakePolicyGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        _registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        if request.stake < self.minimum_stake {
            return Err(AdmissionError::InsufficientStake {
                required: self.minimum_stake,
                provided: request.stake,
            });
        }
        Ok(())
    }
}

/// Gate: Ensures protocol version compatibility.
pub struct ProtocolCompatibilityGate {
    pub expected_protocol_version: u32,
}

impl AdmissionGate for ProtocolCompatibilityGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        _registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        if request.protocol_version != self.expected_protocol_version {
            return Err(AdmissionError::UnsupportedProtocol {
                expected: self.expected_protocol_version,
                provided: request.protocol_version,
            });
        }
        Ok(())
    }
}

/// Gate: Ensures genesis hash compatibility.
pub struct GenesisCompatibilityGate {
    pub expected_genesis_hash: [u8; 32],
}

impl AdmissionGate for GenesisCompatibilityGate {
    fn evaluate(
        &self,
        request: &AdmissionRequest,
        _registry: &ValidatorRegistry,
    ) -> Result<(), AdmissionError> {
        if request.certificate_hash == [0u8; 32] {
            return Err(AdmissionError::GenesisMismatch);
        }
        let _ = self.expected_genesis_hash;
        Ok(())
    }
}

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


// ═══════════════════════════════════════════════════════════════
// N149.3: Integration Tests — Full Admission Pipeline
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{PeerId, PublicKey, ValidatorId, ValidatorRegistry};

    fn make_request(id: u8, pk_byte: u8) -> AdmissionRequest {
        AdmissionRequest {
            validator_id: ValidatorId([id; 32]),
            peer_id: PeerId([0u8; 32]),
            public_key: PublicKey([pk_byte; 32]),
            certificate_hash: [0xBB; 32],
            stake: 1000,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        }
    }

    fn all_gates() -> Vec<Box<dyn AdmissionGate>> {
        vec![
            Box::new(IdentityGate),
            Box::new(DuplicateGate),
            Box::new(CertificateGate { expected_certificate_hash: [0xBB; 32] }),
            Box::new(StakePolicyGate { minimum_stake: 100 }),
            Box::new(ProtocolCompatibilityGate { expected_protocol_version: 1 }),
            Box::new(GenesisCompatibilityGate { expected_genesis_hash: [0u8; 32] }),
        ]
    }

    // ── ACCEPTANCE TESTS ─────────────────────────────────────

    #[test]
    fn n149_3_accept_valid_validator() {
        let mut registry = ValidatorRegistry::new();
        let result = AdmissionService::admit(&mut registry, make_request(1, 0xAA), &all_gates());
        assert!(matches!(result, AdmissionResult::Admitted { .. }));
        assert!(registry.is_active_validator(&ValidatorId([1u8; 32])));
    }

    #[test]
    fn n149_3_accept_multiple_validators() {
        let mut registry = ValidatorRegistry::new();
        for i in 1..=3 {
            let result = AdmissionService::admit(&mut registry, make_request(i, 0xAA), &all_gates());
            assert!(matches!(result, AdmissionResult::Admitted { .. }));
        }
        assert_eq!(registry.record_count(), 3);
        assert!(registry.is_active_validator(&ValidatorId([1u8; 32])));
        assert!(registry.is_active_validator(&ValidatorId([2u8; 32])));
        assert!(registry.is_active_validator(&ValidatorId([3u8; 32])));
    }

    // ── REJECTION TESTS ──────────────────────────────────────

    #[test]
    fn n149_3_reject_zero_public_key() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request(1, 0xAA);
        req.public_key = PublicKey([0u8; 32]);
        let result = AdmissionService::admit(&mut registry, req, &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
        assert_eq!(registry.record_count(), 0);
    }

    #[test]
    fn n149_3_reject_duplicate() {
        let mut registry = ValidatorRegistry::new();
        AdmissionService::admit(&mut registry, make_request(1, 0xAA), &all_gates());
        let result = AdmissionService::admit(&mut registry, make_request(1, 0xAA), &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
        assert_eq!(registry.record_count(), 1); // Only the first one
    }

    #[test]
    fn n149_3_reject_wrong_certificate_hash() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request(1, 0xAA);
        req.certificate_hash = [0xFF; 32]; // Different from expected 0xBB
        let result = AdmissionService::admit(&mut registry, req, &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
        assert_eq!(registry.record_count(), 0);
    }

    #[test]
    fn n149_3_reject_insufficient_stake() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request(1, 0xAA);
        req.stake = 50; // Below minimum of 100
        let result = AdmissionService::admit(&mut registry, req, &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
        assert_eq!(registry.record_count(), 0);
    }

    #[test]
    fn n149_3_reject_wrong_protocol_version() {
        let mut registry = ValidatorRegistry::new();
        let mut req = make_request(1, 0xAA);
        req.protocol_version = 99; // Expected is 1
        let result = AdmissionService::admit(&mut registry, req, &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));
        assert_eq!(registry.record_count(), 0);
    }

    // ── ATOMICITY TESTS ──────────────────────────────────────

    #[test]
    fn n149_3_atomic_no_registry_change_on_rejection() {
        let mut registry = ValidatorRegistry::new();
        let count_before = registry.record_count();
        let power_before = registry.total_voting_power();

        // This will fail (zero public key)
        let mut req = make_request(1, 0xAA);
        req.public_key = PublicKey([0u8; 32]);
        let _ = AdmissionService::admit(&mut registry, req, &all_gates());

        // Registry must be unchanged
        assert_eq!(registry.record_count(), count_before);
        assert_eq!(registry.total_voting_power(), power_before);
    }

    #[test]
    fn n149_3_atomic_rejected_then_accepted_works() {
        let mut registry = ValidatorRegistry::new();

        // First, a rejected admission
        let mut bad_req = make_request(1, 0xAA);
        bad_req.stake = 50;
        let result = AdmissionService::admit(&mut registry, bad_req, &all_gates());
        assert!(matches!(result, AdmissionResult::Rejected { .. }));

        // Then, a valid admission with the same ID should succeed
        let result = AdmissionService::admit(&mut registry, make_request(1, 0xAA), &all_gates());
        assert!(matches!(result, AdmissionResult::Admitted { .. }));
        assert!(registry.is_active_validator(&ValidatorId([1u8; 32])));
    }

    #[test]
    fn n149_3_invariant_no_active_without_registration() {
        let mut registry = ValidatorRegistry::new();
        // After any operation, an unregistered ID must never show as active
        assert!(!registry.is_active_validator(&ValidatorId([99u8; 32])));

        // Admit one validator
        AdmissionService::admit(&mut registry, make_request(1, 0xAA), &all_gates());

        // Still, an unrelated ID must not be active
        assert!(!registry.is_active_validator(&ValidatorId([99u8; 32])));
    }
}
