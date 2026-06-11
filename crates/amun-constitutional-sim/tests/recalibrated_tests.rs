use amun_constitutional_sim::{ExperimentalProtocol, SimulationRunner};

#[test]
fn test_factor_isolation_recalibrated() {
    let erosion = 0.0;
    let steps = 200;
    let n = 50;

    // Group A: High treaty density (1.0) — strongest structural foundation
    let protocol_a = ExperimentalProtocol {
        num_sovereigns: n,
        num_steps: steps,
        initial_recognition_density: 0.8,
        initial_treaty_density: 1.0,
        erosion_rate: erosion,
        formation_base_rate: 0.05,
        formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0,
        horizon: steps,
    };
    let last_a = SimulationRunner::run(&protocol_a)
        .last()
        .unwrap()
        .mean_legitimacy;

    // Group B: Medium treaty density (0.5) — moderate foundation
    let protocol_b = ExperimentalProtocol {
        num_sovereigns: n,
        num_steps: steps,
        initial_recognition_density: 0.8,
        initial_treaty_density: 0.5,
        erosion_rate: erosion,
        formation_base_rate: 0.05,
        formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0,
        horizon: steps,
    };
    let last_b = SimulationRunner::run(&protocol_b)
        .last()
        .unwrap()
        .mean_legitimacy;

    // Group C: Low treaty density (0.3) — weakest foundation
    let protocol_c = ExperimentalProtocol {
        num_sovereigns: n,
        num_steps: steps,
        initial_recognition_density: 0.8,
        initial_treaty_density: 0.3,
        erosion_rate: erosion,
        formation_base_rate: 0.05,
        formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0,
        horizon: steps,
    };
    let last_c = SimulationRunner::run(&protocol_c)
        .last()
        .unwrap()
        .mean_legitimacy;

    println!("Group A (treaty=1.0): Ē={:.4}", last_a);
    println!("Group B (treaty=0.5): Ē={:.4}", last_b);
    println!("Group C (treaty=0.3): Ē={:.4}", last_c);

    // Higher treaty density provides stronger structural foundation.
    // Group A must exceed Group C.
    assert!(
        last_a > last_c,
        "Higher treaty density should produce higher legitimacy: A={:.4} vs C={:.4}",
        last_a,
        last_c
    );

    // Group A should have the highest legitimacy.
    assert!(
        last_a > last_b,
        "Full treaty density should exceed moderate: A={:.4} vs B={:.4}",
        last_a,
        last_b
    );

    // All legitimacy values must be bounded.
    assert!(
        last_a <= 1.0 && last_b <= 1.0 && last_c <= 1.0,
        "All legitimacy values must be ≤ 1.0"
    );
}
