#![allow(dead_code, unused_imports, unused_variables)]
use amun_constitutional_sim::{SimulationRunner, ExperimentalProtocol};

#[test]
fn test_interaction_effects() {
    let n = 50;
    let steps = 300;
    let runs = 3;
    
    // Define levels
    let f2_low = 0.05;  let f2_high = 0.20;
    let f3_low = 0.2;   let f3_high = 1.0;
    // f4 is structural; we capture it by comparing with Group A (f4=1) from Phase 115
    
    println!("\n=== Phase 116B: Interaction Effects ===");
    println!("f₂\tf₃\tĒ\tC_R\tD_L");
    
    let conditions = [
        (f2_low, f3_low, "Low/Low"),
        (f2_high, f3_low, "High/Low"),
        (f2_low, f3_high, "Low/High"),
        (f2_high, f3_high, "High/High"),
    ];
    
    for (f2, f3, label) in &conditions {
        let protocol = ExperimentalProtocol {
            num_sovereigns: n, num_steps: steps,
            initial_recognition_density: 0.8, initial_treaty_density: *f3,
            erosion_rate: 0.0,
            formation_base_rate: *f2, formation_legitimacy_factor: *f2 * 2.0,
            formation_reciprocity_bias: *f2 * 4.0,
            treaty_failure_rate: 0.0, horizon: steps,
        };
        
        let mut e_sum = 0.0; let mut cr_sum = 0.0; let mut dl_sum = 0.0;
        for _ in 0..runs {
            let results = SimulationRunner::run(&protocol);
            let last = results.last().unwrap();
            e_sum += last.mean_legitimacy;
            cr_sum += last.recognition_connectivity;
            dl_sum += last.divergence_from_start;
        }
        println!("{}\t{}\t{:.4}\t{:.4}\t{:.4}", 
            if *f2 == f2_high {"High"} else {"Low "},
            if *f3 == f3_high {"High"} else {"Low "},
            e_sum/runs as f64, cr_sum/runs as f64, dl_sum/runs as f64);
    }
}
