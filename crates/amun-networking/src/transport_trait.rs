use std::net::SocketAddr;
use std::sync::Arc;

pub trait Transport: Send + Sync {
    fn bind(&mut self) -> Result<(), String>;
    fn connect_to(&self, addr: SocketAddr);
    fn connect_persistent(&self, addr: SocketAddr);
    fn send_to(&self, peer: SocketAddr, data: Arc<[u8]>) -> Result<(), String>;
    fn broadcast(&self, data: Arc<[u8]>);
    fn recv_from(&self) -> Option<(SocketAddr, Arc<[u8]>)>;
    fn tick(&mut self, max_iterations: u32);
    fn next_request_id(&self) -> u64;
}
