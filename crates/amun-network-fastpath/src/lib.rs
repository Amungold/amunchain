use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct MessageBatch {
    pub messages: Vec<Vec<u8>>,
    pub batch_hash: [u8; 32],
}

impl Default for MessageBatch {
    fn default() -> Self {
        Self::with_capacity(128)
    }
}

impl MessageBatch {
    pub fn new() -> Self {
        Self::with_capacity(128)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            messages: Vec::with_capacity(cap),
            batch_hash: [0; 32],
        }
    }

    #[inline]
    pub fn add_message(&mut self, msg: Vec<u8>) {
        self.messages.push(msg);
    }

    #[inline]
    pub fn finalize(&mut self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for msg in &self.messages {
            hasher.update(msg);
        }
        self.batch_hash = hasher.finalize().into();
        self.batch_hash
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct FastPathResult {
    pub messages_sent: u64,
    pub batches_created: u64,
    pub total_bytes: u64,
    pub duration_ms: u64,
    pub throughput_kbps: f64,
}

pub fn benchmark_batching(message_count: u64, batch_size: usize) -> FastPathResult {
    let payload = vec![b'X'; 100];

    let start = Instant::now();

    let mut total_bytes = 0u64;
    let mut batches_created = 0u64;
    let mut messages_sent = 0u64;

    let mut batch = MessageBatch::with_capacity(batch_size);

    for i in 0..message_count {
        let mut msg = Vec::with_capacity(128);

        msg.extend_from_slice(b"MSG_");
        msg.extend_from_slice(i.to_string().as_bytes());
        msg.extend_from_slice(b"_DATA_");
        msg.extend_from_slice(&payload);

        total_bytes += msg.len() as u64;
        batch.add_message(msg);
        messages_sent += 1;

        if batch.len() == batch_size {
            batch.finalize();
            batches_created += 1;
            batch.messages.clear();
        }
    }

    if !batch.is_empty() {
        batch.finalize();
        batches_created += 1;
    }

    let elapsed = start.elapsed();

    FastPathResult {
        messages_sent,
        batches_created,
        total_bytes,
        duration_ms: elapsed.as_millis() as u64,
        throughput_kbps: (total_bytes as f64 / 1024.0) / elapsed.as_secs_f64(),
    }
}
