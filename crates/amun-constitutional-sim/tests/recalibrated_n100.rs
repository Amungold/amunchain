use amun_constitutional_sim::{ExperimentalProtocol, SimulationRunner};

#[test]
fn test_n100_validation_recalibrated() {
    let protocol = ExperimentalProtocol {
        num_sovereigns: 100,
        num_steps: 300,
        initial_recognition_density: 0.8,
        initial_treaty_density: 0.0,
        erosion_rate: 0.01,
        formation_base_rate: 0.05,
        formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0,
        horizon: 300,
    };
    let results = SimulationRunner::run(&protocol);
    let first = results.first().unwrap();
    let last = results.last().unwrap();
    println!("\n=== Recalibrated N=100 Validation ===");
    println!(
        "Initial Ē: {:.4}, Final Ē: {:.4}",
        first.mean_legitimacy, last.mean_legitimacy
    );
    println!(
        "Initial C_R: {:.4}, Final C_R: {:.4}",
        first.recognition_connectivity, last.recognition_connectivity
    );
    println!(
        "Final D_L: {:.4}, Collapse Risk: {:.4}",
        last.divergence_from_start, last.collapse_risk
    );
    assert!(
        last.recognition_connectivity > 0.5,
        "Network must stay connected"
    );
    assert!(last.mean_legitimacy <= 1.0, "Ē must be <= 1.0");
}
