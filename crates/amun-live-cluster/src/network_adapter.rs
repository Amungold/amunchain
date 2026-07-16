use crate::fault_injector::FaultInjector;
use amun_networking::frame::{FrameKind, NetworkFrame};
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ValidatorNetworkAdapter {
    transport: Arc<Mutex<TcpTransport>>,
    /// Optional fault injector for R2.3 testing.
    /// None in production; Some(...) in fault injection tests.
    fault_injector: Option<Arc<FaultInjector>>,
    /// Buffered frames for deterministic message reordering.
    /// (serialized frame, destination peer; None = broadcast)
    reorder_buffer: Arc<Mutex<std::collections::VecDeque<(Vec<u8>, Option<SocketAddr>)>>>,
}

impl ValidatorNetworkAdapter {
    pub fn new(transport: Arc<Mutex<TcpTransport>>) -> Self {
        Self {
            transport,
            fault_injector: None,
            reorder_buffer: Arc::new(Mutex::new(std::collections::VecDeque::new())),
        }
    }

    /// Create with fault injection enabled (R2.3 testing).
    pub fn with_fault_injector(
        transport: Arc<Mutex<TcpTransport>>,
        fault_injector: Arc<FaultInjector>,
    ) -> Self {
        Self {
            transport,
            fault_injector: Some(fault_injector),
            reorder_buffer: Arc::new(Mutex::new(std::collections::VecDeque::new())),
        }
    }

    pub fn poll(&self) {
        let mut t = self.transport.lock().expect("mutex poisoned");
        t.tick(10);
    }

    pub fn broadcast_vote(&self, vote: Vec<u8>) {
        // R2.3: Fault injection — reorder, delay, then maybe drop
        if let Some(ref fi) = self.fault_injector {
            // Reorder first
            if let Some(buffer_size) = fi.should_reorder() {
                let frame = NetworkFrame::new(FrameKind::Vote, vote.into());
                let bytes = postcard::to_stdvec(&frame).expect("Frame serialization failed");
                {
                    let mut buffer = self.reorder_buffer.lock().expect("mutex poisoned");
                    buffer.push_back((bytes, None));
                }
                self.try_flush_reorder(buffer_size);
                return;
            }
            // Delay
            if let Some(ms) = fi.should_delay() {
                eprintln!("FAULT_DELAY: broadcast_vote {}ms", ms);
                std::thread::sleep(std::time::Duration::from_millis(ms));
            }
            // Drop
            if fi.should_drop() {
                eprintln!("FAULT_DROP: broadcast_vote");
                return;
            }
        }
        let frame = NetworkFrame::new(FrameKind::Vote, vote.into());
        let bytes = postcard::to_stdvec(&frame).expect("Frame serialization failed");
        let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
        let t = self.transport.lock().expect("mutex poisoned");
        t.broadcast(data);
    }

    pub fn send_to(&self, peer: SocketAddr, frame: NetworkFrame) -> Result<(), String> {
        // R2.3: Fault injection — reorder, delay, then maybe drop
        if let Some(ref fi) = self.fault_injector {
            // Reorder first
            if let Some(buffer_size) = fi.should_reorder() {
                let bytes = postcard::to_stdvec(&frame).map_err(|e| format!("encode: {}", e))?;
                {
                    let mut buffer = self.reorder_buffer.lock().expect("mutex poisoned");
                    buffer.push_back((bytes, Some(peer)));
                }
                self.try_flush_reorder(buffer_size);
                return Ok(());
            }
            // Delay
            if let Some(ms) = fi.should_delay() {
                eprintln!("FAULT_DELAY: send_to {} {:?} {}ms", peer, frame.kind, ms);
                std::thread::sleep(std::time::Duration::from_millis(ms));
            }
            // Drop
            if fi.should_drop() {
                eprintln!("FAULT_DROP: send_to {} {:?}", peer, frame.kind);
                return Ok(());
            }
        }
        let bytes = postcard::to_stdvec(&frame).map_err(|e| format!("encode: {}", e))?;
        let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
        let t = self.transport.lock().expect("mutex poisoned");
        t.send_to(peer, data)
    }

    #[allow(unused_mut)]
    pub fn recv_from(&self) -> Option<(SocketAddr, NetworkFrame)> {
        let mut t = self.transport.lock().expect("mutex poisoned");
        if let Some((addr, data)) = t.recv_from() {
            if let Ok(frame) = postcard::from_bytes::<NetworkFrame>(&data) {
                return Some((addr, frame));
            }
        }
        None
    }

    /// Flush the reorder buffer in LIFO order if it reaches capacity.
    fn try_flush_reorder(&self, buffer_size: usize) {
        let drained: Vec<(Vec<u8>, Option<SocketAddr>)> = {
            let mut buffer = self.reorder_buffer.lock().expect("mutex poisoned");
            if buffer.len() < buffer_size {
                return;
            }
            eprintln!("FAULT_REORDER_FLUSH: {} messages (LIFO)", buffer.len());
            let mut drained = Vec::with_capacity(buffer.len());
            while let Some(item) = buffer.pop_back() {
                drained.push(item);
            }
            drained
        };
        let t = self.transport.lock().expect("mutex poisoned");
        for (data, peer) in drained {
            let data: Arc<[u8]> = Arc::from(data.into_boxed_slice());
            if let Some(addr) = peer {
                let _ = t.send_to(addr, data);
            } else {
                t.broadcast(data);
            }
        }
    }

    /// Force-flush all remaining messages in the reorder buffer.
    pub fn force_flush_reorder(&self) {
        let drained: Vec<(Vec<u8>, Option<SocketAddr>)> = {
            let mut buffer = self.reorder_buffer.lock().expect("mutex poisoned");
            if buffer.is_empty() {
                return;
            }
            eprintln!(
                "FAULT_REORDER_FORCE_FLUSH: {} messages (LIFO)",
                buffer.len()
            );
            let mut drained = Vec::with_capacity(buffer.len());
            while let Some(item) = buffer.pop_back() {
                drained.push(item);
            }
            drained
        };
        let t = self.transport.lock().expect("mutex poisoned");
        for (data, peer) in drained {
            let data: Arc<[u8]> = Arc::from(data.into_boxed_slice());
            if let Some(addr) = peer {
                let _ = t.send_to(addr, data);
            } else {
                t.broadcast(data);
            }
        }
    }

    pub fn next_request_id(&self) -> u64 {
        let t = self.transport.lock().expect("mutex poisoned");
        t.next_request_id()
    }
}
