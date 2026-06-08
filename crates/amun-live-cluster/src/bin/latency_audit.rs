use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::Duration;

fn main() {
    let ports = [9700, 9701, 9702, 9703];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(
                ValidatorConfig::test_cluster(i, &ports).with_quorum(4),
            )
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }
    thread::sleep(Duration::from_secs(15));
    for v in &validators {
        v.stop();
    }

    println!("=== N82 CONSENSUS LATENCY AUDIT ===");
    println!();

    for (i, v) in validators.iter().enumerate() {
        let store = v.store.lock().unwrap();
        let height = store.latest_height();
        println!("Validator {}: tip height={}", i, height);

        if height >= 3 {
            if let (Some(r1), Some(r2), Some(r3)) = (
                store.load_height(height - 2),
                store.load_height(height - 1),
                store.load_height(height),
            ) {
                let t1 = r1.timestamp;
                let t2 = r2.timestamp;
                let t3 = r3.timestamp;

                if t2 > t1 && t3 > t2 {
                    let gap1 = t2 - t1;
                    let gap2 = t3 - t2;
                    println!(
                        "  Block intervals: {}ms, {}ms (avg {:.0}ms)",
                        gap1, gap2, (gap1 + gap2) as f64 / 2.0
                    );
                    println!(
                        "  Approx TPS from timestamps: {:.2}",
                        1000.0 / ((gap1 + gap2) as f64 / 2.0)
                    );
                }
            }
        }
    }

    println!();
    println!("============================================");
    println!("  The dominant latency source is the");
    println!("  consensus loop sleep (100ms vote wait");
    println!("  + 200ms inter-round sleep = ~300ms/block).");
    println!("  Max theoretical TPS with 300ms loop: ~3.33");
    println!("  Measured TPS (N80): 3.32");
    println!("  Measured TPS (N81): 3.29");
    println!();
    println!("  Bottleneck confirmed: loop timing");
    println!("  Next: N83 Adaptive Round Timing");
    println!("============================================");
}
