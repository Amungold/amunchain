use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_sync::catch_up::{append_missing_records, download_missing_records};

/// SyncRuntime handles catching up to the chain tip from peers.
///
/// ADR-023 Phase 2: Extracted from LiveValidator::start() consensus loop.
/// Owns no threads; runs synchronously within the consensus loop when triggered.
pub struct SyncRuntime {
    engine: Arc<Mutex<ConsensusEngine>>,
    store: Arc<Mutex<ChainStore>>,
    peers: Vec<SocketAddr>,
}

impl SyncRuntime {
    pub fn new(
        engine: Arc<Mutex<ConsensusEngine>>,
        store: Arc<Mutex<ChainStore>>,
        peers: Vec<SocketAddr>,
    ) -> Self {
        Self {
            engine,
            store,
            peers,
        }
    }

    /// Attempt catch-up if the engine signals it is needed.
    /// Returns true if a catch-up was performed (caller should `continue`).
    pub fn catch_up_if_needed(&self) -> bool {
        let needs_sync = {
            let eng = self.engine.lock().unwrap();
            eng.needs_catchup.load(std::sync::atomic::Ordering::SeqCst)
        };

        if !needs_sync {
            return false;
        }

        // Reset the flag
        self.engine
            .lock()
            .unwrap()
            .needs_catchup
            .store(false, std::sync::atomic::Ordering::SeqCst);

        let current_h = self.engine.lock().unwrap().current_height;

        match download_missing_records(current_h, &self.peers) {
            Ok(records) => {
                if records.is_empty() {
                    return false;
                }

                let mut store_g = self.store.lock().unwrap();
                match append_missing_records(&mut store_g, current_h, records) {
                    Ok(new_h) if new_h > current_h => {
                        let mut eng = self.engine.lock().unwrap();
                        eng.current_height = new_h;
                        eng.rounds.clear();
                        if let Some(tip) = store_g.load_tip() {
                            eng.history_root = tip.history_root;
                        }
                        eprintln!("SYNC: catchup from {} to {}", current_h, new_h);
                        true
                    }
                    Ok(_) => false,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}
