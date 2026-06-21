use amun_contract_security::*;

#[test]
fn n170_audit_reentrancy_pass() {
    let result = audit_reentrancy();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_audit_gas_exhaustion_pass() {
    let result = audit_gas_exhaustion();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_audit_state_isolation_pass() {
    let result = audit_state_isolation();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_audit_determinism_pass() {
    let result = audit_determinism();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_audit_malicious_bytecode_pass() {
    let result = audit_malicious_bytecode();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_audit_evidence_consistency_pass() {
    let result = audit_evidence_consistency();
    assert!(result.passed, "{}: {}", result.test_name, result.details);
}

#[test]
fn n170_full_security_suite() {
    let results = vec![
        audit_reentrancy(),
        audit_gas_exhaustion(),
        audit_state_isolation(),
        audit_determinism(),
        audit_malicious_bytecode(),
        audit_evidence_consistency(),
    ];

    for result in &results {
        assert!(result.passed, "{}: {}", result.test_name, result.details);
    }
}
