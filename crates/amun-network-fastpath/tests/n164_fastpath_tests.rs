use amun_network_fastpath::*;

#[test]
fn n164_message_batching_reduces_overhead() {
    let result_batched = benchmark_batching(10_000, 100);
    let result_unbatched = benchmark_batching(10_000, 1);

    println!("Batched (x100): {}ms, {} KB/s", result_batched.duration_ms, result_batched.throughput_kbps as u64);
    println!("Unbatched: {}ms, {} KB/s", result_unbatched.duration_ms, result_unbatched.throughput_kbps as u64);

    assert!(result_batched.batches_created < result_unbatched.batches_created);
    assert!(result_batched.throughput_kbps >= result_unbatched.throughput_kbps * 0.9,
        "Batched throughput should be at least 90% of unbatched");
}

#[test]
fn n164_large_message_throughput() {
    let result = benchmark_batching(100_000, 50);
    println!("Large test: {}ms, {} KB/s, {} messages",
        result.duration_ms, result.throughput_kbps as u64, result.messages_sent);
    assert!(result.messages_sent == 100_000);
    assert!(result.duration_ms < 2000, "Too slow: {}ms", result.duration_ms);
}

#[test]
fn n164_batch_hash_deterministic() {
    let mut batch1 = MessageBatch::new();
    let mut batch2 = MessageBatch::new();

    for i in 0..100 {
        let msg = format!("MSG_{}", i).into_bytes();
        batch1.add_message(msg.clone());
        batch2.add_message(msg);
    }

    let hash1 = batch1.finalize();
    let hash2 = batch2.finalize();
    assert_eq!(hash1, hash2);
}
