use amun_soak_full::*;

#[test]
fn n165_full_soak_30s() {
    let config = FullSoakConfig {
        duration_secs: 30,
        validators: 2,
        ..Default::default()
    };
    let result = run_full_soak(config);
    println!(
        "30s soak: {} ops, {} failures, {} roots",
        result.total_ops, result.total_failures, result.state_roots_collected
    );
    assert!(
        result.passed,
        "Soak test failed: {} failures, roots_consistent: {}",
        result.total_failures, result.state_roots_consistent
    );
}

#[test]
fn n165_full_soak_60s_with_adversarial() {
    let config = FullSoakConfig {
        duration_secs: 60,
        validators: 2,
        adversarial_events: true,
        ..Default::default()
    };
    let result = run_full_soak(config);
    println!(
        "60s soak: {} ops, {} failures, {} roots",
        result.total_ops, result.total_failures, result.state_roots_collected
    );
    assert!(
        result.passed,
        "Soak test with adversarial events failed: {} failures",
        result.total_failures
    );
}

#[test]
fn n165_state_consistency_under_full_load() {
    let config = FullSoakConfig {
        duration_secs: 15,
        validators: 1,
        adversarial_events: false,
        ..Default::default()
    };
    let result = run_full_soak(config);
    assert!(result.state_roots_consistent, "State roots inconsistent");
    assert!(result.state_roots_collected > 0, "No state roots collected");
}
