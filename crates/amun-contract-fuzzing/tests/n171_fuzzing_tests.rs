use amun_contract_fuzzing::*;

#[test]
fn n171_fuzz_deploy_10000() {
    let result = fuzz_contract_deploy(10000);
    println!("Deploy fuzz: {} deploys, {} failed, {} evidence mismatches",
        result.successful_deploys, result.failed_deploys, result.evidence_mismatches);
    assert!(result.passed(), "Evidence mismatches: {}", result.evidence_mismatches);
    assert!(result.successful_deploys > 0, "No successful deploys");
}

#[test]
fn n171_fuzz_call_5000() {
    let result = fuzz_contract_call(5000);
    println!("Call fuzz: {} calls, {} failed, {} gas exhaustions, {} evidence mismatches",
        result.successful_calls, result.failed_calls, result.gas_exhaustions, result.evidence_mismatches);
    assert!(result.passed(), "Evidence mismatches: {}", result.evidence_mismatches);
}

#[test]
fn n171_fuzz_gas_limits_5000() {
    let result = fuzz_gas_limits(5000);
    println!("Gas fuzz: {} calls, {} failed, {} gas exhaustions, {} evidence mismatches",
        result.successful_calls, result.failed_calls, result.gas_exhaustions, result.evidence_mismatches);
    assert!(result.passed(), "Evidence mismatches: {}", result.evidence_mismatches);
    assert!(result.gas_exhaustions > 0, "No gas exhaustions detected");
}
