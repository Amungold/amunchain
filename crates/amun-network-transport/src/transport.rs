use crate::message::NetworkMessage;
use crate::peer::{NodeId, PeerIdentity};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// Errors that can occur during network transport.
#[derive(Debug)]
pub enum TransportError {
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    EncodeFailed(String),
    DecodeFailed(String),
    PeerNotFound(NodeId),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::ConnectionFailed(s) => write!(f, "Connection failed: {}", s),
            TransportError::SendFailed(s) => write!(f, "Send failed: {}", s),
            TransportError::ReceiveFailed(s) => write!(f, "Receive failed: {}", s),
            TransportError::EncodeFailed(s) => write!(f, "Encode failed: {}", s),
            TransportError::DecodeFailed(s) => write!(f, "Decode failed: {}", s),
            TransportError::PeerNotFound(id) => {
                write!(f, "Peer not found: {:?}", id.0[..4].to_vec())
            }
        }
    }
}

/// Connection to a peer: a TCP stream with length-prefixed framing.
struct PeerConnection {
    stream: TcpStream,
    peer_id: NodeId,
}

impl PeerConnection {
    fn send(&mut self, message: &NetworkMessage) -> Result<(), TransportError> {
        let data = message.encode().map_err(TransportError::EncodeFailed)?;
        let len = data.len() as u32;
        self.stream
            .write_all(&len.to_be_bytes())
            .map_err(|e| TransportError::SendFailed(e.to_string()))?;
        self.stream
            .write_all(&data)
            .map_err(|e| TransportError::SendFailed(e.to_string()))?;
        self.stream
            .flush()
            .map_err(|e| TransportError::SendFailed(e.to_string()))?;
        Ok(())
    }

    fn receive(&mut self) -> Result<NetworkMessage, TransportError> {
        let mut len_buf = [0u8; 4];
        self.stream
            .read_exact(&mut len_buf)
            .map_err(|e| TransportError::ReceiveFailed(e.to_string()))?;
        let len = u32::from_be_bytes(len_buf) as usize;

        if len > 16 * 1024 * 1024 {
            return Err(TransportError::ReceiveFailed(format!(
                "Frame too large: {} bytes",
                len
            )));
        }

        let mut data = vec![0u8; len];
        self.stream
            .read_exact(&mut data)
            .map_err(|e| TransportError::ReceiveFailed(e.to_string()))?;

        NetworkMessage::decode(&data).map_err(TransportError::DecodeFailed)
    }
}

/// TCP-based network transport for validator communication.
pub struct TcpTransport {
    pub identity: PeerIdentity,
    peers: Arc<Mutex<HashMap<NodeId, PeerIdentity>>>,
    connections: Arc<Mutex<HashMap<NodeId, PeerConnection>>>,
    bind_address: SocketAddr,
    inbox: Arc<Mutex<Vec<(NodeId, NetworkMessage)>>>,
}

impl TcpTransport {
    pub fn new(identity: PeerIdentity, bind_address: SocketAddr) -> Self {
        Self {
            identity,
            peers: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            bind_address,
            inbox: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_peer(&self, peer: PeerIdentity) {
        self.peers
            .lock()
            .unwrap()
            .insert(peer.node_id.clone(), peer);
    }

    pub fn start_listen(&self) -> Result<(), TransportError> {
        let listener = TcpListener::bind(self.bind_address)
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;
        listener
            .set_nonblocking(true)
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        let connections = self.connections.clone();
        let inbox = self.inbox.clone();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let _ = stream.set_nonblocking(false);
                        let mut conn = PeerConnection {
                            stream,
                            peer_id: NodeId([0u8; 32]),
                        };
                        if let Ok(msg) = conn.receive() {
                            let sender_id = extract_sender_id(&msg);
                            conn.peer_id = sender_id.clone();
                            connections.lock().unwrap().insert(sender_id.clone(), conn);
                            inbox.lock().unwrap().push((sender_id, msg));
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(())
    }

    pub fn connect(&self, peer_id: &NodeId) -> Result<(), TransportError> {
        let peer = {
            let peers = self.peers.lock().unwrap();
            peers
                .get(peer_id)
                .cloned()
                .ok_or_else(|| TransportError::PeerNotFound(peer_id.clone()))?
        };

        let stream = TcpStream::connect(peer.address)
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;
        stream
            .set_nonblocking(false)
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        let conn = PeerConnection {
            stream,
            peer_id: peer.node_id.clone(),
        };
        self.connections.lock().unwrap().insert(peer.node_id, conn);
        Ok(())
    }

    pub fn send(&self, peer_id: &NodeId, message: &NetworkMessage) -> Result<(), TransportError> {
        let mut connections = self.connections.lock().unwrap();
        let conn = connections
            .get_mut(peer_id)
            .ok_or_else(|| TransportError::PeerNotFound(peer_id.clone()))?;
        conn.send(message)
    }

    pub fn broadcast(&self, message: &NetworkMessage) -> Result<(), TransportError> {
        let mut connections = self.connections.lock().unwrap();
        let mut errors = Vec::new();
        for (peer_id, conn) in connections.iter_mut() {
            if let Err(e) = conn.send(message) {
                errors.push(format!("{}: {}", hex::encode(&peer_id.0[..4]), e));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(TransportError::SendFailed(errors.join("; ")))
        }
    }

    pub fn poll_messages(&self) -> Vec<(NodeId, NetworkMessage)> {
        let mut inbox = self.inbox.lock().unwrap();
        std::mem::take(&mut *inbox)
    }
}

fn extract_sender_id(msg: &NetworkMessage) -> NodeId {
    match msg {
        NetworkMessage::BlockAnnounce(b) => NodeId(b.validator_id.0),
        NetworkMessage::CertificateAnnounce(c) => {
            if let Some(first) = c.quorum_signers.first() {
                NodeId(first.0)
            } else {
                NodeId([0u8; 32])
            }
        }
        NetworkMessage::StateSyncRequest(r) => NodeId(r.requester_id),
        NetworkMessage::StateSyncResponse(_) => NodeId([0u8; 32]),
        NetworkMessage::Ping(p) => NodeId(p.sender_id),
        NetworkMessage::Pong(p) => NodeId(p.sender_id),
        NetworkMessage::SlashingCertificateAnnounce(s) => NodeId(s.validator_id),
        NetworkMessage::EvidenceAnnounce(e) => NodeId(e.validator_id),
        NetworkMessage::EvidencePush(p) => NodeId(p.sender_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::BlockAnnounce;
    use crate::peer::PublicKeyBytes;
    use amun_resource_core::ResourceId;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn n66_transport_create_and_add_peer() {
        let identity = PeerIdentity::new(
            NodeId([1u8; 32]),
            PublicKeyBytes([2u8; 32]),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        );
        let transport = TcpTransport::new(
            identity,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        );
        let peer = PeerIdentity::new(
            NodeId([3u8; 32]),
            PublicKeyBytes([4u8; 32]),
            "127.0.0.1:9001".parse().unwrap(),
        );
        transport.add_peer(peer);
    }

    #[test]
    fn n66_message_encode_decode_preserves_content() {
        let msg = NetworkMessage::BlockAnnounce(BlockAnnounce {
            validator_id: ResourceId([9u8; 32]),
            height: 100,
            block_hash: [0xAB; 32],
            state_root: [0xCD; 32],
            parent_hash: [0xEF; 32],
            timestamp: 9999,
        });
        let encoded = msg.encode().unwrap();
        let decoded = NetworkMessage::decode(&encoded).unwrap();
        match decoded {
            NetworkMessage::BlockAnnounce(b) => {
                assert_eq!(b.height, 100);
                assert_eq!(b.timestamp, 9999);
            }
            _ => panic!("Wrong variant"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::message::{BlockAnnounce, CertificateAnnounce, Ping, Pong};
    use crate::peer::NodeId;
    use amun_resource_core::ResourceId;
    use std::net::SocketAddr;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn n66_real_socket_ping_pong() {
        let addr_b: SocketAddr = "127.0.0.1:19002".parse().unwrap();
        let node_a_id = NodeId([0xAA; 32]);
        let node_b_id = NodeId([0xBB; 32]);

        let listener = TcpListener::bind(addr_b).unwrap();
        listener.set_nonblocking(false).unwrap();

        let b_done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let b_done_clone = b_done.clone();

        let b_thread = thread::spawn(move || {
            let (mut stream, _addr) = listener.accept().unwrap();
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).unwrap();
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).unwrap();

            let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
            match msg {
                NetworkMessage::Ping(ping) => {
                    assert_eq!(ping.sender_id, node_a_id.0);
                    let pong = NetworkMessage::Pong(Pong {
                        sender_id: node_b_id.0,
                        sequence: ping.sequence,
                        timestamp: 999,
                        current_height: 42,
                        state_root: [0xCC; 32],
                    });
                    let resp = pong.encode().unwrap();
                    let resp_len = resp.len() as u32;
                    stream.write_all(&resp_len.to_be_bytes()).unwrap();
                    stream.write_all(&resp).unwrap();
                    stream.flush().unwrap();
                }
                _ => panic!("Expected Ping"),
            }
            b_done_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(100));

        let mut stream_a = TcpStream::connect(addr_b).unwrap();
        stream_a.set_nonblocking(false).unwrap();

        let ping = NetworkMessage::Ping(Ping {
            sender_id: node_a_id.0,
            sequence: 1,
            timestamp: 500,
        });
        let data = ping.encode().unwrap();
        let len = data.len() as u32;
        stream_a.write_all(&len.to_be_bytes()).unwrap();
        stream_a.write_all(&data).unwrap();
        stream_a.flush().unwrap();

        let mut len_buf = [0u8; 4];
        stream_a.read_exact(&mut len_buf).unwrap();
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        let mut resp_data = vec![0u8; resp_len];
        stream_a.read_exact(&mut resp_data).unwrap();

        let resp: NetworkMessage = NetworkMessage::decode(&resp_data).unwrap();
        match resp {
            NetworkMessage::Pong(pong) => {
                assert_eq!(pong.sender_id, node_b_id.0);
                assert_eq!(pong.sequence, 1);
                assert_eq!(pong.current_height, 42);
            }
            _ => panic!("Expected Pong"),
        }

        b_thread.join().unwrap();
        assert!(b_done.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn n66_real_socket_block_announce() {
        let addr_b: SocketAddr = "127.0.0.1:19003".parse().unwrap();
        let listener = TcpListener::bind(addr_b).unwrap();
        listener.set_nonblocking(false).unwrap();

        let b_thread = thread::spawn(move || {
            let (mut stream, _addr) = listener.accept().unwrap();
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).unwrap();
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).unwrap();
            let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
            match msg {
                NetworkMessage::BlockAnnounce(block) => {
                    assert_eq!(block.height, 100);
                    assert_eq!(block.block_hash, [0xAB; 32]);
                }
                _ => panic!("Expected BlockAnnounce"),
            }
        });

        thread::sleep(Duration::from_millis(100));

        let mut stream_a = TcpStream::connect(addr_b).unwrap();
        stream_a.set_nonblocking(false).unwrap();

        let block = NetworkMessage::BlockAnnounce(BlockAnnounce {
            validator_id: ResourceId([0x11; 32]),
            height: 100,
            block_hash: [0xAB; 32],
            state_root: [0xCD; 32],
            parent_hash: [0xEF; 32],
            timestamp: 10000,
        });
        let data = block.encode().unwrap();
        let len = data.len() as u32;
        stream_a.write_all(&len.to_be_bytes()).unwrap();
        stream_a.write_all(&data).unwrap();
        stream_a.flush().unwrap();

        b_thread.join().unwrap();
    }

    #[test]
    fn n66_real_socket_certificate_announce() {
        let addr_b: SocketAddr = "127.0.0.1:19004".parse().unwrap();
        let listener = TcpListener::bind(addr_b).unwrap();
        listener.set_nonblocking(false).unwrap();

        let b_thread = thread::spawn(move || {
            let (mut stream, _addr) = listener.accept().unwrap();
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).unwrap();
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).unwrap();
            let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
            match msg {
                NetworkMessage::CertificateAnnounce(cert) => {
                    assert_eq!(cert.height, 10);
                    assert_eq!(cert.quorum_signers.len(), 2);
                }
                _ => panic!("Expected CertificateAnnounce"),
            }
        });

        thread::sleep(Duration::from_millis(100));

        let mut stream_a = TcpStream::connect(addr_b).unwrap();
        stream_a.set_nonblocking(false).unwrap();

        let cert = NetworkMessage::CertificateAnnounce(CertificateAnnounce {
            height: 10,
            block_hash: [0x11; 32],
            state_root: [0x22; 32],
            certificate_hash: [0x33; 32],
            quorum_signers: vec![ResourceId([1u8; 32]), ResourceId([2u8; 32])],
            timestamp: 20000,
        });
        let data = cert.encode().unwrap();
        let len = data.len() as u32;
        stream_a.write_all(&len.to_be_bytes()).unwrap();
        stream_a.write_all(&data).unwrap();
        stream_a.flush().unwrap();

        b_thread.join().unwrap();
    }

    #[test]
    fn n66_real_socket_two_way_exchange() {
        let addr_b: SocketAddr = "127.0.0.1:19005".parse().unwrap();
        let node_a_id = [0xAA; 32];
        let node_b_id = [0xBB; 32];

        let listener = TcpListener::bind(addr_b).unwrap();
        listener.set_nonblocking(false).unwrap();

        let b_thread = thread::spawn(move || {
            let (mut stream, _addr) = listener.accept().unwrap();

            // Receive Ping
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).unwrap();
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).unwrap();
            let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
            assert!(matches!(msg, NetworkMessage::Ping(_)));

            // Send Pong
            let pong = NetworkMessage::Pong(Pong {
                sender_id: node_b_id,
                sequence: 1,
                timestamp: 999,
                current_height: 10,
                state_root: [0xBB; 32],
            });
            let resp = pong.encode().unwrap();
            let resp_len = resp.len() as u32;
            stream.write_all(&resp_len.to_be_bytes()).unwrap();
            stream.write_all(&resp).unwrap();
            stream.flush().unwrap();

            // Receive BlockAnnounce
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).unwrap();
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).unwrap();
            let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
            match msg {
                NetworkMessage::BlockAnnounce(b) => {
                    assert_eq!(b.height, 1);
                    assert_eq!(b.validator_id.0, node_a_id);
                }
                _ => panic!("Expected BlockAnnounce"),
            }
        });

        thread::sleep(Duration::from_millis(100));

        let mut stream_a = TcpStream::connect(addr_b).unwrap();
        stream_a.set_nonblocking(false).unwrap();

        // Send Ping
        let ping = NetworkMessage::Ping(Ping {
            sender_id: node_a_id,
            sequence: 1,
            timestamp: 100,
        });
        let data = ping.encode().unwrap();
        let len = data.len() as u32;
        stream_a.write_all(&len.to_be_bytes()).unwrap();
        stream_a.write_all(&data).unwrap();
        stream_a.flush().unwrap();

        // Receive Pong
        let mut len_buf = [0u8; 4];
        stream_a.read_exact(&mut len_buf).unwrap();
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut data = vec![0u8; len];
        stream_a.read_exact(&mut data).unwrap();
        let msg: NetworkMessage = NetworkMessage::decode(&data).unwrap();
        assert!(matches!(msg, NetworkMessage::Pong(_)));

        // Send BlockAnnounce
        let block = NetworkMessage::BlockAnnounce(BlockAnnounce {
            validator_id: ResourceId(node_a_id),
            height: 1,
            block_hash: [0x01; 32],
            state_root: [0x02; 32],
            parent_hash: [0x00; 32],
            timestamp: 200,
        });
        let data = block.encode().unwrap();
        let len = data.len() as u32;
        stream_a.write_all(&len.to_be_bytes()).unwrap();
        stream_a.write_all(&data).unwrap();
        stream_a.flush().unwrap();

        b_thread.join().unwrap();
    }
}
