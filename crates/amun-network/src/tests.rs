#[cfg(test)]
use crate::*;
use amun_kernel_types::PublicKey;

fn make_addr() -> heapless::String<128> {
    let mut s = heapless::String::new();
    s.push_str("127.0.0.1:9000").ok();
    s
}

#[test]
fn test_peer_state_transitions() {
    let pk = PublicKey::new([1u8; 48]);
    let addr = make_addr();
    let mut peer = Peer::new(pk, addr);
    assert!(!peer.is_active());
    peer.mark_connected(1000);
    assert!(peer.is_active());
    peer.mark_disconnected();
    assert!(!peer.is_active());
}
#[test]
fn test_discovery_add_peer() {
    let mut d = Discovery::new();
    let pk = PublicKey::new([1u8; 48]);
    let addr = make_addr();
    let peer = Peer::new(pk, addr);
    d.add_peer(peer).expect("test invariant");
    assert_eq!(d.peer_count(), 1);
}
#[test]
fn test_connection_limit() {
    let mut conn = Connection::new();
    let ip = [127, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    for i in 0..8u8 {
        let pk = PublicKey::new([i; 48]);
        let addr = make_addr();
        let peer = Peer::new(pk, addr);
        conn.connect(peer, ip).expect("test invariant");
    }
    let pk = PublicKey::new([9u8; 48]);
    let addr = make_addr();
    let peer = Peer::new(pk, addr);
    assert!(conn.connect(peer, ip).is_err());
}
#[test]
fn test_rate_limiter() {
    let mut rl = RateLimiter::new();
    for _ in 0..100 {
        assert!(rl.allow(0));
    }
    assert!(!rl.allow(0));
    assert!(rl.allow(1001));
}
#[test]
fn test_heartbeat_timeout() {
    let mut hb = Heartbeat::new();
    assert!(!hb.check_timeout(30000, 60000));
    assert!(hb.check_timeout(70000, 60000));
}
#[test]
fn test_framing_encode() {
    let mut frame = Frame::new(MessageType::Heartbeat);
    frame.payload.push(0xAB).ok();
    let encoded = frame.encode();
    assert_eq!(encoded[0], MessageType::Heartbeat as u8);
    assert_eq!(encoded[1], 0xAB);
}
