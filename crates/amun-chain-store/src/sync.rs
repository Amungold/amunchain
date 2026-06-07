use crate::record::FinalizedChainRecord;
use crate::store::ChainStore;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct SyncServer {
    store: Arc<Mutex<ChainStore>>,
    addr: SocketAddr,
}

impl SyncServer {
    pub fn new(store: Arc<Mutex<ChainStore>>, addr: SocketAddr) -> Self {
        Self { store, addr }
    }

    pub fn serve(&self) -> Result<(), String> {
        let listener = TcpListener::bind(self.addr)
            .map_err(|e| format!("Sync bind error: {}", e))?;
        listener.set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let store = self.store.clone();
        thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                let _ = stream.set_nonblocking(false);
                let mut buf = [0u8; 16];
                if stream.read_exact(&mut buf).is_ok() {
                    let start = u64::from_le_bytes(buf[0..8].try_into().unwrap());
                    let end = u64::from_le_bytes(buf[8..16].try_into().unwrap());

                    let store = store.lock().unwrap();
                    let tip = store.latest_height();
                    let end = end.min(tip);
                    let mut records: Vec<FinalizedChainRecord> = Vec::new();
                    for h in start..=end {
                        if let Some(record) = store.load_height(h) {
                            records.push(record.clone());
                        }
                    }
                    drop(store);

                    let encoded: Vec<u8> = records.iter()
                        .flat_map(|r| {
                            let data = r.encode();
                            let len = data.len() as u32;
                            let mut out = Vec::new();
                            out.extend_from_slice(&len.to_le_bytes());
                            out.extend_from_slice(&data);
                            out
                        })
                        .collect();
                    let total = encoded.len() as u32;
                    let _ = stream.write_all(&total.to_le_bytes());
                    let _ = stream.write_all(&encoded);
                    let _ = stream.flush();
                }
            }
        });
        Ok(())
    }
}

pub struct SyncClient;

impl SyncClient {
    pub fn download_range(
        peer_addr: SocketAddr,
        start_height: u64,
        end_height: u64,
    ) -> Result<Vec<FinalizedChainRecord>, String> {
        let mut stream = TcpStream::connect(peer_addr)
            .map_err(|e| format!("Sync connect error: {}", e))?;
        stream.set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let mut req = Vec::new();
        req.extend_from_slice(&start_height.to_le_bytes());
        req.extend_from_slice(&end_height.to_le_bytes());
        stream.write_all(&req).map_err(|e| format!("Sync write error: {}", e))?;
        stream.flush().map_err(|e| format!("Sync flush error: {}", e))?;

        let mut size_buf = [0u8; 4];
        stream.read_exact(&mut size_buf).map_err(|e| format!("Sync read size error: {}", e))?;
        let total = u32::from_le_bytes(size_buf) as usize;
        if total > 16 * 1024 * 1024 {
            return Err("Response too large".into());
        }

        let mut data = vec![0u8; total];
        stream.read_exact(&mut data).map_err(|e| format!("Sync read data error: {}", e))?;

        let mut records = Vec::new();
        let mut offset = 0;
        while offset + 4 <= data.len() {
            let len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + len > data.len() { break; }
            let record = FinalizedChainRecord::decode(&data[offset..offset + len])?;
            records.push(record);
            offset += len;
        }
        Ok(records)
    }

    pub fn catch_up(peer_addr: SocketAddr, local_store: &mut ChainStore) -> Result<u64, String> {
        let all_records = Self::download_range(peer_addr, 0, u64::MAX)?;
        let peer_tip = all_records.last().map(|r| r.height).unwrap_or(0);

        let local_height = local_store.latest_height();
        let start_height = if local_store.is_empty() && local_height == 0 {
            0
        } else {
            local_height + 1
        };

        if start_height > peer_tip {
            return Ok(local_store.latest_height());
        }

        let missing = Self::download_range(peer_addr, start_height, peer_tip)?;
        for record in &missing {
            local_store.append(record.clone()).map_err(|e| format!("Append error: {}", e))?;
        }
        Ok(local_store.latest_height())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::ChainStore;
    use std::time::Duration;

    fn make_record(h: u64) -> FinalizedChainRecord {
        FinalizedChainRecord {
            height: h, block_hash: [h as u8; 32],
            state_root: [0xBB; 32], history_root: [h as u8; 32],
            certificate_hash: [0xDD; 32], timestamp: h * 1000,
        }
    }

    fn setup_server(port: u16, count: u64) -> (Arc<Mutex<ChainStore>>, SocketAddr) {
        let dir = tempfile::tempdir().unwrap();
        let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        for h in 0..count { store.append(make_record(h)).unwrap(); }
        let store = Arc::new(Mutex::new(store));
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        SyncServer::new(store.clone(), addr).serve().unwrap();
        thread::sleep(Duration::from_millis(100));
        (store, addr)
    }

    #[test]
    fn n72_sync_download_range() {
        let (_, addr) = setup_server(9901, 5);
        let records = SyncClient::download_range(addr, 0, 4).unwrap();
        assert_eq!(records.len(), 5);
    }

    #[test]
    fn n72_sync_catch_up() {
        let (_, addr) = setup_server(9902, 10);
        let dir = tempfile::tempdir().unwrap();
        let mut local = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        let h = SyncClient::catch_up(addr, &mut local).unwrap();
        assert_eq!(h, 9);
        assert_eq!(local.len(), 10);
    }

    #[test]
    fn n72_sync_partial_catch_up() {
        let (_, addr) = setup_server(9903, 20);
        let dir = tempfile::tempdir().unwrap();
        let mut local = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        for h in 0..5 { local.append(make_record(h)).unwrap(); }
        let h = SyncClient::catch_up(addr, &mut local).unwrap();
        assert_eq!(h, 19);
        assert_eq!(local.len(), 20);
    }

    #[test]
    fn n72_sync_already_synced() {
        let (_, addr) = setup_server(9904, 5);
        let dir = tempfile::tempdir().unwrap();
        let mut local = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        for h in 0..5 { local.append(make_record(h)).unwrap(); }
        let h = SyncClient::catch_up(addr, &mut local).unwrap();
        assert_eq!(h, 4);
    }

    #[test]
    fn n72_sync_empty_peer() {
        let dir = tempfile::tempdir().unwrap();
        let store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
        let store = Arc::new(Mutex::new(store));
        let addr: SocketAddr = "127.0.0.1:9905".parse().unwrap();
        SyncServer::new(store.clone(), addr).serve().unwrap();
        thread::sleep(Duration::from_millis(100));

        let dir2 = tempfile::tempdir().unwrap();
        let mut local = ChainStore::open(dir2.path().to_str().unwrap()).unwrap();
        let h = SyncClient::catch_up(addr, &mut local).unwrap();
        assert_eq!(h, 0);
    }
}
