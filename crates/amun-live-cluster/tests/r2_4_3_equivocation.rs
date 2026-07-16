use amun_live_cluster::fault_injector::FaultInjector;
use std::sync::Arc;

#[test]
fn r2_4_3_equivocate_0_percent_never_equivocates() {
    let fi = FaultInjector::equivocate(0);
    for _ in 0..1000 {
        assert!(!fi.should_equivocate());
    }
}

#[test]
fn r2_4_3_equivocate_100_percent_always_equivocates() {
    let fi = FaultInjector::equivocate(100);
    let mut count = 0;
    for _ in 0..200 {
        if fi.should_equivocate() { count += 1; }
    }
    assert_eq!(count, 200);
}

#[test]
fn r2_4_3_equivocate_is_deterministic() {
    let fi1 = FaultInjector::equivocate(30);
    let fi2 = FaultInjector::equivocate(30);
    let s1: Vec<bool> = (0..200).map(|_| fi1.should_equivocate()).collect();
    let s2: Vec<bool> = (0..200).map(|_| fi2.should_equivocate()).collect();
    assert_eq!(s1, s2);
    let c = s1.iter().filter(|&&d| d).count();
    let pct = (c as f64 / 200.0) * 100.0;
    assert!((20.0..=40.0).contains(&pct));
}

#[test]
fn r2_4_3_equivocate_does_not_drop() {
    let fi = FaultInjector::equivocate(100);
    for _ in 0..500 { assert!(!fi.should_drop()); }
}

#[test]
fn r2_4_3_equivocate_does_not_delay() {
    let fi = FaultInjector::equivocate(100);
    for _ in 0..500 { assert!(fi.should_delay().is_none()); }
}

#[test]
fn r2_4_3_equivocate_shared_across_threads() {
    use std::thread;
    let fi = Arc::new(FaultInjector::equivocate(25));
    let fi2 = Arc::clone(&fi);
    let h = thread::spawn(move || {
        let mut c = 0;
        for _ in 0..500 { if fi2.should_equivocate() { c += 1; } }
        c
    });
    let mut cm = 0;
    for _ in 0..500 { if fi.should_equivocate() { cm += 1; } }
    let total = cm + h.join().unwrap();
    let pct = (total as f64 / 1000.0) * 100.0;
    assert!((15.0..=35.0).contains(&pct));
}
