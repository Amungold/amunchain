use amun_mempool_gossip::messages::Transaction;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

struct LoadConfig {
    tx_count: u64,
    rate_per_second: u64,
    validators: Vec<SocketAddr>,
}

struct LoadResult {
    total_tx: u64,
    total_time_secs: f64,
    tps_average: f64,
    latencies: Vec<f64>,
}

impl LoadResult {
    fn p50(&self) -> String {
        if self.latencies.is_empty() {
            "N/A".into()
        } else {
            format!("{:.2}ms", percentile(&self.latencies, 50.0))
        }
    }
    fn p95(&self) -> String {
        if self.latencies.is_empty() {
            "N/A".into()
        } else {
            format!("{:.2}ms", percentile(&self.latencies, 95.0))
        }
    }
    fn p99(&self) -> String {
        if self.latencies.is_empty() {
            "N/A".into()
        } else {
            format!("{:.2}ms", percentile(&self.latencies, 99.0))
        }
    }

    fn min_lat_str(&self) -> String {
        if self.latencies.is_empty() {
            "N/A".into()
        } else {
            format!(
                "{:.2}ms",
                self.latencies.iter().cloned().fold(f64::INFINITY, f64::min)
            )
        }
    }
    fn max_lat_str(&self) -> String {
        if self.latencies.is_empty() {
            "N/A".into()
        } else {
            format!(
                "{:.2}ms",
                self.latencies.iter().cloned().fold(0.0_f64, f64::max)
            )
        }
    }

    fn print_report(&self) {
        println!("═══════════════════════════════════════");
        println!("  N73-A LOAD TEST REPORT");
        println!("═══════════════════════════════════════");
        println!("  Total TX:       {}", self.total_tx);
        println!("  Total Time:     {:.2}s", self.total_time_secs);
        println!("  TPS Average:    {:.0}", self.tps_average);
        println!("  P50 Latency:    {}", self.p50());
        println!("  P95 Latency:    {}", self.p95());
        println!("  P99 Latency:    {}", self.p99());
        println!("  Min Latency:    {}", self.min_lat_str());
        println!("  Max Latency:    {}", self.max_lat_str());
        println!("═══════════════════════════════════════");
    }

    fn save_json(&self, path: &str) {
        let json = serde_json::json!({
            "test": "N73-A",
            "total_tx": self.total_tx,
            "total_time_secs": self.total_time_secs,
            "tps_average": self.tps_average,
            "p50_latency_ms": self.p50(),
            "p95_latency_ms": self.p95(),
            "p99_latency_ms": self.p99(),
            "min_latency_ms": self.min_lat_str(),
            "max_latency_ms": self.max_lat_str(),
            "latencies": &self.latencies,
        });
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())
            .unwrap();
        println!("  Report saved: {}", path);
    }
}

fn percentile(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

fn make_tx(nonce: u64) -> Transaction {
    let mut tx = Transaction {
        tx_hash: [0u8; 32],
        sender: [1u8; 32],
        recipient: [2u8; 32],
        amount: 100,
        nonce,
        signature: [0u8; 64],
        timestamp: 0,
    };
    tx.tx_hash = tx.compute_hash();
    tx
}

fn send_tx(addr: SocketAddr, tx: &Transaction) -> Result<f64, String> {
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(2))
        .map_err(|e| format!("Connect: {}", e))?;
    stream
        .set_nonblocking(false)
        .map_err(|e| format!("Nonblocking: {}", e))?;

    let data = postcard::to_stdvec(tx).map_err(|e| format!("Encode: {}", e))?;
    let len = data.len() as u32;
    stream
        .write_all(&len.to_be_bytes())
        .map_err(|e| format!("Write len: {}", e))?;
    stream
        .write_all(&data)
        .map_err(|e| format!("Write data: {}", e))?;
    stream.flush().map_err(|e| format!("Flush: {}", e))?;

    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

fn run_load_test(config: LoadConfig) -> LoadResult {
    let mut latencies = Vec::with_capacity(config.tx_count as usize);
    let start = Instant::now();
    let delay = if config.rate_per_second > 0 {
        Duration::from_secs_f64(1.0 / config.rate_per_second as f64)
    } else {
        Duration::from_secs(0)
    };

    for i in 0..config.tx_count {
        let tx = make_tx(i);
        let addr = config.validators[(i as usize) % config.validators.len()];

        match send_tx(addr, &tx) {
            Ok(lat) => latencies.push(lat),
            Err(e) => eprintln!("TX {} failed: {}", i, e),
        }

        if i % 1000 == 0 {
            println!("  Progress: {}/{}", i, config.tx_count);
        }

        if config.rate_per_second > 0 {
            std::thread::sleep(delay);
        }
    }

    let total_time = start.elapsed().as_secs_f64();
    let total_time = if total_time.is_finite() && total_time > 0.0 {
        total_time
    } else {
        0.001
    };
    let total_tx = latencies.len() as u64;

    LoadResult {
        total_tx,
        total_time_secs: total_time,
        tps_average: if total_time > 0.0 {
            total_tx as f64 / total_time
        } else {
            0.0
        },
        latencies,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tx_count: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let rate: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let ports: Vec<u16> = args
        .get(3)
        .map(|s| s.split(',').filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_else(|| vec![10001, 10002, 10003, 10004]);

    let validators: Vec<SocketAddr> = ports
        .iter()
        .map(|p| format!("127.0.0.1:{}", p).parse().unwrap())
        .collect();

    println!("═══════════════════════════════════════");
    println!("  AmunChain Load Generator — N73-A");
    println!("═══════════════════════════════════════");
    println!("  TX Count:    {}", tx_count);
    println!(
        "  Rate:        {} tx/s",
        if rate > 0 {
            rate.to_string()
        } else {
            "unlimited".into()
        }
    );
    println!("  Validators:  {:?}", ports);
    println!("═══════════════════════════════════════");

    let config = LoadConfig {
        tx_count,
        rate_per_second: rate,
        validators,
    };

    let result = run_load_test(config);
    result.print_report();
    result.save_json("docs/phase2/N73-A_LOAD_TEST_REPORT.json");
    println!("═══════════════════════════════════════");
}
