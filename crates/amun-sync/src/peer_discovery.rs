use crate::protocol::send_tip_request;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub address: SocketAddr,
    pub tip_height: u64,
    pub tip_hash: [u8; 32],
}

pub fn discover_peer_tip(peers: &[SocketAddr]) -> Option<PeerInfo> {
    let mut best: Option<PeerInfo> = None;

    for addr in peers {
        eprintln!("DISCOVERY trying {}", addr);
        match std::net::TcpStream::connect_timeout(addr, std::time::Duration::from_millis(500)) {
            Ok(mut stream) => {
                eprintln!("DISCOVERY connected {}", addr);
                match send_tip_request(&mut stream) {
                    Ok((tip_height, tip_hash)) => {
                        eprintln!("DISCOVERY tip {} height={}", addr, tip_height);
                        match &best {
                            None => {
                                best = Some(PeerInfo {
                                    address: *addr,
                                    tip_height,
                                    tip_hash,
                                });
                            }
                            Some(current) if tip_height > current.tip_height => {
                                best = Some(PeerInfo {
                                    address: *addr,
                                    tip_height,
                                    tip_hash,
                                });
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("TIP_REQUEST FAILED {} : {:?}", addr, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("CONNECT FAILED {} : {:?}", addr, e);
            }
        }
    }
    best
}
