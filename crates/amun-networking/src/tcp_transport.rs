use crate::backpressure::BackpressureManager;
use crate::capability_enforcement::CapabilityEnforcer;
use crate::connection_state::{ConnectionInfo, ConnectionState};
use crate::frame_codec::FrameCodec;
use crate::global_rate_limiter::GlobalRateLimiter;
use crate::handshake::{
    AuthConfirmation, AuthProof, ChallengeResponse, ConstitutionInfo, HandshakeError, HelloRequest,
};
use crate::priority_queue::{MessagePriority, PriorityQueue};
use crate::transport_trait::Transport;
use std::collections::{HashMap, VecDeque};
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const IDLE_TIMEOUT_SECS: u64 = 60;
const MAX_REQUESTS_PER_SEC: f64 = 100.0;
const TOKEN_BUCKET_SIZE: f64 = 200.0;
const MAX_BACKOFF_ATTEMPTS: u32 = 10;
const GLOBAL_MAX_MSGS_PER_SEC: u64 = 10000;
const GLOBAL_MAX_BYTES_PER_SEC: u64 = 100 * 1024 * 1024;
const HANDSHAKE_TIMEOUT_SECS: u64 = 10;

struct PeerConnection {
    stream: TcpStream,
    codec: FrameCodec,
    write_queue: PriorityQueue,
    last_seen: Instant,
    tokens: f64,
    last_token_refill: Instant,
    is_persistent: bool,
    connection_info: ConnectionInfo,
    backpressure: BackpressureManager,
    handshake_completed: bool,
    handshake_started: Instant,
    peer_node_id: Option<[u8; 32]>,
    session_id: Option<[u8; 32]>,
}

struct PendingPeer {
    addr: SocketAddr,
    retry_after: Instant,
    attempt: u32,
}

#[derive(Default)]
pub struct TransportMetrics {
    pub logical_frames_sent: AtomicU64,
    pub physical_frames_sent: AtomicU64,
    pub frames_received: AtomicU64,
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
    pub disconnects: AtomicU64,
    pub reconnects: AtomicU64,
    pub queue_overflows: AtomicU64,
    pub rate_limited: AtomicU64,
    pub idle_timeouts: AtomicU64,
    pub connection_attempts: AtomicU64,
    pub backpressure_drops: AtomicU64,
    pub connections_active: AtomicU64,
    pub handshake_success: AtomicU64,
    pub handshake_failed: AtomicU64,
    pub global_rate_limited: AtomicU64,
    pub capability_violations: AtomicU64,
    pub auth_failures: AtomicU64,
}

type InboxQueue = Arc<Mutex<VecDeque<(SocketAddr, Arc<[u8]>)>>>;
pub struct TcpTransport {
    listen_addr: SocketAddr,
    listener: Option<TcpListener>,
    peers: Arc<Mutex<HashMap<SocketAddr, PeerConnection>>>,
    pub session_manager: crate::session_manager::SessionManager,
    inbox: InboxQueue,
    pending_connections: Arc<Mutex<VecDeque<PendingPeer>>>,
    metrics: TransportMetrics,
    next_request_id: AtomicU64,
    network_id: [u8; 32],
    genesis_hash: [u8; 32],
    node_id: [u8; 32],
    signing_key: ed25519_dalek::SigningKey,
    constitution: ConstitutionInfo,
    global_limiter: GlobalRateLimiter,
    capability_enforcer: CapabilityEnforcer,
}

impl TcpTransport {
    pub fn new(
        listen_addr: SocketAddr,
        network_id: [u8; 32],
        genesis_hash: [u8; 32],
        node_id: [u8; 32],
        signing_key: ed25519_dalek::SigningKey,
        constitution: ConstitutionInfo,
    ) -> Self {
        Self {
            listen_addr,
            listener: None,
            peers: Arc::new(Mutex::new(HashMap::new())),
            session_manager: crate::session_manager::SessionManager::new(),
            inbox: Arc::new(Mutex::new(VecDeque::new())),
            pending_connections: Arc::new(Mutex::new(VecDeque::new())),
            metrics: TransportMetrics::default(),
            next_request_id: AtomicU64::new(1),
            network_id,
            genesis_hash,
            node_id,
            signing_key,
            constitution,
            global_limiter: GlobalRateLimiter::new(
                GLOBAL_MAX_MSGS_PER_SEC,
                GLOBAL_MAX_BYTES_PER_SEC,
            ),
            capability_enforcer: CapabilityEnforcer::new(),
        }
    }

    pub fn metrics(&self) -> &TransportMetrics {
        &self.metrics
    }

    fn perform_outgoing_handshake(
        &self,
        stream: &mut TcpStream,
    ) -> Result<(AuthConfirmation, Vec<String>), HandshakeError> {
        let hello = HelloRequest::new(
            self.network_id,
            self.genesis_hash,
            self.node_id,
            &self.signing_key,
            self.constitution.clone(),
            vec![
                "sync".to_string(),
                "vote".to_string(),
                "block_range".to_string(),
            ],
        );

        let encoded =
            postcard::to_stdvec(&hello).map_err(|e| HandshakeError::IoError(e.to_string()))?;
        let framed = FrameCodec::encode(&encoded);
        stream
            .write_all(&framed)
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;
        stream
            .flush()
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;

        let mut codec = FrameCodec::new();
        let frames = codec
            .decode(stream)
            .map_err(|e| HandshakeError::IoError(format!("{:?}", e)))?;
        if frames.is_empty() {
            return Err(HandshakeError::Timeout);
        }

        let challenge: ChallengeResponse =
            postcard::from_bytes(&frames[0]).map_err(|e| HandshakeError::IoError(e.to_string()))?;

        if !challenge.accepted {
            return Err(HandshakeError::IoError(
                challenge.reason.unwrap_or("Rejected".to_string()),
            ));
        }

        let proof = AuthProof::create(self.node_id, challenge.nonce, &self.signing_key);
        let encoded =
            postcard::to_stdvec(&proof).map_err(|e| HandshakeError::IoError(e.to_string()))?;
        let framed = FrameCodec::encode(&encoded);
        stream
            .write_all(&framed)
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;
        stream
            .flush()
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;

        let frames = codec
            .decode(stream)
            .map_err(|e| HandshakeError::IoError(format!("{:?}", e)))?;
        if frames.is_empty() {
            return Err(HandshakeError::Timeout);
        }

        let confirmation: AuthConfirmation =
            postcard::from_bytes(&frames[0]).map_err(|e| HandshakeError::IoError(e.to_string()))?;

        if !confirmation.accepted {
            self.metrics.auth_failures.fetch_add(1, Ordering::Relaxed);
            return Err(HandshakeError::AuthenticationFailed);
        }

        self.metrics
            .handshake_success
            .fetch_add(1, Ordering::Relaxed);
        Ok((confirmation, challenge.capabilities))
    }

    fn perform_incoming_handshake(
        &self,
        stream: &mut TcpStream,
    ) -> Result<([u8; 32], Vec<String>, AuthConfirmation), HandshakeError> {
        let mut codec = FrameCodec::new();
        let frames = codec
            .decode(stream)
            .map_err(|e| HandshakeError::IoError(format!("{:?}", e)))?;
        if frames.is_empty() {
            return Err(HandshakeError::Timeout);
        }

        let hello: HelloRequest =
            postcard::from_bytes(&frames[0]).map_err(|e| HandshakeError::IoError(e.to_string()))?;

        hello.validate_basic(self.network_id, self.genesis_hash, &self.constitution)?;
        hello.validate_capabilities(&["sync".to_string(), "vote".to_string()])?;

        let challenge = ChallengeResponse::create_challenge(
            self.node_id,
            &self.signing_key,
            self.constitution.clone(),
            vec![
                "sync".to_string(),
                "vote".to_string(),
                "block_range".to_string(),
            ],
        );

        let encoded =
            postcard::to_stdvec(&challenge).map_err(|e| HandshakeError::IoError(e.to_string()))?;
        let framed = FrameCodec::encode(&encoded);
        stream
            .write_all(&framed)
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;
        stream
            .flush()
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;

        let frames = codec
            .decode(stream)
            .map_err(|e| HandshakeError::IoError(format!("{:?}", e)))?;
        if frames.is_empty() {
            return Err(HandshakeError::Timeout);
        }

        let proof: AuthProof =
            postcard::from_bytes(&frames[0]).map_err(|e| HandshakeError::IoError(e.to_string()))?;

        proof.verify(challenge.nonce, hello.verifying_key)?;

        let confirmation = AuthConfirmation::accept();
        let encoded = postcard::to_stdvec(&confirmation)
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;
        let framed = FrameCodec::encode(&encoded);
        stream
            .write_all(&framed)
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;
        stream
            .flush()
            .map_err(|e| HandshakeError::IoError(e.to_string()))?;

        self.metrics
            .handshake_success
            .fetch_add(1, Ordering::Relaxed);
        Ok((hello.node_id, hello.capabilities, confirmation))
    }

    fn refill_tokens(conn: &mut PeerConnection) {
        let now = Instant::now();
        let elapsed = now.duration_since(conn.last_token_refill).as_secs_f64();
        conn.tokens = (conn.tokens + elapsed * MAX_REQUESTS_PER_SEC).min(TOKEN_BUCKET_SIZE);
        conn.last_token_refill = now;
    }

    fn schedule_reconnect(&self, addr: SocketAddr) {
        let mut pending = self.pending_connections.lock().unwrap();
        let already_pending = pending.iter().any(|p| p.addr == addr);
        if !already_pending {
            pending.push_back(PendingPeer {
                addr,
                retry_after: Instant::now(),
                attempt: 0,
            });
        }
    }

    fn create_peer_connection(stream: TcpStream, is_persistent: bool) -> PeerConnection {
        PeerConnection {
            stream,
            codec: FrameCodec::new(),
            write_queue: PriorityQueue::new(),
            last_seen: Instant::now(),
            tokens: TOKEN_BUCKET_SIZE,
            last_token_refill: Instant::now(),
            is_persistent,
            connection_info: ConnectionInfo::new(),
            backpressure: BackpressureManager::new(),
            handshake_completed: false,
            handshake_started: Instant::now(),
            peer_node_id: None,
            session_id: None,
        }
    }

    fn accept_new_connections(&self) {
        let listener = match &self.listener {
            Some(l) => l,
            None => return,
        };

        let mut new_connections = Vec::new();
        loop {
            match listener
                .accept()
                .map(|(stream, addr)| {
                    stream.set_nodelay(true).ok();
                    (stream, addr)
                })
                .map(|(stream, addr)| {
                    stream.set_nodelay(true).ok();
                    (stream, addr)
                })
                .map(|(stream, addr)| {
                    stream.set_nodelay(true).ok();
                    (stream, addr)
                })
                .map(|(stream, addr)| {
                    stream.set_nodelay(true).ok();
                    (stream, addr)
                }) {
                Ok((mut stream, addr)) => {
                    stream.set_nonblocking(true).ok();

                    match self.perform_incoming_handshake(&mut stream) {
                        Ok((peer_node_id, capabilities, confirmation)) => {
                            new_connections.push((
                                addr,
                                stream,
                                peer_node_id,
                                capabilities,
                                confirmation,
                            ));
                        }
                        Err(e) => {
                            eprintln!("Incoming handshake failed for {}: {:?}", addr, e);
                            self.metrics
                                .handshake_failed
                                .fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        if !new_connections.is_empty() {
            let mut peers = self.peers.lock().unwrap();
            for (addr, stream, peer_node_id, capabilities, confirmation) in new_connections {
                let mut conn = Self::create_peer_connection(stream, false);
                conn.handshake_completed = true;
                // N132.2A: Register session
                conn.session_id = Some(confirmation.session_id);
                self.session_manager
                    .register_from_handshake_with_id(addr, confirmation.session_id);
                conn.peer_node_id = Some(peer_node_id);
                // N132.2A: Use confirmation.session_id as source of truth
                conn.session_id = Some(confirmation.session_id);
                self.session_manager
                    .register_from_handshake_with_id(addr, confirmation.session_id);
                conn.connection_info.transition(ConnectionState::Connected);

                self.capability_enforcer.register_peer(addr, capabilities);
                peers.insert(addr, conn);
                self.metrics
                    .connections_active
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn process_pending_connections(&self) {
        let work: VecDeque<PendingPeer> = {
            let mut pending = self.pending_connections.lock().unwrap();
            std::mem::take(&mut *pending)
        };

        let now = Instant::now();
        let mut remaining = VecDeque::new();
        let mut new_connections = Vec::new();

        for peer in work {
            self.metrics
                .connection_attempts
                .fetch_add(1, Ordering::Relaxed);

            if now < peer.retry_after {
                remaining.push_back(peer);
                continue;
            }

            {
                let peers = self.peers.lock().unwrap();
                if peers.contains_key(&peer.addr) {
                    continue;
                }
            }

            if peer.attempt > MAX_BACKOFF_ATTEMPTS {
                continue;
            }

            match TcpStream::connect_timeout(&peer.addr, std::time::Duration::from_millis(100))
                .inspect(|s| {
                    s.set_nodelay(true).ok();
                }) {
                Ok(mut stream) => {
                    stream.set_nonblocking(true).ok();

                    match self.perform_outgoing_handshake(&mut stream) {
                        Ok((confirmation, capabilities)) => {
                            self.metrics.reconnects.fetch_add(1, Ordering::Relaxed);
                            new_connections.push((peer.addr, stream, confirmation, capabilities));
                        }
                        Err(e) => {
                            eprintln!("Outgoing handshake failed for {}: {:?}", peer.addr, e);
                            let backoff_ms = 500 * 2u64.pow(peer.attempt.min(6));
                            remaining.push_back(PendingPeer {
                                addr: peer.addr,
                                retry_after: now + std::time::Duration::from_millis(backoff_ms),
                                attempt: peer.attempt + 1,
                            });
                        }
                    }
                }
                Err(_) => {
                    let backoff_ms = 500 * 2u64.pow(peer.attempt.min(6));
                    remaining.push_back(PendingPeer {
                        addr: peer.addr,
                        retry_after: now + std::time::Duration::from_millis(backoff_ms),
                        attempt: peer.attempt + 1,
                    });
                }
            }
        }

        if !new_connections.is_empty() {
            let mut peers = self.peers.lock().unwrap();
            for (addr, stream, confirmation, capabilities) in new_connections {
                let mut conn = Self::create_peer_connection(stream, true);
                conn.handshake_completed = true;
                // N132.2A: Register session
                conn.session_id = Some(confirmation.session_id);
                self.session_manager
                    .register_from_handshake_with_id(addr, confirmation.session_id);
                conn.session_id = Some(confirmation.session_id);
                conn.connection_info.transition(ConnectionState::Connected);

                self.capability_enforcer.register_peer(addr, capabilities);
                peers.insert(addr, conn);
                self.metrics
                    .connections_active
                    .fetch_add(1, Ordering::Relaxed);
            }
        }

        {
            let mut pending = self.pending_connections.lock().unwrap();
            *pending = remaining;
        }
    }

    fn write_to_peers(&self) {
        let mut peers = self.peers.lock().unwrap();
        let mut to_drop = Vec::new();
        let mut to_reconnect = Vec::new();

        for (addr, conn) in peers.iter_mut() {
            if !conn.handshake_completed {
                if conn.handshake_started.elapsed().as_secs() > HANDSHAKE_TIMEOUT_SECS {
                    to_drop.push(*addr);
                    if conn.is_persistent {
                        to_reconnect.push(*addr);
                    }
                }
                continue;
            }

            let _backpressure_state = conn
                .backpressure
                .check(conn.write_queue.len(), conn.write_queue.bytes());

            while let Some(pending) = conn.write_queue.front_mut() {
                let remaining = &pending.data[pending.offset..];
                match conn.stream.write(remaining) {
                    Ok(0) => {
                        to_drop.push(*addr);
                        if conn.is_persistent {
                            to_reconnect.push(*addr);
                        }
                        break;
                    }
                    Ok(n) => {
                        pending.offset += n;
                        conn.last_seen = Instant::now();
                        if pending.offset >= pending.data.len() {
                            conn.write_queue.pop_front();
                        } else {
                            break;
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(e) => {
                        conn.connection_info
                            .set_error(format!("Write error: {}", e));
                        to_drop.push(*addr);
                        if conn.is_persistent {
                            to_reconnect.push(*addr);
                        }
                        break;
                    }
                }
            }

            let _ = conn.stream.flush();
        }

        for addr in to_drop {
            if let Some(_conn) = peers.remove(&addr) {
                self.metrics.disconnects.fetch_add(1, Ordering::Relaxed);
                self.metrics
                    .connections_active
                    .fetch_sub(1, Ordering::Relaxed);
                self.capability_enforcer.remove_peer(&addr);
            }
        }

        drop(peers);

        for addr in to_reconnect {
            self.schedule_reconnect(addr);
        }
    }

    fn read_from_peers(&self) {
        let mut peers = self.peers.lock().unwrap();
        let mut inbox = self.inbox.lock().unwrap();
        let mut to_drop = Vec::new();
        let mut to_reconnect = Vec::new();

        for (addr, conn) in peers.iter_mut() {
            if !conn.handshake_completed {
                continue;
            }

            match conn.codec.decode(&mut conn.stream) {
                Ok(frames) => {
                    for frame in frames {
                        self.metrics.frames_received.fetch_add(1, Ordering::Relaxed);
                        self.metrics
                            .bytes_received
                            .fetch_add(frame.len() as u64, Ordering::Relaxed);
                        inbox.push_back((*addr, Arc::from(frame.as_ref())));
                    }
                    conn.last_seen = Instant::now();
                }
                Err(e) => {
                    conn.connection_info
                        .set_error(format!("Decode error: {:?}", e));
                    to_drop.push(*addr);
                    if conn.is_persistent {
                        to_reconnect.push(*addr);
                    }
                }
            }
        }

        for addr in to_drop {
            if let Some(_conn) = peers.remove(&addr) {
                self.metrics.disconnects.fetch_add(1, Ordering::Relaxed);
                self.metrics
                    .connections_active
                    .fetch_sub(1, Ordering::Relaxed);
                self.capability_enforcer.remove_peer(&addr);
            }
        }

        drop(peers);

        for addr in to_reconnect {
            self.schedule_reconnect(addr);
        }
    }

    fn check_idle_timeouts(&self) {
        let mut peers = self.peers.lock().unwrap();
        let now = Instant::now();
        let mut to_drop = Vec::new();
        let mut to_reconnect = Vec::new();

        for (addr, conn) in peers.iter() {
            if conn.handshake_completed
                && now.duration_since(conn.last_seen).as_secs() > IDLE_TIMEOUT_SECS
            {
                to_drop.push(*addr);
                self.metrics.idle_timeouts.fetch_add(1, Ordering::Relaxed);
                if conn.is_persistent {
                    to_reconnect.push(*addr);
                }
            }
        }

        for addr in to_drop {
            if let Some(_conn) = peers.remove(&addr) {
                self.metrics.disconnects.fetch_add(1, Ordering::Relaxed);
                self.metrics
                    .connections_active
                    .fetch_sub(1, Ordering::Relaxed);
                self.capability_enforcer.remove_peer(&addr);
            }
        }

        drop(peers);

        for addr in to_reconnect {
            self.schedule_reconnect(addr);
        }
    }
}

impl Transport for TcpTransport {
    fn bind(&mut self) -> Result<(), String> {
        let listener =
            TcpListener::bind(self.listen_addr).map_err(|e| format!("Bind failed: {}", e))?;
        listener
            .set_nonblocking(true)
            .map_err(|e| format!("Set nonblocking failed: {}", e))?;
        self.listener = Some(listener);
        Ok(())
    }

    fn connect_to(&self, addr: SocketAddr) {
        self.connect_persistent(addr);
    }

    fn connect_persistent(&self, addr: SocketAddr) {
        let peers = self.peers.lock().unwrap();
        if peers.contains_key(&addr) {
            return;
        }
        drop(peers);
        self.schedule_reconnect(addr);
    }

    fn send_to(&self, peer: SocketAddr, data: Arc<[u8]>) -> Result<(), String> {
        if !self.global_limiter.try_consume(data.len()) {
            self.metrics
                .global_rate_limited
                .fetch_add(1, Ordering::Relaxed);
            return Err("Global rate limit exceeded".to_string());
        }

        let mut peers = self.peers.lock().unwrap();
        if let Some(conn) = peers.get_mut(&peer) {
            if !conn.handshake_completed {
                return Err("Handshake not completed".to_string());
            }

            Self::refill_tokens(conn);

            if conn.tokens < 1.0 {
                self.metrics.rate_limited.fetch_add(1, Ordering::Relaxed);
                return Err(format!("Peer {} rate limited", peer));
            }
            conn.tokens -= 1.0;

            if conn.backpressure.should_drop(
                data.len(),
                conn.write_queue.len(),
                conn.write_queue.bytes(),
            ) {
                self.metrics
                    .backpressure_drops
                    .fetch_add(1, Ordering::Relaxed);
                conn.write_queue.drop_lowest_priority();
            }

            conn.write_queue
                .push(Arc::clone(&data), MessagePriority::Normal);

            self.metrics
                .logical_frames_sent
                .fetch_add(1, Ordering::Relaxed);
            self.metrics
                .bytes_sent
                .fetch_add(data.len() as u64, Ordering::Relaxed);
            Ok(())
        } else {
            Err(format!("Peer {} not connected", peer))
        }
    }

    fn broadcast(&self, data: Arc<[u8]>) {
        let mut peers = self.peers.lock().unwrap();
        let mut broadcast_count = 0u64;

        for conn in peers.values_mut() {
            if !conn.handshake_completed {
                continue;
            }

            Self::refill_tokens(conn);

            if conn.tokens < 1.0 {
                continue;
            }
            conn.tokens -= 1.0;

            if conn.backpressure.should_drop(
                data.len(),
                conn.write_queue.len(),
                conn.write_queue.bytes(),
            ) {
                conn.write_queue.drop_lowest_priority();
            }

            conn.write_queue
                .push(Arc::clone(&data), MessagePriority::Critical);
            broadcast_count += 1;
        }

        if broadcast_count > 0 {
            self.metrics
                .logical_frames_sent
                .fetch_add(1, Ordering::Relaxed);
            self.metrics
                .physical_frames_sent
                .fetch_add(broadcast_count, Ordering::Relaxed);
            self.metrics
                .bytes_sent
                .fetch_add(data.len() as u64 * broadcast_count, Ordering::Relaxed);
        }
    }

    fn recv_from(&self) -> Option<(SocketAddr, Arc<[u8]>)> {
        let mut inbox = self.inbox.lock().unwrap();
        // N132.2B: Skip messages from unauthenticated peers
        while let Some((addr, data)) = inbox.pop_front() {
            if self.session_manager.verify_and_touch(&addr) {
                return Some((addr, data));
            }
        }
        None
    }

    fn tick(&mut self, _max_iterations: u32) {
        self.accept_new_connections();
        self.process_pending_connections();
        self.write_to_peers();
        self.read_from_peers();
        self.check_idle_timeouts();
    }

    fn next_request_id(&self) -> u64 {
        self.next_request_id.fetch_add(1, Ordering::Relaxed)
    }
}

// Backward compatibility methods (outside Transport trait)
impl TcpTransport {
    pub fn recv_raw(&self) -> Option<crate::payload::Payload> {
        self.recv_from()
            .map(|(_, data)| crate::payload::Payload::from(data.to_vec()))
    }

    pub fn send_raw(&self, data: crate::payload::Payload) {
        self.broadcast(std::sync::Arc::from(data.as_ref()));
    }
}
