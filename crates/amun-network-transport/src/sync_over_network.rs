use crate::message::{
    NetworkMessage, StateSyncRequest as NetSyncRequest, StateSyncResponse as NetSyncResponse,
};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
/// Listens on `addr`, and for each connection, reads a StateSyncRequest,
/// generates a StateSyncResponse, and sends it back.
pub struct SyncServer {
    addr: SocketAddr,
}

impl SyncServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Start serving sync requests. For each connection:
    /// 1. Read length-prefixed NetworkMessage
    /// 2. If it's a StateSyncRequest, call the handler
    /// 3. Send back the StateSyncResponse
    pub fn serve<F>(&self, handler: F) -> Result<(), String>
    where
        F: Fn(&NetSyncRequest) -> NetSyncResponse + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(self.addr).map_err(|e| format!("Bind error: {}", e))?;
        listener
            .set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let handler = std::sync::Arc::new(handler);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let h = handler.clone();
                    thread::spawn(move || {
                        Self::handle_connection(&mut stream, &*h);
                    });
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    fn handle_connection<F>(stream: &mut TcpStream, handler: &F)
    where
        F: Fn(&NetSyncRequest) -> NetSyncResponse,
    {
        // Read request
        let request = match Self::read_message(stream) {
            Ok(NetworkMessage::StateSyncRequest(req)) => req,
            Ok(_) => return,
            Err(_) => return,
        };

        // Process
        let response = handler(&request);
        let msg = NetworkMessage::StateSyncResponse(response);

        // Send response
        let _ = Self::send_message(stream, &msg);
    }

    fn read_message(stream: &mut TcpStream) -> Result<NetworkMessage, String> {
        let mut len_buf = [0u8; 4];
        stream
            .read_exact(&mut len_buf)
            .map_err(|e| format!("Read len error: {}", e))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > 16 * 1024 * 1024 {
            return Err("Frame too large".into());
        }
        let mut data = vec![0u8; len];
        stream
            .read_exact(&mut data)
            .map_err(|e| format!("Read data error: {}", e))?;
        NetworkMessage::decode(&data).map_err(|e| format!("Decode error: {}", e))
    }

    fn send_message(stream: &mut TcpStream, msg: &NetworkMessage) -> Result<(), String> {
        let data = msg.encode().map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream
            .write_all(&len.to_be_bytes())
            .map_err(|e| format!("Write len error: {}", e))?;
        stream
            .write_all(&data)
            .map_err(|e| format!("Write data error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }
}

/// Client for requesting state sync from a peer over TCP.
pub struct SyncClient;

impl SyncClient {
    /// Send a StateSyncRequest to a peer and receive the response.
    pub fn request(
        server_addr: SocketAddr,
        request: &NetSyncRequest,
    ) -> Result<NetSyncResponse, String> {
        let mut stream =
            TcpStream::connect(server_addr).map_err(|e| format!("Connect error: {}", e))?;
        stream
            .set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        // Send request
        let msg = NetworkMessage::StateSyncRequest(request.clone());
        let data = msg.encode().map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream
            .write_all(&len.to_be_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        stream
            .write_all(&data)
            .map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;

        // Read response
        let mut len_buf = [0u8; 4];
        stream
            .read_exact(&mut len_buf)
            .map_err(|e| format!("Read len error: {}", e))?;
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        if resp_len > 16 * 1024 * 1024 {
            return Err("Response too large".into());
        }
        let mut data = vec![0u8; resp_len];
        stream
            .read_exact(&mut data)
            .map_err(|e| format!("Read data error: {}", e))?;

        let msg = NetworkMessage::decode(&data).map_err(|e| format!("Decode error: {}", e))?;

        match msg {
            NetworkMessage::StateSyncResponse(resp) => Ok(resp),
            _ => Err("Unexpected response type".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{DeltaSyncData, FullSnapshotData};
    use std::time::Duration;

    /// N67 — Full network state sync: Node B serves a snapshot, Node A requests and receives it.
    #[test]
    fn n67_network_sync_snapshot_request_response() {
        let addr: SocketAddr = "127.0.0.1:19101".parse().unwrap();

        // Start sync server in a thread
        let server_addr = addr;
        let server_ready = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let server_ready_clone = server_ready.clone();

        let _server_thread = thread::spawn(move || {
            let server = SyncServer::new(server_addr);
            server_ready_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            server
                .serve(|_req| {
                    // Return a full snapshot response
                    NetSyncResponse::FullSnapshot(FullSnapshotData {
                        height: 100,
                        block_hash: [0xAA; 32],
                        state_root: [0xBB; 32],
                        history_root: [0xCC; 32],
                        chunks: vec![b"chunk_data_1".to_vec(), b"chunk_data_2".to_vec()],
                        chunk_root: [0xDD; 32],
                        total_resources: 500,
                    })
                })
                .ok();
        });

        // Wait for server to be ready
        while !server_ready.load(std::sync::atomic::Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        // Client sends request
        let request = NetSyncRequest {
            requester_id: [9u8; 32],
            current_height: 0,
            current_state_root: [0u8; 32],
            target_height: 100,
        };

        let response = SyncClient::request(addr, &request).unwrap();

        match response {
            NetSyncResponse::FullSnapshot(snap) => {
                assert_eq!(snap.height, 100);
                assert_eq!(snap.block_hash, [0xAA; 32]);
                assert_eq!(snap.state_root, [0xBB; 32]);
                assert_eq!(snap.history_root, [0xCC; 32]);
                assert_eq!(snap.chunks.len(), 2);
                assert_eq!(snap.total_resources, 500);
            }
            _ => panic!("Expected FullSnapshot"),
        }
    }

    /// N67 — Server returns AlreadySynced when client is up to date.
    #[test]
    fn n67_network_sync_already_synced() {
        let addr: SocketAddr = "127.0.0.1:19102".parse().unwrap();

        let _server_thread = thread::spawn(move || {
            let server = SyncServer::new(addr);
            server
                .serve(|req| {
                    if req.current_height >= 100 {
                        NetSyncResponse::AlreadySynced
                    } else {
                        NetSyncResponse::FullSnapshot(FullSnapshotData {
                            height: 100,
                            block_hash: [0xAA; 32],
                            state_root: [0xBB; 32],
                            history_root: [0xCC; 32],
                            chunks: vec![],
                            chunk_root: [0xDD; 32],
                            total_resources: 0,
                        })
                    }
                })
                .ok();
        });

        thread::sleep(Duration::from_millis(100));

        // Client at height 100 — should get AlreadySynced
        let request = NetSyncRequest {
            requester_id: [9u8; 32],
            current_height: 100,
            current_state_root: [0u8; 32],
            target_height: 100,
        };

        let response = SyncClient::request(addr, &request).unwrap();
        match response {
            NetSyncResponse::AlreadySynced => {} // expected
            _ => panic!("Expected AlreadySynced"),
        }
    }

    /// N67 — Server returns DeltaSync for close heights.
    #[test]
    fn n67_network_sync_delta() {
        let addr: SocketAddr = "127.0.0.1:19103".parse().unwrap();

        let _server_thread = thread::spawn(move || {
            let server = SyncServer::new(addr);
            server
                .serve(|req| {
                    let diff = 100 - req.current_height;
                    if diff <= 10 {
                        NetSyncResponse::DeltaSync(DeltaSyncData {
                            start_height: req.current_height + 1,
                            end_height: 100,
                            blocks: (req.current_height + 1..=100)
                                .map(|h| vec![h as u8; 32])
                                .collect(),
                        })
                    } else {
                        NetSyncResponse::FullSnapshot(FullSnapshotData {
                            height: 100,
                            block_hash: [0xAA; 32],
                            state_root: [0xBB; 32],
                            history_root: [0xCC; 32],
                            chunks: vec![],
                            chunk_root: [0xDD; 32],
                            total_resources: 0,
                        })
                    }
                })
                .ok();
        });

        thread::sleep(Duration::from_millis(100));

        // Client at height 95 — diff=5, should get delta
        let request = NetSyncRequest {
            requester_id: [9u8; 32],
            current_height: 95,
            current_state_root: [0u8; 32],
            target_height: 100,
        };

        let response = SyncClient::request(addr, &request).unwrap();
        match response {
            NetSyncResponse::DeltaSync(delta) => {
                assert_eq!(delta.start_height, 96);
                assert_eq!(delta.end_height, 100);
                assert_eq!(delta.blocks.len(), 5);
            }
            _ => panic!("Expected DeltaSync, got something else"),
        }
    }

    /// N67 — Malformed request gets no crash.
    #[test]
    fn n67_network_sync_malformed_request_no_crash() {
        let addr: SocketAddr = "127.0.0.1:19104".parse().unwrap();

        let _server_thread = thread::spawn(move || {
            let server = SyncServer::new(addr);
            server.serve(|_req| NetSyncResponse::AlreadySynced).ok();
        });

        thread::sleep(Duration::from_millis(100));

        // Send garbage bytes — server should handle gracefully
        let mut stream = TcpStream::connect(addr).unwrap();
        let garbage = vec![0xFFu8; 100];
        let len = garbage.len() as u32;
        stream.write_all(&len.to_be_bytes()).unwrap();
        stream.write_all(&garbage).unwrap();
        stream.flush().unwrap();

        // Server should not crash; this test just verifies no panic
        thread::sleep(Duration::from_millis(50));
    }
}
