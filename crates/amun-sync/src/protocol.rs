pub use amun_network_transport::protocol::{
    handle_incoming, send_block_range_request, send_tip_request, send_vote,
    MSG_BLOCK_RANGE_REQUEST, MSG_BLOCK_RANGE_RESPONSE, MSG_TIP_REQUEST, MSG_TIP_RESPONSE, MSG_VOTE,
};

use amun_chain_store::store::ChainStore;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_incoming_with_store(
    mut stream: TcpStream,
    store: &ChainStore,
    vote_handler: impl Fn(&[u8]),
) {
    let _ = stream.set_nonblocking(false);
    let mut msg_type = [0u8; 1];
    if stream.read_exact(&mut msg_type).is_err() {
        return;
    }
    match msg_type[0] {
        MSG_VOTE => {
            let mut len_buf = [0u8; 4];
            if stream.read_exact(&mut len_buf).is_err() {
                return;
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            if len > 1024 * 1024 {
                return;
            }
            let mut data = vec![0u8; len];
            if stream.read_exact(&mut data).is_err() {
                return;
            }
            vote_handler(&data);
        }
        MSG_TIP_REQUEST => {
            let tip = store.load_tip();
            let mut response = vec![MSG_TIP_RESPONSE];
            let height = tip.map(|r| r.height).unwrap_or(0);
            let hash = tip.map(|r| r.block_hash).unwrap_or([0u8; 32]);
            response.extend_from_slice(&height.to_be_bytes());
            response.extend_from_slice(&hash);
            let _ = stream.write_all(&response);
            let _ = stream.flush();
        }
        MSG_BLOCK_RANGE_REQUEST => {
            let mut range_buf = [0u8; 16];
            if stream.read_exact(&mut range_buf).is_err() {
                return;
            }
            let start = u64::from_be_bytes(range_buf[0..8].try_into().unwrap());
            let end = u64::from_be_bytes(range_buf[8..16].try_into().unwrap());

            eprintln!("SYNC_SERVER: request {}..{}", start, end);
            eprintln!(
                "SYNC_SERVER: store has {} records, highest={}",
                store.len(),
                store.latest_height()
            );

            let records = store.load_height_range(start, end);

            eprintln!("SYNC_SERVER: found {} records in range", records.len());

            let mut response = vec![MSG_BLOCK_RANGE_RESPONSE];
            response.extend_from_slice(&(records.len() as u32).to_be_bytes());
            for r in &records {
                eprintln!(
                    "SYNC_SERVER_RECORD: height={} len={}",
                    r.height,
                    r.encode().len()
                );
                let encoded = r.encode();
                response.extend_from_slice(&(encoded.len() as u32).to_be_bytes());
                response.extend_from_slice(&encoded);
            }
            let _ = stream.write_all(&response);
            let _ = stream.flush();
        }
        _ => {}
    }
}
