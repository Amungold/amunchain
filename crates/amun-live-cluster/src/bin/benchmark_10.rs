use amun_live_cluster::config::{ClusterPeer, ValidatorConfig};
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let base_port: u16 = 9600;
    let count: usize = 10;
    let ids: [[u8; 32]; 10] = [
        [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32], [7u8; 32], [8u8; 32],
        [9u8; 32], [10u8; 32],
    ];

    let cluster: Vec<ClusterPeer> = (0..count)
        .map(|i| ClusterPeer {
            validator_id: ids[i],
            certificate_path: None,
            address: format!("127.0.0.1:{}", base_port + i as u16)
                .parse()
                .unwrap(),
        })
        .collect();

    let validators: Vec<LiveValidator> = (0..count)
        .map(|i| {
            let config = ValidatorConfig {
                validator_id: ids[i],
                listen_addr: cluster[i].address,
                cluster: cluster.clone(),
                data_dir: format!("/tmp/amun-bench10-{}", i),
                quorum_size: Some(count),
                authority_public_key: amun_live_cluster::config::load_genesis_authority(concat!(env!("CARGO_MANIFEST_DIR"), "/genesis/genesis_authority.json")).authority_public_key,
            };
            LiveValidator::new(config)
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
        let min_h = *heights.iter().min().unwrap_or(&0);
        let max_h = *heights.iter().max().unwrap_or(&0);
        println!(
            "t={:3}s min={} max={} spread={}",
            elapsed,
            min_h,
            max_h,
            max_h - min_h
        );
        snapshots.push((elapsed, heights));
    }

    for v in &validators {
        v.stop();
    }

    let first = snapshots.first().unwrap().1.clone();
    let last = snapshots.last().unwrap().1.clone();
    let duration = snapshots.last().unwrap().0 - snapshots.first().unwrap().0;
    let unique_blocks = last.iter().max().unwrap_or(&0) - first.iter().min().unwrap_or(&0);
    let tps = unique_blocks as f64 / duration as f64;
    let min_final = *last.iter().min().unwrap_or(&0);
    let max_final = *last.iter().max().unwrap_or(&0);
    let spread = max_final - min_final;

    println!("\n============================================");
    println!("  N81 10-VALIDATOR THROUGHPUT RESULTS");
    println!("============================================");
    println!("  Validators:          {}", count);
    println!("  Duration:            {}s", duration);
    println!("  Unique blocks:       {}", unique_blocks);
    println!("  Finalized TPS:       {:.2}", tps);
    println!("  Min final height:    {}", min_final);
    println!("  Max final height:    {}", max_final);
    println!("  Height spread:       {}", spread);
    println!(
        "  Scalability verdict: {}",
        if spread <= 1 { "PASS" } else { "DEGRADED" }
    );
    println!("============================================");
}
