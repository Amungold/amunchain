use crate::block_download::download_block_range;
use crate::peer_discovery::discover_peer_tip;
use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use std::net::SocketAddr;

pub fn download_missing_records(
    current_height: u64,
    peers: &[SocketAddr],
) -> Result<Vec<FinalizedChainRecord>, String> {
    let peer = discover_peer_tip(peers)
        .ok_or_else(|| "No reachable peer for catch-up".to_string())?;

    if peer.tip_height <= current_height {
        return Ok(Vec::new());
    }

    let start = current_height + 1;
    let end = peer.tip_height;

    eprintln!(
        "SYNC: downloading blocks {}..{} from {}",
        start, end, peer.address
    );

    let records = download_block_range(peer.address, start, end)?;
    eprintln!("SYNC: received {} blocks", records.len());
    Ok(records)
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
    eprintln!("SYNC: appended {} blocks, new_height={}", appended, new_height);
    Ok(new_height)
}
