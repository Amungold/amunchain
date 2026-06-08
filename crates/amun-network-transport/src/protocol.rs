use std::io::{Read, Write};
use std::net::TcpStream;

pub const MSG_VOTE: u8 = 0x00;
pub const MSG_TIP_REQUEST: u8 = 0x01;
pub const MSG_TIP_RESPONSE: u8 = 0x02;
pub const MSG_BLOCK_RANGE_REQUEST: u8 = 0x03;
pub const MSG_BLOCK_RANGE_RESPONSE: u8 = 0x04;

pub fn send_vote(stream: &mut TcpStream, vote_data: &[u8]) -> Result<(), String> {
    stream.write_all(&[MSG_VOTE]).map_err(|e| format!("write type: {}", e))?;
    let len = vote_data.len() as u32;
    stream.write_all(&len.to_be_bytes()).map_err(|e| format!("write len: {}", e))?;
    stream.write_all(vote_data).map_err(|e| format!("write data: {}", e))?;
    stream.flush().map_err(|e| format!("flush: {}", e))?;
    Ok(())
}

pub fn send_tip_request(stream: &mut TcpStream) -> Result<(u64, [u8; 32]), String> {
    stream.write_all(&[MSG_TIP_REQUEST]).map_err(|e| format!("write: {}", e))?;
    stream.flush().map_err(|e| format!("flush: {}", e))?;
    let mut response_type = [0u8; 1];
    stream.read_exact(&mut response_type).map_err(|e| format!("read type: {}", e))?;
    if response_type[0] != MSG_TIP_RESPONSE {
        return Err(format!("unexpected response type: {}", response_type[0]));
    }
    let mut buf = [0u8; 40];
    stream.read_exact(&mut buf).map_err(|e| format!("read tip: {}", e))?;
    let height = u64::from_be_bytes(buf[0..8].try_into().unwrap());
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&buf[8..40]);
    Ok((height, hash))
}

pub fn send_block_range_request(stream: &mut TcpStream, start: u64, end: u64) -> Result<Vec<u8>, String> {
    let mut req = vec![MSG_BLOCK_RANGE_REQUEST];
    req.extend_from_slice(&start.to_be_bytes());
    req.extend_from_slice(&end.to_be_bytes());
    stream.write_all(&req).map_err(|e| format!("write: {}", e))?;
    stream.flush().map_err(|e| format!("flush: {}", e))?;
    let mut response_type = [0u8; 1];
    stream.read_exact(&mut response_type).map_err(|e| format!("read type: {}", e))?;
    if response_type[0] != MSG_BLOCK_RANGE_RESPONSE {
        return Err(format!("unexpected response type: {}", response_type[0]));
    }
    let mut count_buf = [0u8; 4];
    stream.read_exact(&mut count_buf).map_err(|e| format!("read count: {}", e))?;
    let count = u32::from_be_bytes(count_buf) as usize;
    eprintln!("SYNC_CLIENT: response count={}", count);
    let mut all_data = Vec::new();
    all_data.extend_from_slice(&(count as u32).to_be_bytes());
    for _ in 0..count {
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).map_err(|e| format!("read len: {}", e))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut data = vec![0u8; len];
        stream.read_exact(&mut data).map_err(|e| format!("read data: {}", e))?;
        all_data.extend_from_slice(&len_buf);
        all_data.extend_from_slice(&data);
    }
    Ok(all_data)
}

pub fn handle_incoming(
    stream: &mut TcpStream,
    tip_height: u64,
    tip_hash: [u8; 32],
    load_height: impl Fn(u64) -> Option<Vec<u8>>,
    vote_handler: impl Fn(&[u8]),
) -> Result<(), String> {
    let mut msg_type = [0u8; 1];
    stream.read_exact(&mut msg_type).map_err(|e| format!("read msg type: {}", e))?;
    match msg_type[0] {
        MSG_VOTE => {
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf).map_err(|e| format!("read len: {}", e))?;
            let len = u32::from_be_bytes(len_buf) as usize;
            if len > 1024 * 1024 { return Err("vote too large".into()); }
            let mut data = vec![0u8; len];
            stream.read_exact(&mut data).map_err(|e| format!("read data: {}", e))?;
            vote_handler(&data);
            Ok(())
        }
        MSG_TIP_REQUEST => {
            let mut response = vec![MSG_TIP_RESPONSE];
            response.extend_from_slice(&tip_height.to_be_bytes());
            response.extend_from_slice(&tip_hash);
            stream.write_all(&response).map_err(|e| format!("write tip: {}", e))?;
            stream.flush().map_err(|e| format!("flush: {}", e))?;
            Ok(())
        }
        MSG_BLOCK_RANGE_REQUEST => {
            let mut range_buf = [0u8; 16];
            stream.read_exact(&mut range_buf).map_err(|e| format!("read range: {}", e))?;
            let start = u64::from_be_bytes(range_buf[0..8].try_into().unwrap());
            let end = u64::from_be_bytes(range_buf[8..16].try_into().unwrap());
            let mut records: Vec<Vec<u8>> = Vec::new();
            for h in start..=end {
                if let Some(data) = load_height(h) {
                    records.push(data);
                }
            }
            let mut response = vec![MSG_BLOCK_RANGE_RESPONSE];
            response.extend_from_slice(&(records.len() as u32).to_be_bytes());
            for r in &records {
                response.extend_from_slice(&(r.len() as u32).to_be_bytes());
                response.extend_from_slice(r);
            }
            stream.write_all(&response).map_err(|e| format!("write blocks: {}", e))?;
            stream.flush().map_err(|e| format!("flush: {}", e))?;
            Ok(())
        }
        _ => Err(format!("unknown message type: {}", msg_type[0])),
    }
}
