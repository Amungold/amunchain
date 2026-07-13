use crate::messages::{GossipMessage, Transaction};
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Mempool {
    transactions: HashMap<[u8; 32], Transaction>,
    seen_hashes: VecDeque<[u8; 32]>,
    max_size: usize,
    max_seen: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            seen_hashes: VecDeque::new(),
            max_size,
            max_seen: max_size * 10,
        }
    }

    pub fn insert(&mut self, tx: Transaction) -> Result<bool, String> {
        if !tx.verify_hash() {
            return Err("Invalid transaction hash".into());
        }
        if self.transactions.contains_key(&tx.tx_hash) {
            return Ok(false);
        }
        if self.seen_hashes.contains(&tx.tx_hash) {
            return Ok(false);
        }
        if self.transactions.len() >= self.max_size {
            if let Some(oldest) = self.seen_hashes.pop_front() {
                self.transactions.remove(&oldest);
            }
        }
        let hash = tx.tx_hash;
        self.transactions.insert(hash, tx);
        self.seen_hashes.push_back(hash);
        if self.seen_hashes.len() > self.max_seen {
            self.seen_hashes.pop_front();
        }
        Ok(true)
    }

    pub fn get(&self, tx_hash: &[u8; 32]) -> Option<&Transaction> {
        self.transactions.get(tx_hash)
    }

    pub fn pending(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

pub struct GossipServer {
    mempool: Arc<Mutex<Mempool>>,
    addr: SocketAddr,
}

impl GossipServer {
    pub fn new(mempool: Arc<Mutex<Mempool>>, addr: SocketAddr) -> Self {
        Self { mempool, addr }
    }

    pub fn serve(&self) -> Result<(), String> {
        let listener =
            TcpListener::bind(self.addr).map_err(|e| format!("Gossip bind error: {}", e))?;
        listener
            .set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;

        let mempool = self.mempool.clone();
        thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                let _ = stream.set_nonblocking(false);
                let mut len_buf = [0u8; 4];
                if stream.read_exact(&mut len_buf).is_ok() {
                    let len = u32::from_be_bytes(len_buf) as usize;
                    if len < 1024 * 1024 {
                        let mut buf = vec![0u8; len];
                        if stream.read_exact(&mut buf).is_ok() {
                            if let Ok(msg) = postcard::from_bytes::<GossipMessage>(&buf) {
                                match msg {
                                    GossipMessage::TransactionAnnounce { .. } => {}
                                    GossipMessage::TransactionRequest { tx_hash, .. } => {
                                        let mp = mempool.lock().unwrap();
                                        if let Some(tx) = mp.get(&tx_hash) {
                                            let resp = GossipMessage::TransactionResponse {
                                                transaction: tx.clone(),
                                            };
                                            if let Ok(data) = postcard::to_stdvec(&resp) {
                                                let len = data.len() as u32;
                                                let _ = stream.write_all(&len.to_be_bytes());
                                                let _ = stream.write_all(&data);
                                                let _ = stream.flush();
                                            }
                                        }
                                    }
                                    GossipMessage::TransactionResponse { transaction } => {
                                        let mut mp = mempool.lock().unwrap();
                                        let _ = mp.insert(transaction);
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

pub struct GossipClient;

impl GossipClient {
    pub fn announce(
        peer_addr: SocketAddr,
        tx_hash: [u8; 32],
        sender_id: [u8; 32],
    ) -> Result<(), String> {
        let mut stream =
            TcpStream::connect(peer_addr).map_err(|e| format!("Gossip connect error: {}", e))?;
        stream
            .set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;
        let msg = GossipMessage::TransactionAnnounce { tx_hash, sender_id };
        let data = postcard::to_stdvec(&msg).map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream
            .write_all(&len.to_be_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        stream
            .write_all(&data)
            .map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }

    pub fn request_tx(
        peer_addr: SocketAddr,
        tx_hash: [u8; 32],
        requester_id: [u8; 32],
    ) -> Result<Transaction, String> {
        let mut stream =
            TcpStream::connect(peer_addr).map_err(|e| format!("Request connect error: {}", e))?;
        stream
            .set_nonblocking(false)
            .map_err(|e| format!("Set nonblocking error: {}", e))?;
        let msg = GossipMessage::TransactionRequest {
            tx_hash,
            requester_id,
        };
        let data = postcard::to_stdvec(&msg).map_err(|e| format!("Encode error: {}", e))?;
        let len = data.len() as u32;
        stream
            .write_all(&len.to_be_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        stream
            .write_all(&data)
            .map_err(|e| format!("Write error: {}", e))?;
        stream.flush().map_err(|e| format!("Flush error: {}", e))?;

        let mut len_buf = [0u8; 4];
        stream
            .read_exact(&mut len_buf)
            .map_err(|e| format!("Read len error: {}", e))?;
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; resp_len];
        stream
            .read_exact(&mut buf)
            .map_err(|e| format!("Read data error: {}", e))?;

        match postcard::from_bytes::<GossipMessage>(&buf)
            .map_err(|e| format!("Decode error: {}", e))?
        {
            GossipMessage::TransactionResponse { transaction } => Ok(transaction),
            _ => Err("Unexpected response".into()),
        }
    }

    pub fn broadcast(peers: &[SocketAddr], tx_hash: [u8; 32], sender_id: [u8; 32]) {
        for addr in peers {
            let _ = Self::announce(*addr, tx_hash, sender_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::Transaction;
    use std::time::Duration;

    fn make_tx(sender: u8, nonce: u64) -> Transaction {
        let mut tx = Transaction {
            tx_hash: [0u8; 32],
            sender: [sender; 32],
            recipient: [sender + 1; 32],
            amount: 100,
            nonce,
            signature: [0u8; 64],
            timestamp: 1000,
        };
        tx.tx_hash = tx.compute_hash();
        tx
    }

    fn setup_gossip_server(port: u16) -> (Arc<Mutex<Mempool>>, SocketAddr) {
        let mempool = Arc::new(Mutex::new(Mempool::new(100)));
        let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
        GossipServer::new(mempool.clone(), addr).serve().unwrap();
        thread::sleep(Duration::from_millis(100));
        (mempool, addr)
    }

    #[test]
    fn n74_mempool_insert_and_retrieve() {
        let mut mp = Mempool::new(10);
        let tx = make_tx(1, 0);
        let hash = tx.tx_hash;
        assert!(mp.insert(tx).unwrap());
        assert!(mp.get(&hash).is_some());
        assert_eq!(mp.len(), 1);
    }

    #[test]
    fn n74_mempool_reject_duplicate() {
        let mut mp = Mempool::new(10);
        let tx = make_tx(1, 0);
        assert!(mp.insert(tx.clone()).unwrap());
        assert!(!mp.insert(tx).unwrap());
        assert_eq!(mp.len(), 1);
    }

    #[test]
    fn n74_mempool_reject_tampered() {
        let mut mp = Mempool::new(10);
        let mut tx = make_tx(1, 0);
        tx.amount = 999;
        assert!(mp.insert(tx).is_err());
    }

    #[test]
    fn n74_gossip_announce() {
        let (_, addr) = setup_gossip_server(11001);
        let tx_hash = [0xAA; 32];
        let result = GossipClient::announce(addr, tx_hash, [1u8; 32]);
        assert!(result.is_ok());
    }

    #[test]
    fn n74_gossip_request_and_response() {
        let (mempool, addr) = setup_gossip_server(11002);
        let tx = make_tx(1, 0);
        let hash = tx.tx_hash;
        mempool.lock().unwrap().insert(tx).unwrap();
        let received = GossipClient::request_tx(addr, hash, [2u8; 32]).unwrap();
        assert_eq!(received.tx_hash, hash);
        assert_eq!(received.sender, [1u8; 32]);
    }

    #[test]
    fn n74_gossip_broadcast() {
        let (mempool1, addr1) = setup_gossip_server(11003);
        let (mempool2, _addr2) = setup_gossip_server(11004);
        let tx = make_tx(1, 0);
        let hash = tx.tx_hash;
        mempool1.lock().unwrap().insert(tx).unwrap();
        let received = GossipClient::request_tx(addr1, hash, [3u8; 32]).unwrap();
        mempool2.lock().unwrap().insert(received).unwrap();
        assert_eq!(mempool2.lock().unwrap().len(), 1);
    }

    #[test]
    fn n74_mempool_pending_transactions() {
        let mut mp = Mempool::new(10);
        mp.insert(make_tx(1, 0)).unwrap();
        mp.insert(make_tx(2, 0)).unwrap();
        mp.insert(make_tx(3, 0)).unwrap();
        assert_eq!(mp.pending().len(), 3);
    }
}
