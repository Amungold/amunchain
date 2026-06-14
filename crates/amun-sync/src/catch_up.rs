use crate::block_download::download_block_range;
use crate::peer_discovery::discover_peer_tip;
use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

pub fn download_missing_records(
    current_height: u64,
    peers: &[SocketAddr],
) -> Result<Vec<FinalizedChainRecord>, String> {
    // Retry loop: keep trying until we find a peer that is ahead of us
    let max_retries = 20;
    for attempt in 0..max_retries {
        if let Some(peer) = discover_peer_tip(peers) {
            if peer.tip_height <= current_height {
                // We are already at the tip or ahead
                return Ok(Vec::new());
            }

            let start = current_height + 1;
            let end = peer.tip_height;

            eprintln!(
                "SYNC: downloading blocks {}..{} from {}",
                start, end, peer.address
            );

            match download_block_range(peer.address, start, end) {
                Ok(records) => {
                    eprintln!("SYNC: received {} blocks", records.len());
                    return Ok(records);
                }
                Err(e) => {
                    eprintln!("SYNC: download failed from {}: {}", peer.address, e);
                    // Will retry with next peer or next attempt
                }
            }
        }
        
        eprintln!(
            "SYNC: no suitable peer found (attempt {}/{}), retrying in 2s...",
            attempt + 1,
            max_retries
        );
        thread::sleep(Duration::from_secs(2));
    }

    Err("No reachable peer for catch-up after multiple attempts".to_string())
}

pub fn append_missing_records(
    store: &mut ChainStore,
    current_height: u64,
    records: Vec<FinalizedChainRecord>,
) -> Result<u64, String> {
    let mut appended = 0u64;

    for record in records {
        let h = record.height;
        if h <= current_height {
            continue;
        }
        store
            .append(record)
            .map_err(|e| format!("store append at height {}: {}", h, e))?;
        appended += 1;
    }

    let new_height = store.latest_height();
    eprintln!(
        "SYNC: appended {} blocks, new_height={}",
        appended, new_height
    );
    Ok(new_height)
}
