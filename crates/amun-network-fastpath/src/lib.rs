use sha2::{Sha256, Digest};
use std::time::Instant;

pub struct MessageBatch {
    pub messages: Vec<Vec<u8>>,
    pub batch_hash: [u8; 32],
}

impl MessageBatch {
    pub fn new() -> Self {
        Self { messages: Vec::new(), batch_hash: [0u8; 32] }
    }

    pub fn add_message(&mut self, msg: Vec<u8>) {
        self.messages.push(msg);
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for msg in &self.messages {
            hasher.update(msg);
        }
        self.batch_hash = hasher.finalize().into();
        self.batch_hash
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn total_bytes(&self) -> usize {
        self.messages.iter().map(|m| m.len()).sum()
    }
}

pub struct FastPathResult {
    pub messages_sent: u64,
    pub batches_created: u64,
    pub total_bytes: u64,
    pub duration_ms: u64,
    pub throughput_kbps: f64,
}

pub fn benchmark_batching(message_count: u64, batch_size: usize) -> FastPathResult {
    let start = Instant::now();
    let mut total_bytes = 0u64;
    let mut batches_created = 0u64;
    let mut messages_sent = 0u64;
    let mut batch = MessageBatch::new();

    for i in 0..message_count {
        let msg = format!("MSG_{}_DATA_{}", i, "X".repeat(100)).into_bytes();
        total_bytes += msg.len() as u64;
        batch.add_message(msg);
        messages_sent += 1;

        if batch.len() >= batch_size {
            batch.finalize();
            batches_created += 1;
            batch = MessageBatch::new();
        }
    }

    if batch.len() > 0 {
        batch.finalize();
        batches_created += 1;
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;
    let throughput_kbps = if elapsed_ms > 0 {
        (total_bytes as f64 / 1024.0) / (elapsed_ms as f64 / 1000.0)
    } else {
        f64::MAX
    };

    FastPathResult {
        messages_sent,
        batches_created,
        total_bytes,
        duration_ms: elapsed_ms,
        throughput_kbps,
    }
}
