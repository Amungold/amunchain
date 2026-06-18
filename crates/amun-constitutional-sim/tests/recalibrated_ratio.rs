use amun_constitutional_sim::{ExperimentalProtocol, SimulationRunner};

#[test]
fn test_ratio_scan_recalibrated() {
    let erosion = 0.01;
    let formation_rates = vec![0.005, 0.01, 0.02, 0.05, 0.10];
    let n = 50;
    let steps = 200;
    let runs = 3;

    println!("\n=== Recalibrated Ratio Scan ===");
    println!("ρ\tĒ_final\tC_R_final\tD_L_final");

    for &formation in &formation_rates {
        let mut e_sum = 0.0;
        let mut cr_sum = 0.0;
        let mut dl_sum = 0.0;

        for _ in 0..runs {
            let protocol = ExperimentalProtocol {
                random_seed: 42,
        num_sovereigns: n,
                num_steps: steps,
                initial_recognition_density: 0.8,
                initial_treaty_density: 0.0,
                erosion_rate: erosion,
                formation_base_rate: formation,
                formation_legitimacy_factor: formation * 2.0,
                formation_reciprocity_bias: formation * 4.0,
                treaty_failure_rate: 0.0,
                horizon: steps,
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
        println!("{:.2}\t{:.4}\t{:.4}\t\t{:.4}", rho, e_bar, cr, dl);
    }
}
