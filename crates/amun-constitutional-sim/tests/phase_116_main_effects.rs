use amun_constitutional_sim::{SimulationRunner, ExperimentalProtocol};

#[test]
fn test_main_effects() {
    let n = 50;
    let steps = 300;
    let runs = 5;
    
    // Baseline: all factors at minimum
    let base = ExperimentalProtocol {
        num_sovereigns: n, num_steps: steps,
        initial_recognition_density: 0.8, initial_treaty_density: 0.2,
        erosion_rate: 0.0,
        formation_base_rate: 0.05, formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0, horizon: steps,
    };

    println!("\n=== Phase 116A: Main Effects ===");
    println!("Condition\tĒ\tC_R\tD_L");
    
    // 1. Baseline (all factors constrained)
    let mut e_sum = 0.0; let mut cr_sum = 0.0; let mut dl_sum = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&base);
        let last = results.last().unwrap();
        e_sum += last.mean_legitimacy;
        cr_sum += last.recognition_connectivity;
        dl_sum += last.divergence_from_start;
    }
    println!("Baseline\t{:.4}\t{:.4}\t{:.4}", e_sum/runs as f64, cr_sum/runs as f64, dl_sum/runs as f64);
    
    // 2. Maximize f2 (Recognition) only
    let mut p_f2 = base.clone();
    p_f2.formation_base_rate = 0.20;
    p_f2.formation_legitimacy_factor = 0.40;
    p_f2.formation_reciprocity_bias = 0.80;
    let mut e_sum = 0.0; let mut cr_sum = 0.0; let mut dl_sum = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&p_f2);
        let last = results.last().unwrap();
        e_sum += last.mean_legitimacy;
        cr_sum += last.recognition_connectivity;
        dl_sum += last.divergence_from_start;
    }
    println!("Max f₂\t{:.4}\t{:.4}\t{:.4}", e_sum/runs as f64, cr_sum/runs as f64, dl_sum/runs as f64);
    
    // 3. Maximize f3 (Treaties) only
    let mut p_f3 = base.clone();
    p_f3.initial_treaty_density = 1.0;
    let mut e_sum = 0.0; let mut cr_sum = 0.0; let mut dl_sum = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&p_f3);
        let last = results.last().unwrap();
        e_sum += last.mean_legitimacy;
        cr_sum += last.recognition_connectivity;
        dl_sum += last.divergence_from_start;
    }
    println!("Max f₃\t{:.4}\t{:.4}\t{:.4}", e_sum/runs as f64, cr_sum/runs as f64, dl_sum/runs as f64);
    
    // 4. Maximize f4 (Jurisdiction) indirectly — f4 is structural, measured by overlap
    // We can't directly set f4=1, but we can observe its effect by comparing groups
}
