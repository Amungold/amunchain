use crate::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResourceStatus {
    pub memory_mb: u64,
    pub cpu_pct: f64,
    pub disk_data_kb: u64,
    pub disk_wal_kb: u64,
    pub fds: u64,
    pub connections: u64,
    pub uptime_secs: u64,
    pub blocks_per_min: u64,
}

fn get_stat(key: &str) -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.split_whitespace().nth(1).map(|v| v.parse().unwrap_or(0)))
        .unwrap_or(0)
}

pub async fn resources(State(_state): State<AppState>) -> Json<ResourceStatus> {
    let rss_kb = get_stat("VmRSS:");
    let fds = std::fs::read_dir("/proc/self/fd")
        .map(|d| d.count() as u64)
        .unwrap_or(0);
    let uptime = std::fs::read_to_string("/proc/uptime")
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0) as u64;
    let connections = std::process::Command::new("sh")
        .arg("-c")
        .arg("ss -tanp 2>/dev/null | grep -c 'pid='")
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    let loadavg = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let cpu_pct = loadavg
        .split_whitespace()
        .next()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0)
        * 10.0;

    Json(ResourceStatus {
        memory_mb: rss_kb / 1024,
        cpu_pct,
        disk_data_kb: 0,
        disk_wal_kb: 0,
        fds,
        connections,
        uptime_secs: uptime,
        blocks_per_min: 0,
    })
}
