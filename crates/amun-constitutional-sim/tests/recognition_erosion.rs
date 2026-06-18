use amun_constitutional_sim::{ExperimentalProtocol, SimulationRunner};

#[test]
fn test_saturation_at_n100() {
    let protocol = ExperimentalProtocol {
        random_seed: 42,
        num_sovereigns: 100,
        num_steps: 500,
        initial_recognition_density: 0.8,
        initial_treaty_density: 0.0,
        erosion_rate: 0.01,
        formation_base_rate: 0.10,
        formation_legitimacy_factor: 0.20,
        formation_reciprocity_bias: 0.40,
        treaty_failure_rate: 0.0,
        horizon: 500,
    };

    let results = SimulationRunner::run(&protocol);
    let first = results.first().unwrap();
    let last = results.last().unwrap();

    println!("\n=== Saturation Test (N=100, ρ=10) ===");
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

    // Robust assertions: test structural properties, not specific values
    assert!(
        last.recognition_connectivity > 0.5,
        "Network should maintain connectivity"
    );
    assert!(
        last.mean_legitimacy > first.mean_legitimacy * 0.5,
        "Legitimacy should not collapse below 50% of initial"
    );
}

#[test]
fn test_saturation_ratio_scan_n100() {
    let erosion = 0.01;
    let formation_rates = vec![0.01, 0.02, 0.05, 0.10];
    let runs = 5;

    println!("\n=== Saturation Ratio Scan (N=100) ===");
    println!("ρ\tĒ_final\tC_R_final\tD_L_final");

    for &formation in &formation_rates {
        let mut e_sum = 0.0;
        let mut cr_sum = 0.0;
        let mut dl_sum = 0.0;

        for _ in 0..runs {
            let protocol = ExperimentalProtocol {
                random_seed: 42,
        num_sovereigns: 100,
                num_steps: 500,
                initial_recognition_density: 0.8,
                initial_treaty_density: 0.0,
                erosion_rate: erosion,
                formation_base_rate: formation,
                formation_legitimacy_factor: formation * 2.0,
                formation_reciprocity_bias: formation * 4.0,
                treaty_failure_rate: 0.0,
                horizon: 500,
            };
            let results = SimulationRunner::run(&protocol);
            if let Some(last) = results.last() {
                e_sum += last.mean_legitimacy;
                cr_sum += last.recognition_connectivity;
                dl_sum += last.divergence_from_start;
            }
        }

        let e_bar = e_sum / runs as f64;
        let cr = cr_sum / runs as f64;
        let dl = dl_sum / runs as f64;
        let rho = formation / erosion;

        println!("{:.1}\t{:.4}\t{:.4}\t\t{:.4}", rho, e_bar, cr, dl);
    }
}
