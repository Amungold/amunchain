use amun_constitutional_sim::{EffectivenessEngine, SimulationState};

#[test]
fn test_effectiveness_bounds() {
    let mut state = SimulationState::new(20, 0.8, 1.0);
    EffectivenessEngine::update_all(&mut state);

    let mut max_e: f64 = 0.0;
    let mut min_e: f64 = 1.0;
    let mut sum: f64 = 0.0;

    for (claim, _eff) in state.claims.iter().zip(state.effectiveness.iter()) {
        let (_f1, _f2, _f3, _f4, _f5, e) = EffectivenessEngine::debug_claim(claim, &state);

        if e > 1.0 {
            println!("WARNING: E={:.4} > 1.0", e);
        }

        if e > max_e {
            max_e = e;
        }
        if e < min_e {
            min_e = e;
        }
        sum += e;
    }

    let avg = sum / state.effectiveness.len() as f64;
    println!("\n=== Effectiveness Bounds Check ===");
    println!(
        "Min E: {:.4}, Max E: {:.4}, Avg Ē: {:.4}",
        min_e, max_e, avg
    );
    println!("Claims: {}", state.effectiveness.len());

    assert!(
        max_e <= 1.0 + 1e-10,
        "E must be ≤ 1.0, got max={:.4}",
        max_e
    );
    assert!(min_e >= 0.0, "E must be ≥ 0.0");
    assert!(avg <= 1.0, "Ē must be ≤ 1.0, got {:.4}", avg);
}
