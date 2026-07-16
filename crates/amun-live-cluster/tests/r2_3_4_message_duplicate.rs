use amun_live_cluster::fault_injector::FaultInjector;
use std::sync::Arc;

#[test]
fn r2_3_4_duplicate_0_percent_never_duplicates() {
    let fi = FaultInjector::duplicate(0, 2);
    for _ in 0..1000 {
        assert!(
            fi.should_duplicate().is_none(),
            "0% duplicate should never trigger"
        );
    }
}

#[test]
fn r2_3_4_duplicate_100_percent_always_duplicates() {
    let fi = FaultInjector::duplicate(100, 3);
    let mut count = 0;
    for _ in 0..200 {
        if fi.should_duplicate().is_some() {
            count += 1;
        }
    }
    assert_eq!(count, 200, "100% duplicate should always trigger");
}

#[test]
fn r2_3_4_duplicate_returns_correct_count() {
    let fi = FaultInjector::duplicate(100, 5);
    for _ in 0..50 {
        assert_eq!(fi.should_duplicate(), Some(5));
    }
}

#[test]
fn r2_3_4_duplicate_is_deterministic() {
    let fi1 = FaultInjector::duplicate(30, 2);
    let fi2 = FaultInjector::duplicate(30, 2);
    let s1: Vec<Option<u8>> = (0..200).map(|_| fi1.should_duplicate()).collect();
    let s2: Vec<Option<u8>> = (0..200).map(|_| fi2.should_duplicate()).collect();
    assert_eq!(s1, s2);
    let c = s1.iter().filter(|d| d.is_some()).count();
    let pct = (c as f64 / 200.0) * 100.0;
    assert!(
        (20.0..=40.0).contains(&pct),
        "Expected ~30%, got {:.1}%",
        pct
    );
}

#[test]
fn r2_3_4_duplicate_does_not_drop() {
    let fi = FaultInjector::duplicate(100, 2);
    for _ in 0..500 {
        assert!(!fi.should_drop());
    }
}

#[test]
fn r2_3_4_duplicate_does_not_delay() {
    let fi = FaultInjector::duplicate(100, 2);
    for _ in 0..500 {
        assert!(fi.should_delay().is_none());
    }
}

#[test]
fn r2_3_4_duplicate_shared_across_threads() {
    use std::thread;
    let fi = Arc::new(FaultInjector::duplicate(25, 2));
    let fi2 = Arc::clone(&fi);
    let h = thread::spawn(move || {
        let mut c = 0;
        for _ in 0..500 {
            if fi2.should_duplicate().is_some() {
                c += 1;
            }
        }
        c
    });
    let mut cm = 0;
    for _ in 0..500 {
        if fi.should_duplicate().is_some() {
            cm += 1;
        }
    }
    let total = cm + h.join().unwrap();
    let pct = (total as f64 / 1000.0) * 100.0;
    assert!(
        (15.0..=35.0).contains(&pct),
        "Expected ~25%, got {:.1}%",
        pct
    );
}
