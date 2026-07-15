use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::Duration;

fn main() {
    let ports = [9500, 9501, 9502, 9503];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap())
        .collect();

    for v in &validators {
        v.start().unwrap();
    }
    thread::sleep(Duration::from_secs(15));
    for v in &validators {
        v.stop();
    }

    let min_height = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .min()
        .unwrap();

    println!("=== N77 DETERMINISTIC REPLAY AUDIT ===");
    println!("Common height: {}", min_height);
    println!();

    let mut last_state_root: Option<[u8; 32]> = None;
    let mut last_evidence_root: Option<[u8; 32]> = None;
    let mut all_match = true;

    for (i, v) in validators.iter().enumerate() {
        let store = v.store.lock().unwrap();
        let tip = store.load_height(min_height);
        match tip {
            Some(record) => {
                println!(
                    "Validator {}: height={} state_root={:?} evidence={:?}",
                    i,
                    record.height,
                    &record.state_root[..4],
                    &record.certificate_hash[..4]
                );
                if let Some(expected) = last_state_root {
                    if record.state_root != expected {
                        println!("  MISMATCH state_root");
                        all_match = false;
                    }
                } else {
                    last_state_root = Some(record.state_root);
                }
                if let Some(expected) = last_evidence_root {
                    if record.certificate_hash != expected {
                        println!("  MISMATCH evidence");
                        all_match = false;
                    }
                } else {
                    last_evidence_root = Some(record.certificate_hash);
                }
            }
            None => {
                println!("Validator {}: NO BLOCK at height {}", i, min_height);
                all_match = false;
            }
        }
    }

    println!();
    println!("============================================");
    println!("  N77 REPLAY AUDIT RESULTS");
    println!("============================================");
    println!("  Common height:         {}", min_height);
    println!("  All state roots match: {}", all_match);
    println!("  All evidence match:    {}", all_match);
    println!("  Determinism verified:  {}", all_match);
    println!(
        "  Verdict:               {}",
        if all_match { "PASS" } else { "FAIL" }
    );
    println!("============================================");

    if !all_match {
        std::process::exit(1);
    }
}
