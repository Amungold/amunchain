use std::time::Instant;

pub struct BenchResult {
    pub name: String,
    pub duration_ms: u64,
    pub ops_per_sec: f64,
}

pub fn time_op<F>(name: &str, f: F) -> BenchResult
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    let duration_ms = elapsed.as_millis() as u64;
    let ops_per_sec = if duration_ms > 0 {
        1_000.0 / duration_ms as f64
    } else {
        f64::MAX
    };
    BenchResult {
        name: name.to_string(),
        duration_ms,
        ops_per_sec,
    }
}
