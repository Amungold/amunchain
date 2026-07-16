use amun_live_cluster::fault_injector::FaultInjector;
use std::sync::Arc;
use std::time::Instant;

#[test]
fn r2_3_2_delay_0_percent_never_delays() {
    let fi = FaultInjector::delay(0, 50, 500);
    for _ in 0..1000 {
        assert!(fi.should_delay().is_none(), "0% delay should never delay");
    }
}

#[test]
fn r2_3_2_delay_100_percent_always_delays() {
    let fi = FaultInjector::delay(100, 50, 500);
    let mut delay_count = 0;
    for _ in 0..200 {
        if fi.should_delay().is_some() {
            delay_count += 1;
        }
    }
    assert_eq!(delay_count, 200, "100% delay should always delay");
}

#[test]
fn r2_3_2_delay_range_is_respected() {
    let fi = FaultInjector::delay(100, 50, 100);
    for _ in 0..200 {
        if let Some(ms) = fi.should_delay() {
            assert!(ms >= 50 && ms <= 100, "Delay {}ms out of range 50..100", ms);
        }
    }
}

#[test]
fn r2_3_2_delay_is_deterministic() {
    let fi1 = FaultInjector::delay(30, 50, 500);
    let fi2 = FaultInjector::delay(30, 50, 500);

    let seq1: Vec<Option<u64>> = (0..200).map(|_| fi1.should_delay()).collect();
    let seq2: Vec<Option<u64>> = (0..200).map(|_| fi2.should_delay()).collect();

    assert_eq!(
        seq1, seq2,
        "Same delay config must produce identical deterministic sequence"
    );

    let delay_count = seq1.iter().filter(|d| d.is_some()).count();
    let delay_pct = (delay_count as f64 / 200.0) * 100.0;
    assert!(
        (20.0..=40.0).contains(&delay_pct),
        "Expected ~30% delay rate, got {:.1}% ({} / 200)",
        delay_pct,
        delay_count
    );
}

#[test]
fn r2_3_2_delay_50_percent_range() {
    let fi = FaultInjector::delay(50, 50, 500);
    let seq: Vec<Option<u64>> = (0..200).map(|_| fi.should_delay()).collect();
    let delay_count = seq.iter().filter(|d| d.is_some()).count();
    let delay_pct = (delay_count as f64 / 200.0) * 100.0;
    assert!(
        (40.0..=60.0).contains(&delay_pct),
        "Expected ~50% delay rate, got {:.1}% ({} / 200)",
        delay_pct,
        delay_count
    );
}

#[test]
fn r2_3_2_delay_does_not_drop() {
    let fi = FaultInjector::delay(100, 10, 20);
    for _ in 0..500 {
        // should_delay may return Some, but should_drop must never return true
        assert!(!fi.should_drop(), "Delay mode must never drop messages");
    }
}

#[test]
fn r2_3_2_delay_shared_across_threads() {
    use std::thread;

    let fi = Arc::new(FaultInjector::delay(25, 50, 200));
    let fi2 = Arc::clone(&fi);

    let h = thread::spawn(move || {
        let mut delays = 0;
        for _ in 0..500 {
            if fi2.should_delay().is_some() {
                delays += 1;
            }
        }
        delays
    });

    let mut delays_main = 0;
    for _ in 0..500 {
        if fi.should_delay().is_some() {
            delays_main += 1;
        }
    }

    let delays_thread = h.join().unwrap();
    let total = delays_main + delays_thread;
    let pct = (total as f64 / 1000.0) * 100.0;

    assert!(
        (15.0..=35.0).contains(&pct),
        "Expected ~25% delay across threads, got {:.1}% ({} / 1000)",
        pct,
        total
    );
}

#[test]
fn r2_3_2_delay_actual_time_measurement() {
    let fi = FaultInjector::delay(100, 50, 100);
    // Each delay should take 50-100ms
    for _ in 0..10 {
        let start = Instant::now();
        if let Some(ms) = fi.should_delay() {
            std::thread::sleep(std::time::Duration::from_millis(ms));
        }
        let elapsed_ms = start.elapsed().as_millis() as u64;
        // Allow some tolerance for OS scheduling jitter
        assert!(
            elapsed_ms >= 40 && elapsed_ms <= 150,
            "Actual delay {}ms out of expected range (40..150)",
            elapsed_ms
        );
    }
}
