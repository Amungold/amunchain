use amun_constitutional_sim::{SimulationRunner, ExperimentalProtocol};

#[test]
fn test_engineering_map() {
    let n = 50;
    let steps = 300;
    let runs = 5;

    // Reference baseline
    let base = ExperimentalProtocol {
        num_sovereigns: n, num_steps: steps,
        initial_recognition_density: 0.8, initial_treaty_density: 0.3,
        erosion_rate: 0.0,
        formation_base_rate: 0.05, formation_legitimacy_factor: 0.10,
        formation_reciprocity_bias: 0.20,
        treaty_failure_rate: 0.0, horizon: steps,
    };

    let mut e_base = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&base);
        e_base += results.last().unwrap().mean_legitimacy;
    }
    e_base /= runs as f64;

    println!("\n=== Phase 116C: Constitutional Engineering Map ===");
    println!("Baseline Ē: {:.4}", e_base);
    println!("Intervention\tΔĒ\tEfficiency");
    
    // Intervention 1: Increase Recognition Formation
    let mut p = base.clone();
    p.formation_base_rate = 0.10;
    p.formation_legitimacy_factor = 0.20;
    p.formation_reciprocity_bias = 0.40;
    let mut e_int = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&p);
        e_int += results.last().unwrap().mean_legitimacy;
    }
    e_int /= runs as f64;
    println!("Recognition↑\t{:.4}\t{:.4}", e_int - e_base, (e_int - e_base) / 0.05);
    
    // Intervention 2: Increase Treaty Density
    let mut p = base.clone();
    p.initial_treaty_density = 0.8;
    let mut e_int = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&p);
        e_int += results.last().unwrap().mean_legitimacy;
    }
    e_int /= runs as f64;
    println!("Treaties↑\t{:.4}\t{:.4}", e_int - e_base, (e_int - e_base) / 0.5);
    
    // Intervention 3: Combined (Recognition + Treaties)
    let mut p = base.clone();
    p.formation_base_rate = 0.10;
    p.formation_legitimacy_factor = 0.20;
    p.formation_reciprocity_bias = 0.40;
    p.initial_treaty_density = 0.8;
    let mut e_int = 0.0;
    for _ in 0..runs {
        let results = SimulationRunner::run(&p);
        e_int += results.last().unwrap().mean_legitimacy;
    }
    e_int /= runs as f64;
    println!("Combined↑\t{:.4}\t{:.4}", e_int - e_base, (e_int - e_base) / 0.55);
}
