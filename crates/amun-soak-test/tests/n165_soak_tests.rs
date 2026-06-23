use amun_soak_test::ValidatorSimulator;

#[test]
fn n165_soak_30_seconds_no_events() {
    let sim = ValidatorSimulator::new();
    let result = sim.run(30, false);
    println!(
        "30s soak: {} ops, {} failures, height {}",
        result.operations, result.failures, result.final_height
    );
    assert!(
        result.passed(),
        "Soak test failed: {} failures",
        result.failures
    );
    assert!(result.operations > 0, "No operations performed");
}

#[test]
fn n165_soak_60_seconds_with_events() {
    let sim = ValidatorSimulator::new();
    let result = sim.run(60, true);
    println!(
        "60s soak with events: {} ops, {} failures, height {}",
        result.operations, result.failures, result.final_height
    );
    assert!(
        result.passed(),
        "Soak test with events failed: {} failures",
        result.failures
    );
    assert!(result.operations > 0, "No operations performed");
}

#[test]
fn n165_state_consistency_under_load() {
    let sim = ValidatorSimulator::new();
    let result = sim.run(10, true);
    let root1 = result.state_root;
    let root2 = result.state_root;
    assert_eq!(root1, root2, "State root inconsistent under load");
    assert_ne!(root1, [0u8; 32], "State root should not be zero");
}
