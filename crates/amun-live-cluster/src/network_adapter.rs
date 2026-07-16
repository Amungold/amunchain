use crate::fault_injector::{CorruptKind, FaultInjector};
use amun_consensus_network::consensus_message::ConsensusMessage;

/// Apply corruption to a NetworkFrame based on CorruptKind.
fn apply_corruption(frame: &mut NetworkFrame, kind: &CorruptKind) {
    // Convert Bytes to Vec<u8> for mutation, then set back
    let mut raw = frame.payload.to_vec();
    match kind {
        CorruptKind::InvalidSignature => {
            if raw.len() >= 64 {
                let start = raw.len() - 64;
                for b in &mut raw[start..] {
                    *b = 0;
                }
            }
        }
        CorruptKind::BitFlip => {
            if !raw.is_empty() {
                raw[0] ^= 0xFF;
            }
        }
        CorruptKind::WrongHeight => {
            if raw.len() >= 8 {
                raw[0..8].copy_from_slice(&999999u64.to_le_bytes());
            }
        }
        CorruptKind::WrongBlockHash => {
            if raw.len() >= 40 {
                for b in &mut raw[8..40] {
                    *b = 0;
                }
            }
        }
        CorruptKind::Truncated => {
            let half = raw.len() / 2;
            for b in &mut raw[half..] {
                *b = 0;
            }
        }
    }
    frame.payload = raw.into();
}

use amun_networking::frame::{FrameKind, NetworkFrame};
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

/// Buffered frame for reordering: (serialized_bytes, optional_peer).
type BufferedFrame = (Vec<u8>, Option<SocketAddr>);
/// Thread-safe reorder buffer.
type ReorderBuffer = Arc<Mutex<std::collections::VecDeque<BufferedFrame>>>;

#[derive(Clone)]
pub struct ValidatorNetworkAdapter {
    transport: Arc<Mutex<TcpTransport>>,
    /// Optional fault injector for R2.3 testing.
    /// None in production; Some(...) in fault injection tests.
    fault_injector: Option<Arc<FaultInjector>>,
    /// Buffered frames for deterministic message reordering.
    /// (serialized frame, destination peer; None = broadcast)
    reorder_buffer: ReorderBuffer,
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
        // R3.4: Delegate to unified consensus message path
        // Deserialize the raw vote bytes into ConsensusVote,
        // wrap in ConsensusMessage::Vote, and send via unified path.
        if let Ok(consensus_vote) =
            postcard::from_bytes::<amun_consensus_network::messages::ConsensusVote>(&vote)
        {
            self.broadcast_consensus_message(ConsensusMessage::Vote(consensus_vote));
        } else {
            // Fallback: send as raw FrameKind::Vote (old path)
            let frame = NetworkFrame::new(FrameKind::Vote, vote.into());
            let bytes = postcard::to_stdvec(&frame).expect("Frame serialization failed");
            let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
            let t = self.transport.lock().expect("mutex poisoned");
            t.broadcast(data);
        }
    }

    /// Broadcast a unified consensus message (proposal, vote, QC, or finality).
    pub fn broadcast_consensus_message(&self, msg: ConsensusMessage) {
        // R2.3 + R2.4: Fault injection
        let mut corrupt_kind: Option<CorruptKind> = None;
        if let Some(ref fi) = self.fault_injector {
            corrupt_kind = fi.should_corrupt();
            if let Some(buffer_size) = fi.should_reorder() {
                let frame = NetworkFrame::new(
                    FrameKind::ConsensusMessage,
                    postcard::to_stdvec(&msg)
                        .expect("ConsensusMessage serialization failed")
                        .into(),
                );
                let bytes = postcard::to_stdvec(&frame).expect("Frame serialization failed");
                {
                    let mut buffer = self.reorder_buffer.lock().expect("mutex poisoned");
                    buffer.push_back((bytes, None));
                }
                self.try_flush_reorder(buffer_size);
                return;
            }
            if let Some(ms) = fi.should_delay() {
                eprintln!("FAULT_DELAY: broadcast_consensus {}ms", ms);
                std::thread::sleep(std::time::Duration::from_millis(ms));
            }
            if fi.should_drop() {
                eprintln!("FAULT_DROP: broadcast_consensus");
                return;
            }
        }
        let mut frame = NetworkFrame::new(
            FrameKind::ConsensusMessage,
            postcard::to_stdvec(&msg)
                .expect("ConsensusMessage serialization failed")
                .into(),
        );
        if let Some(kind) = &corrupt_kind {
            eprintln!("FAULT_CORRUPT: broadcast_consensus {:?}", kind);
            apply_corruption(&mut frame, kind);
        }
        let bytes = postcard::to_stdvec(&frame).expect("Frame serialization failed");
        let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
        let t = self.transport.lock().expect("mutex poisoned");
        t.broadcast(data);
    }

    pub fn send_to(&self, peer: SocketAddr, frame: NetworkFrame) -> Result<(), String> {
        // R2.3 + R2.4: Fault injection
        let mut corrupt_kind: Option<CorruptKind> = None;
        if let Some(ref fi) = self.fault_injector {
            // Corrupt (R2.4)
            corrupt_kind = fi.should_corrupt();
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
        let mut frame = frame.clone();
        if let Some(kind) = &corrupt_kind {
            eprintln!(
                "FAULT_CORRUPT: send_to {} {:?} {:?}",
                peer, frame.kind, kind
            );
            apply_corruption(&mut frame, kind);
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
