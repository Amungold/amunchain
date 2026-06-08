use crate::protocol::send_block_range_request;
use amun_chain_store::record::FinalizedChainRecord;
use std::net::SocketAddr;

pub fn download_block_range(
    peer: SocketAddr,
    start: u64,
    end: u64,
) -> Result<Vec<FinalizedChainRecord>, String> {
    let mut stream =
        std::net::TcpStream::connect(peer).map_err(|e| format!("connect: {}", e))?;
    let raw = send_block_range_request(&mut stream, start, end)?;
    let mut records = Vec::new();
    let mut pos = 4;
    let count = u32::from_be_bytes(raw[0..4].try_into().unwrap()) as usize;
    eprintln!("SYNC_DECODE: processing {} records, raw_len={}", count, raw.len());
    for i in 0..count {
        if pos + 4 > raw.len() {
            eprintln!("SYNC_DECODE ERROR: trunc at record {}, pos={}", i, pos);
            break;
        }
        let len = u32::from_be_bytes(raw[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        if pos + len > raw.len() {
            eprintln!("SYNC_DECODE ERROR: trunc data at record {}, len={}", i, len);
            break;
        }
        match FinalizedChainRecord::decode(&raw[pos..pos+len]) {
            Ok(record) => {
                records.push(record);
            }
            Err(e) => {
                eprintln!("SYNC_DECODE ERROR: record {} len={} err={}", i, len, e);
            }
        }
        pos += len;
    }
    eprintln!("SYNC_DECODE: decoded {} records out of {}", records.len(), count);
    Ok(records)
}
