use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::{LiveValidator, RuntimeSummary};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn r2_5_1_continuous_traffic_soak_10min() {
    // 4 validators, quorum=3, run for 10 minutes
    let ports = [9500, 9501, 9502, 9503];
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(3)).unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();
    let soak_duration = Duration::from_secs(600); // 10 minutes
    let mut last_height = 0u64;
    let mut stall_count = 0u64;
    let max_stall = 6u64; // Allow up to ~60s of no progress (6 checks * 10s)

    println!(
        "R2.5.1: Soak test started — {}s duration",
        soak_duration.as_secs()
    );

    while start.elapsed() < soak_duration {
        thread::sleep(Duration::from_secs(10));

        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();

        let min_h = *heights.iter().min().unwrap_or(&0);
        let max_h = *heights.iter().max().unwrap_or(&0);
        let elapsed = start.elapsed().as_secs();

        println!(
            "t={:4}s heights={:?} spread={}",
            elapsed,
            heights,
            max_h - min_h
        );

        // Check: height should advance
        if min_h <= last_height {
            stall_count += 1;
            assert!(
                stall_count <= max_stall,
                "R2.5.1 FAIL: Network stalled for {} checks ({}s). Last height: {}",
                stall_count,
                stall_count * 10,
                min_h
            );
        } else {
            stall_count = 0;
            last_height = min_h;
        }

        // Check: spread should be small
        assert!(
            max_h - min_h <= 3,
            "R2.5.1 FAIL: Height spread {} exceeds 3 at t={}s",
            max_h - min_h,
            elapsed
        );
    }

    // Final verification: history_root must match
    let summaries: Vec<RuntimeSummary> = validators.iter().map(|v| v.runtime_summary()).collect();

    let reference = &summaries[0].history_root;
    for (i, s) in summaries.iter().enumerate() {
        assert_eq!(
            &s.history_root, reference,
            "R2.5.1 FAIL: history_root mismatch at validator {}",
            i
        );
    }

    let final_h = summaries[0].height;
    for v in &validators {
        v.stop();
    }

    println!(
        "R2.5.1 PASS: Soak complete — final height={}, all roots match, {}s elapsed",
        final_h,
        start.elapsed().as_secs()
    );
}
