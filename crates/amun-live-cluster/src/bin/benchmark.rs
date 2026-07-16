use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9100, 9101, 9102, 9103];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();
    let mut snapshots: Vec<(u64, Vec<u64>)> = Vec::new();

    for _sec in 0..60 {
        thread::sleep(Duration::from_secs(1));
        let elapsed = start.elapsed().as_secs();
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", elapsed, heights);
        snapshots.push((elapsed, heights));
    }

    for v in &validators {
        v.stop();
    }

    let first = snapshots.first().unwrap().1.clone();
    let last = snapshots.last().unwrap().1.clone();
    let duration = snapshots.last().unwrap().0 - snapshots.first().unwrap().0;

    let min_final = *last.iter().min().unwrap_or(&0);
    let max_final = *last.iter().max().unwrap_or(&0);
    let unique_blocks = last.iter().max().unwrap_or(&0) - first.iter().min().unwrap_or(&0);
    let tps = unique_blocks as f64 / duration as f64;

    println!("\n============================================");
    println!("  N74-E BENCHMARK RESULTS");
    println!("============================================");
    println!("  Duration:            {}s", duration);
    println!("  Start heights:       {:?}", first);
    println!("  Final heights:       {:?}", last);
    println!("  Unique blocks:       {}", unique_blocks);
    println!("  Finalized TPS:       {:.2}", tps);
    println!("  Min final height:    {}", min_final);
    println!("  Max final height:    {}", max_final);
    println!("  Height spread:       {}", max_final - min_final);
    println!("============================================");
}
