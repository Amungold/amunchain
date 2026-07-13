use std::net::TcpListener;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

pub fn free_ports<const N: usize>() -> [u16; N] {
    let mut ports = [0u16; N];

    for p in &mut ports {
        *p = free_port();
    }

    ports
}

pub fn unique_test_dir(name: &str, validator: usize) -> String {
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let path = std::env::temp_dir().join(format!("amun-{}-{}-{}", name, id, validator));

    path.to_string_lossy().to_string()
}

pub fn cleanup(path: &str) {
    let _ = std::fs::remove_dir_all(PathBuf::from(path));
}
