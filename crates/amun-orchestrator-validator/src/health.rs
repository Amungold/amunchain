use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn wait_until_ready(addr: SocketAddr, timeout: Duration) -> Result<(), String> {
    let deadline = Instant::now() + timeout;

    loop {
        if TcpStream::connect(addr).is_ok() {
            return Ok(());
        }

        if Instant::now() >= deadline {
            return Err(format!("timeout waiting for {}", addr));
        }

        sleep(Duration::from_secs(1));
    }
}
