use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::{LiveValidator, RuntimeSummary};
use std::thread;
use std::time::{Duration, Instant};

// R2.1b constants
const TARGET_FINALIZED_DELTA: u64 = 10;
const MAX_HEIGHT_SPREAD: u64 = 1;
const MAX_OBSERVATION_SECS: u64 = 30;

/// Delta between two runtime snapshots for a single validator.
#[derive(Debug)]
struct RuntimeDelta {
    validator: usize,
    height_delta: u64,
    votes_delta: u64,
    qcs_delta: u64,
    finalized_delta: u64,
}

/// Collect runtime summaries from all validators.
/// Pure data collection — no computation, no verification.
fn collect_snapshot(validators: &[LiveValidator]) -> Vec<RuntimeSummary> {
    validators.iter().map(|v| v.runtime_summary()).collect()
}

/// Compute deltas between two snapshots for each validator.
/// Pure computation — no verification, no PASS/FAIL.
fn compute_deltas(before: &[RuntimeSummary], after: &[RuntimeSummary]) -> Vec<RuntimeDelta> {
    assert_eq!(
        before.len(),
        after.len(),
        "Snapshot arrays must have same length"
    );
    before
        .iter()
        .zip(after.iter())
        .enumerate()
        .map(|(i, (a, b))| RuntimeDelta {
            validator: i,
            height_delta: b.height.saturating_sub(a.height),
            votes_delta: b.votes_received.saturating_sub(a.votes_received),
            qcs_delta: b.qcs_formed.saturating_sub(a.qcs_formed),
            finalized_delta: b.blocks_finalized.saturating_sub(a.blocks_finalized),
        })
        .collect()
}

/// Verify runtime progress across all validators.
/// Returns true if all criteria are met, false otherwise.
/// Does NOT panic or exit — caller decides the outcome.
fn verify_runtime_progress(snapshot_b: &[RuntimeSummary], deltas: &[RuntimeDelta]) -> bool {
    let mut all_ok = true;

    // Criterion 1: All history roots identical in snapshot B
    let first_root = &snapshot_b[0].history_root;
    for (i, s) in snapshot_b.iter().enumerate() {
        if s.history_root != *first_root {
            eprintln!(
                "R2.1b FAIL: history_root mismatch at validator {}: expected {:?}, got {:?}",
                i, first_root, s.history_root
            );
            all_ok = false;
        }
    }
    if all_ok {
        println!("  history_root: OK (all identical)");
    }

    // Criterion 2: Height spread within tolerance
    let max_h = snapshot_b.iter().map(|s| s.height).max().unwrap_or(0);
    let min_h = snapshot_b.iter().map(|s| s.height).min().unwrap_or(0);
    let spread = max_h.saturating_sub(min_h);
    if spread <= MAX_HEIGHT_SPREAD {
        println!(
            "  height spread: OK (max={}, min={}, spread={})",
            max_h, min_h, spread
        );
    } else {
        eprintln!(
            "R2.1b FAIL: height spread {} exceeds MAX_HEIGHT_SPREAD {}",
            spread, MAX_HEIGHT_SPREAD
        );
        all_ok = false;
    }

    // Criterion 3: Every validator made progress in all dimensions
    for d in deltas {
        let mut ok = true;
        if d.height_delta == 0 {
            eprintln!(
                "R2.1b FAIL: validator {} height did not advance",
                d.validator
            );
            ok = false;
        }
        if d.votes_delta == 0 {
            eprintln!(
                "R2.1b FAIL: validator {} votes did not advance",
                d.validator
            );
            ok = false;
        }
        if d.qcs_delta == 0 {
            eprintln!("R2.1b FAIL: validator {} qcs did not advance", d.validator);
            ok = false;
        }
        if d.finalized_delta == 0 {
            eprintln!(
                "R2.1b FAIL: validator {} finalized did not advance",
                d.validator
            );
            ok = false;
        }
        if ok {
            println!(
                "  validator {}: PASS (+{}h +{}v +{}q +{}f)",
                d.validator, d.height_delta, d.votes_delta, d.qcs_delta, d.finalized_delta
            );
        } else {
            all_ok = false;
        }
    }

    all_ok
}

fn main() {
    let ports = [9200, 9201, 9202, 9203];
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();

    println!("=== Phase 1: All 4 validators running (30s) ===");
    for _ in 0..30 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    println!("=== Phase 2: Crashing validator 3 ===");
    validators[3].stop();

    for _ in 0..15 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators[0..3]
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights=[{}, {}, {}, --]", t, h[0], h[1], h[2]);
    }

    println!("=== Phase 3: Restarting validator 3 ===");
    let new_v3 =
        LiveValidator::new(ValidatorConfig::test_cluster(3, &ports).with_quorum(4)).unwrap();
    new_v3.start().unwrap();
    validators[3] = new_v3;

    println!("=== Phase 4: Waiting for full convergence (timeout=60s) ===");
    let convergence_deadline = Instant::now() + Duration::from_secs(60);
    let mut converged = false;

    while Instant::now() < convergence_deadline {
        thread::sleep(Duration::from_millis(500));
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();

        let min_h = *heights.iter().min().unwrap();
        let max_h = *heights.iter().max().unwrap();
        let spread = max_h - min_h;
        let elapsed = start.elapsed().as_secs();

        println!("t={:3}s heights={:?} spread={}", elapsed, heights, spread);

        if spread == 0 {
            converged = true;
            println!("R2.1 FULLY CONVERGED: spread=0 at t={}s", elapsed);
            break;
        }
    }

    if !converged {
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        eprintln!("R2.1 FAIL: Validators did not fully converge within 60s timeout");
        eprintln!("  Final heights: {:?}", heights);
        std::process::exit(1);
    }

    println!("=== Phase 5: Operational convergence verification ===");
    let snapshot = collect_snapshot(&validators);

    println!("\n=== Runtime Summary After Catch-up ===\n");
    for (i, s) in snapshot.iter().enumerate() {
        println!("validator {}: {}", i, s);
    }

    // R2.1b Observation Window
    let observation_start = std::time::Instant::now();
    let baseline_finalized = snapshot
        .iter()
        .map(|s| s.blocks_finalized)
        .min()
        .unwrap_or(0);

    println!(
        "\n=== Observation Window (target: +{} finalized, max: {}s) ===",
        TARGET_FINALIZED_DELTA, MAX_OBSERVATION_SECS
    );

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let current = collect_snapshot(&validators);
        let current_finalized = current
            .iter()
            .map(|s| s.blocks_finalized)
            .min()
            .unwrap_or(0);

        if current_finalized >= baseline_finalized + TARGET_FINALIZED_DELTA {
            println!(
                "Observation window completed: finalized target reached ({} -> {})",
                baseline_finalized, current_finalized
            );
            break;
        }
        if observation_start.elapsed().as_secs() >= MAX_OBSERVATION_SECS {
            println!(
                "Observation window completed: timeout after {}s",
                observation_start.elapsed().as_secs()
            );
            break;
        }
    }

    let snapshot_b = collect_snapshot(&validators);

    let deltas = compute_deltas(&snapshot, &snapshot_b);
    println!("\n=== Deltas (Snapshot A -> B) ===\n");
    for d in &deltas {
        println!(
            "validator {}: height +{} votes +{} qcs +{} finalized +{}",
            d.validator, d.height_delta, d.votes_delta, d.qcs_delta, d.finalized_delta,
        );
    }

    println!("\n=== R2.1b Verification ===\n");
    if verify_runtime_progress(&snapshot_b, &deltas) {
        println!("\nR2.1b RESULT: PASS");
        println!("  Persistent recovery ........ PASS");
        println!("  Catch-up ................... PASS");
        println!("  Active participation ....... PASS");
        println!("  Network progress ........... PASS");
        println!("  Operational convergence .... PASS");
        println!("\nR2.1 STATUS: COMPLETE");
    } else {
        eprintln!("\nR2.1b RESULT: FAIL");
        eprintln!("  One or more criteria not met. See details above.");
        eprintln!("  -> R2.1a (ConsensusEngine state reconstruction) is justified.");
        std::process::exit(1);
    }
}
