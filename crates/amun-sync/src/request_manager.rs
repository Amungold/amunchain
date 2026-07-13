use crate::sync_messages::RequestKey;
use amun_networking::frame::NetworkFrame;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct SyncRequestManager {
    pending: Arc<Mutex<HashMap<RequestKey, Sender<NetworkFrame>>>>,
}

impl Default for SyncRequestManager {
    fn default() -> Self {
        Self::new()
    }
}
impl SyncRequestManager {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_request(&self, key: RequestKey) -> Receiver<NetworkFrame> {
        let (tx, rx) = channel();
        let mut pending = self.pending.lock().unwrap();
        pending.insert(key, tx);
        rx
    }

    pub fn complete_request(&self, key: RequestKey, response: NetworkFrame) {
        let mut pending = self.pending.lock().unwrap();
        if let Some(tx) = pending.remove(&key) {
            let _ = tx.send(response);
        }
    }

    pub fn wait_for_response(
        &self,
        rx: Receiver<NetworkFrame>,
        timeout_ms: u64,
    ) -> Option<NetworkFrame> {
        rx.recv_timeout(Duration::from_millis(timeout_ms)).ok()
    }

    pub fn cancel_request(&self, key: RequestKey) {
        let mut pending = self.pending.lock().unwrap();
        pending.remove(&key);
    }
}
