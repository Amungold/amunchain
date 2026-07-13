use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let index: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let port: u16 = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10001 + index as u16);

    let base_port = port - index as u16;
    let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];

    let config = ValidatorConfig::test_cluster(index, &ports);

    println!("Validator {} listening on {}", index, config.listen_addr);
    println!(
        "Peers: {:?}",
        config
            .other_peers()
            .iter()
            .map(|p| p.address)
            .collect::<Vec<_>>()
    );

    let validator = LiveValidator::new(config);
    validator.start().unwrap();

    println!("Validator {} running. Press Ctrl+C to stop.", index);

    loop {
        let summary = validator.metrics_summary();
        if !summary.contains("rounds: 0") {
            eprintln!("METRICS: {}", summary);
        }
        thread::sleep(Duration::from_secs(1));
    }
}
