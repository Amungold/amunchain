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
        let stream = std::net::TcpStream::connect_timeout(
            addr,
            std::time::Duration::from_millis(500),
        );
        if let Ok(mut stream) = stream {
            if let Ok((tip_height, tip_hash)) = send_tip_request(&mut stream) {
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
        }
    }
    best
}
