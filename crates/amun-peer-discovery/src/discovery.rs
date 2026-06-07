use crate::peer_table::PeerTable;
use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;

/// Discovery protocol messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMessage {
    /// Request peer list from a known peer.
    PeerRequest { requester_id: [u8; 32], requester_addr: SocketAddr },
    /// Response with known peers.
    PeerResponse { peers: Vec<(SocketAddr, u64)> },
    /// Announce self to the network.
    PeerAnnounce { node_id: [u8; 32], address: SocketAddr, chain_height: u64 },
}

/// Discovery server: listens for discovery messages.
pub struct DiscoveryServer {
    table: Arc<Mutex<PeerTable>>,
    addr: SocketAddr,
    own_id: [u8; 32],
}

impl DiscoveryServer {
    pub fn new(table: Arc<Mutex<PeerTable>>, addr: SocketAddr, own_id: [u8; 32]) -> Self {
        Self { table, addr, own_id }
    }

    pub fn serve(&self) -> Result<(), String> {
        let listener = TcpListener::bind(self.addr)
            .map_err(|e| format!("Discovery bind error: {}", e))?;
        listener.set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let table = self.table.clone();
        let own_id = self.own_id;
        thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                let _ = stream.set_nonblocking(false);
                let mut len_buf = [0u8; 4];
                if stream.read_exact(&mut len_buf).is_ok() {
                    let len = u32::from_be_bytes(len_buf) as usize;
                    if len < 1024 * 1024 {
                        let mut buf = vec![0u8; len];
                        if stream.read_exact(&mut buf).is_ok() {
                            if let Ok(msg) = postcard::from_bytes::<DiscoveryMessage>(&buf) {
                                match msg {
                                    DiscoveryMessage::PeerRequest { requester_id, requester_addr } => {
                                        // Don't add self
                                        if requester_id != own_id {
                                            let mut table = table.lock().unwrap();
                                            let _ = table.upsert(requester_id, requester_addr, 0);
                                        }
                                        // Respond with our peer list
                                        let peers = table.lock().unwrap().peer_addresses();
                                        let resp = DiscoveryMessage::PeerResponse { peers };
                                        if let Ok(data) = postcard::to_stdvec(&resp) {
                                            let len = data.len() as u32;
                                            let _ = stream.write_all(&len.to_be_bytes());
                                            let _ = stream.write_all(&data);
                                            let _ = stream.flush();
                                        }
                                    }
                                    DiscoveryMessage::PeerResponse { peers } => {
                                        let mut table = table.lock().unwrap();
                                        for (addr, height) in peers {
                                            let id = addr_to_id(&addr);
                                            if id != own_id {
                                                let _ = table.upsert(id, addr, height);
                                            }
                                        }
                                    }
                                    DiscoveryMessage::PeerAnnounce { node_id, address, chain_height } => {
                                        if node_id != own_id {
                                            let mut table = table.lock().unwrap();
                                            let _ = table.upsert(node_id, address, chain_height);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}

/// Discovery client: sends discovery requests to peers.
pub struct DiscoveryClient;

impl DiscoveryClient {
    /// Request peers from a known bootstrap peer.
    pub fn request_peers(
        peer_addr: SocketAddr,
        requester_id: [u8; 32],
        requester_addr: SocketAddr,
    ) -> Result<Vec<(SocketAddr, u64)>, String> {
        let mut stream = TcpStream::connect(peer_addr)
            .map_err(|e| format!("Discovery connect error: {}", e))?;
        stream.set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let req = DiscoveryMessage::PeerRequest {
            requester_id,
            requester_addr,
        };
        let data = postcard::to_stdvec(&req).map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream.write_all(&len.to_be_bytes()).map_err(|e| format!("Write error: {}", e))?;
        stream.write_all(&data).map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;

        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).map_err(|e| format!("Read len error: {}", e))?;
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        if resp_len > 1024 * 1024 {
            return Err("Response too large".into());
        }
        let mut buf = vec![0u8; resp_len];
        stream.read_exact(&mut buf).map_err(|e| format!("Read data error: {}", e))?;

        match postcard::from_bytes::<DiscoveryMessage>(&buf)
            .map_err(|e| format!("Decode error: {}", e))?
        {
            DiscoveryMessage::PeerResponse { peers } => Ok(peers),
            _ => Err("Unexpected response".into()),
        }
    }

    /// Announce self to a peer.
    pub fn announce(
        peer_addr: SocketAddr,
        node_id: [u8; 32],
        address: SocketAddr,
        chain_height: u64,
    ) -> Result<(), String> {
        let mut stream = TcpStream::connect(peer_addr)
            .map_err(|e| format!("Announce connect error: {}", e))?;
        stream.set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let msg = DiscoveryMessage::PeerAnnounce {
            node_id,
            address,
            chain_height,
        };
        let data = postcard::to_stdvec(&msg).map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream.write_all(&len.to_be_bytes()).map_err(|e| format!("Write error: {}", e))?;
        stream.write_all(&data).map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }
}

fn addr_to_id(addr: &SocketAddr) -> [u8; 32] {
    let mut id = [0u8; 32];
    let addr_str = addr.to_string();
    let bytes = addr_str.as_bytes();
    let len = bytes.len().min(32);
    id[..len].copy_from_slice(&bytes[..len]);
    id
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;
    use crate::peer_table::PeerTable;

    fn setup_discovery_server(
        port: u16,
        own_id: [u8; 32],
    ) -> (Arc<Mutex<PeerTable>>, SocketAddr) {
        let table = Arc::new(Mutex::new(PeerTable::new(10)));
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        DiscoveryServer::new(table.clone(), addr, own_id).serve().unwrap();
        thread::sleep(Duration::from_millis(100));
        (table, addr)
    }

    fn make_id(id: u8) -> [u8; 32] {
        let mut a = [0u8; 32];
        a[0] = id;
        a
    }

    #[test]
    fn n73_discover_single_peer() {
        let own_id = make_id(1);
        let (table, addr) = setup_discovery_server(10001, own_id);

        // Pre-populate server's table with a known peer
        table.lock().unwrap().upsert(make_id(2), "127.0.0.1:10002".parse().unwrap(), 100).unwrap();

        // Client requests peers
        let requester_id = make_id(3);
        let requester_addr: SocketAddr = "127.0.0.1:10003".parse().unwrap();
        let peers = DiscoveryClient::request_peers(addr, requester_id, requester_addr).unwrap();

        assert!(peers.len() >= 1, "Should discover at least 1 peer");
        // The requester should also be added to the server's table
        assert!(table.lock().unwrap().get(&requester_id).is_some());
    }

    #[test]
    fn n73_self_not_added() {
        let own_id = make_id(1);
        let (table, addr) = setup_discovery_server(10004, own_id);

        // Client with same ID as server should NOT be added
        let requester_id = own_id;
        let requester_addr: SocketAddr = "127.0.0.1:10005".parse().unwrap();
        let _ = DiscoveryClient::request_peers(addr, requester_id, requester_addr).unwrap();

        // Server should not have itself in the table
        let has_self = table.lock().unwrap().get(&own_id).is_some();
        assert!(!has_self, "Server should not add itself to peer table");
    }

    #[test]
    fn n73_merge_peer_lists() {
        // Server A knows B
        let id_a = make_id(1);
        let (table_a, addr_a) = setup_discovery_server(10006, id_a);
        table_a.lock().unwrap().upsert(make_id(2), "127.0.0.1:10007".parse().unwrap(), 100).unwrap();

        // Client C requests from A, gets B
        let id_c = make_id(3);
        let addr_c: SocketAddr = "127.0.0.1:10008".parse().unwrap();
        let peers = DiscoveryClient::request_peers(addr_a, id_c, addr_c).unwrap();

        assert!(peers.iter().any(|(addr, _)| addr.port() == 10007),
            "Should discover peer B from server A");
    }

    #[test]
    fn n73_peer_announce() {
        let own_id = make_id(1);
        let (table, addr) = setup_discovery_server(10009, own_id);

        let new_id = make_id(5);
        let new_addr: SocketAddr = "127.0.0.1:10010".parse().unwrap();
        DiscoveryClient::announce(addr, new_id, new_addr, 50).unwrap();

        thread::sleep(Duration::from_millis(100));
        let peer = table.lock().unwrap().get(&new_id).cloned();
        assert!(peer.is_some());
        assert_eq!(peer.unwrap().chain_height, 50);
    }

    #[test]
    fn n73_duplicate_peer_not_duplicated() {
        let own_id = make_id(1);
        let (table, addr) = setup_discovery_server(10011, own_id);

        let new_id = make_id(6);
        let new_addr: SocketAddr = "127.0.0.1:10012".parse().unwrap();
        DiscoveryClient::announce(addr, new_id, new_addr, 50).unwrap();
        thread::sleep(Duration::from_millis(100));
        assert_eq!(table.lock().unwrap().active_count(), 1);

        // Second announce should update, not duplicate
        DiscoveryClient::announce(addr, new_id, new_addr, 75).unwrap();
        thread::sleep(Duration::from_millis(100));
        assert_eq!(table.lock().unwrap().active_count(), 1);
    }
}
