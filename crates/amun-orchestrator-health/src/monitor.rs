use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Check if a TCP endpoint is reachable.
pub async fn check_tcp_endpoint(addr: &str, timeout_secs: u64) -> bool {
    let addr = match addr.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => return false,
        },
        Err(_) => return false,
    };

    matches!(
        timeout(Duration::from_secs(timeout_secs), TcpStream::connect(addr)).await,
        Ok(Ok(_))
    )
}

/// Check if a validator's RPC endpoint is responding.
pub async fn check_rpc_health(rpc_port: u16, timeout_secs: u64) -> bool {
    let addr = format!("127.0.0.1:{}", rpc_port);
    check_tcp_endpoint(&addr, timeout_secs).await
}

/// Check if a validator is reachable on its P2P port.
pub async fn check_p2p_health(p2p_port: u16, timeout_secs: u64) -> bool {
    let addr = format!("127.0.0.1:{}", p2p_port);
    check_tcp_endpoint(&addr, timeout_secs).await
}
