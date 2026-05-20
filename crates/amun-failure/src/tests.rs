#[cfg(test)]
use crate::kernel_state::*;
use crate::taxonomy::*;

#[test]
fn test_all_fault_severities() {
    // Critical faults must halt
    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
    assert!(ConstitutionalFault::UnsafeContractViolation.should_halt());
    assert!(ConstitutionalFault::ConstitutionalViolation.should_halt());
    assert!(ConstitutionalFault::InvalidQuorum.should_halt());
    assert!(ConstitutionalFault::SignatureInvalid.should_halt());
    assert!(ConstitutionalFault::MerkleProofInvalid.should_halt());
    assert!(ConstitutionalFault::DurabilityViolation.should_halt());
    assert!(ConstitutionalFault::JournalHashMismatch.should_halt());
    assert!(ConstitutionalFault::ArithmeticOverflow.should_halt());
    assert!(ConstitutionalFault::ArithmeticUnderflow.should_halt());
    assert!(ConstitutionalFault::DecodeBudgetExceeded.should_halt());
    assert!(ConstitutionalFault::CryptoBudgetExceeded.should_halt());
}

#[test]
fn test_degraded_faults_no_halt() {
    assert!(!ConstitutionalFault::BufferTooSmall.should_halt());
    assert!(!ConstitutionalFault::CapacityExceeded.should_halt());
    assert!(!ConstitutionalFault::TableFull.should_halt());
    assert!(!ConstitutionalFault::MemoryBudgetExhausted.should_halt());
}

#[test]
fn test_rejected_faults_no_halt() {
    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
    assert!(!ConstitutionalFault::UninitializedAccess.should_halt());
    assert!(!ConstitutionalFault::DoubleInitialization.should_halt());
    assert!(!ConstitutionalFault::TemporalViolation.should_halt());
    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
    assert!(!ConstitutionalFault::SequenceMismatch.should_halt());
}

#[test]
fn test_failure_context_creation() {
    let ctx = FailureContext::new(
        ConstitutionalFault::CapacityExceeded,
        module_ids::AMUN_FAILURE,
        operation_ids::KERNEL_CHECK_HEALTH,
    );
    assert_eq!(ctx.fault, ConstitutionalFault::CapacityExceeded);
    assert_eq!(ctx.module_id, module_ids::AMUN_FAILURE);
    assert_eq!(ctx.operation_id, operation_ids::KERNEL_CHECK_HEALTH);
}

#[test]
fn test_failure_context_severity() {
    let ctx = FailureContext::new(
        ConstitutionalFault::EquivocationDetected,
        module_ids::AMUN_FAILURE,
        0,
    );
    assert!(ctx.should_halt());
}

#[test]
fn test_kernel_health_healthy() {
    let health = KernelHealth::healthy();
    assert!(health.can_participate());
}

#[test]
fn test_kernel_health_poisoned() {
    let health = KernelHealth::healthy().poison(ConstitutionalFault::UnsafeContractViolation, 1, 5);
    assert!(!health.can_participate());
}

#[test]
fn test_kernel_health_idempotent_poison() {
    let health = KernelHealth::healthy()
        .poison(ConstitutionalFault::EquivocationDetected, 1, 3)
        .poison(ConstitutionalFault::InvalidQuorum, 2, 7);
    if let KernelHealth::Poisoned {
        fault, at_epoch, ..
    } = health
    {
        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
        assert_eq!(at_epoch, 1);
    } else {
        panic!("Expected poisoned");
    }
}

#[test]
fn test_quarantine_actions() {
    let health = KernelHealth::healthy().poison(ConstitutionalFault::UnsafeContractViolation, 1, 0);
    let actions = health.quarantine_actions();
    assert!(actions.halt_consensus);
    assert!(actions.seal_journal);
    assert!(actions.invalidate_snapshots);
    assert!(actions.preserve_evidence);
}

#[test]
fn test_kernel_state_check_healthy() {
    let state = KernelState::new(1, 1, 0);
    assert!(state.check_healthy().is_ok());
}

#[test]
fn test_kernel_state_record_fault() {
    let state = KernelState::new(1, 1, 0);
    let ctx = FailureContext::new(
        ConstitutionalFault::UnsafeContractViolation,
        module_ids::AMUN_UNSAFE,
        0,
    );
    let state = state.record_fault(&ctx);
    assert!(state.check_healthy().is_err());
}

#[test]
fn test_fault_severity_ordering() {
    assert!(FaultSeverity::Critical > FaultSeverity::Rejected);
    assert!(FaultSeverity::Rejected > FaultSeverity::Degraded);
}
