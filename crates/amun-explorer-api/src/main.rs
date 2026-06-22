use amun_explorer_api::{serve, AppState};
use amun_consensus_network::engine::ConsensusEngine;
use amun_chain_store::store::ChainStore;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rpc_port: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(9072);
    
    // Connect to the existing validator's data
    let store = ChainStore::open("/tmp/amun-test-validator-0")
        .unwrap_or_else(|_| ChainStore::open("/tmp/amun-fallback").unwrap());
    
    let consensus = ConsensusEngine::new([1u8; 32], 4);
    
    let state = AppState {
        consensus: Arc::new(Mutex::new(consensus)),
        chain_store: Arc::new(Mutex::new(store)),
    };
    
    serve(state, rpc_port).await;
}
