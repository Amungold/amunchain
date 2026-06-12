use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: handshake <peer_addr> <validator_id> <local_height>");
        std::process::exit(1);
    }
    let peer: SocketAddr = args[1].parse().expect("Invalid peer address");
    let validator_id: u8 = args[2].parse().unwrap_or(0);
    let local_height: u64 = args[3].parse().unwrap_or(0);

    let mut stream = TcpStream::connect_timeout(&peer, Duration::from_secs(5))
        .expect("Failed to connect to peer");
    stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    eprintln!(
        "HANDSHAKE: validator_id={}, local_height={}",
        validator_id, local_height
    );
    // Send HELLO message with length-prefixed framing (same as vote protocol)
    let request = b"HELLO".to_vec();
    let req_len = (request.len() as u32).to_be_bytes();
    stream.write_all(&req_len).expect("write len");
    stream.write_all(&request).expect("write request");
    stream.flush().expect("flush");

    // Read response
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).expect("read len");
    let resp_len = u32::from_be_bytes(len_buf) as usize;
    let mut resp_data = vec![0u8; resp_len];
    stream.read_exact(&mut resp_data).expect("read data");

    // Response format: "WELCOME" + network_height (8 bytes) + validator_count (1 byte)
    if resp_data.starts_with(b"WELCOME") {
        let network_height = u64::from_le_bytes(resp_data[7..15].try_into().unwrap());
        let validator_count = resp_data[15];
        eprintln!(
            "HANDSHAKE: accepted. network_height={}, validators={}",
            network_height, validator_count
        );
    } else {
        eprintln!(
            "HANDSHAKE: unexpected response: {:?}",
            &resp_data[..7.min(resp_data.len())]
        );
    }
}
