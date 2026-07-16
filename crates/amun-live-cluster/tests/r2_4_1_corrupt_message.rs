use amun_live_cluster::fault_injector::{CorruptKind, FaultInjector};
use std::sync::Arc;

#[test]
fn r2_4_1_corrupt_0_percent_never_corrupts() {
    let fi = FaultInjector::corrupt(0, CorruptKind::BitFlip);
    for _ in 0..1000 {
        assert!(fi.should_corrupt().is_none());
    }
}

#[test]
fn r2_4_1_corrupt_100_percent_always_corrupts() {
    let fi = FaultInjector::corrupt(100, CorruptKind::InvalidSignature);
    let mut count = 0;
    for _ in 0..200 {
        if fi.should_corrupt().is_some() {
            count += 1;
        }
    }
    assert_eq!(count, 200);
}

#[test]
fn r2_4_1_corrupt_returns_correct_kind() {
    let fi = FaultInjector::corrupt(100, CorruptKind::WrongHeight);
    for _ in 0..50 {
        assert_eq!(fi.should_corrupt(), Some(CorruptKind::WrongHeight));
    }
}

#[test]
fn r2_4_1_corrupt_is_deterministic() {
    let fi1 = FaultInjector::corrupt(30, CorruptKind::BitFlip);
    let fi2 = FaultInjector::corrupt(30, CorruptKind::BitFlip);
    let s1: Vec<Option<CorruptKind>> = (0..200).map(|_| fi1.should_corrupt()).collect();
    let s2: Vec<Option<CorruptKind>> = (0..200).map(|_| fi2.should_corrupt()).collect();
    assert_eq!(s1, s2);
    let c = s1.iter().filter(|d| d.is_some()).count();
    let pct = (c as f64 / 200.0) * 100.0;
    assert!((20.0..=40.0).contains(&pct));
}

#[test]
fn r2_4_1_corrupt_does_not_drop() {
    let fi = FaultInjector::corrupt(100, CorruptKind::Truncated);
    for _ in 0..500 {
        assert!(!fi.should_drop());
    }
}

#[test]
fn r2_4_1_corrupt_does_not_delay() {
    let fi = FaultInjector::corrupt(100, CorruptKind::WrongBlockHash);
    for _ in 0..500 {
        assert!(fi.should_delay().is_none());
    }
}

#[test]
fn r2_4_1_corrupt_shared_across_threads() {
    use std::thread;
    let fi = Arc::new(FaultInjector::corrupt(25, CorruptKind::InvalidSignature));
    let fi2 = Arc::clone(&fi);
    let h = thread::spawn(move || {
        let mut c = 0;
        for _ in 0..500 {
            if fi2.should_corrupt().is_some() {
                c += 1;
            }
        }
        c
    });
    let mut cm = 0;
    for _ in 0..500 {
        if fi.should_corrupt().is_some() {
            cm += 1;
        }
    }
    let total = cm + h.join().unwrap();
    let pct = (total as f64 / 1000.0) * 100.0;
    assert!((15.0..=35.0).contains(&pct));
}

#[test]
fn r2_4_1_all_corrupt_kinds_constructible() {
    for kind in &[
        CorruptKind::InvalidSignature,
        CorruptKind::BitFlip,
        CorruptKind::WrongHeight,
        CorruptKind::WrongBlockHash,
        CorruptKind::Truncated,
    ] {
        let fi = FaultInjector::corrupt(100, kind.clone());
        assert_eq!(fi.should_corrupt(), Some(kind.clone()));
    }
}
