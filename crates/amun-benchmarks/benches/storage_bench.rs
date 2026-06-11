use amun_chain_store::record::FinalizedChainRecord;
use amun_chain_store::store::ChainStore;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn make_record(h: u64) -> FinalizedChainRecord {
    FinalizedChainRecord {
        height: h,
        block_hash: [h as u8; 32],
        state_root: [0xBB; 32],
        history_root: [h as u8; 32],
        certificate_hash: [0xDD; 32],
        timestamp: h * 1000,
    }
}

fn bench_append_100_records(c: &mut Criterion) {
    c.bench_function("chain_store_append_100", |b| {
        b.iter(|| {
            let dir = tempfile::tempdir().unwrap();
            let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
            for h in 0..100 {
                store.append(make_record(h)).unwrap();
            }
            black_box(store.latest_height());
        })
    });
}

fn bench_read_100_records(c: &mut Criterion) {
    let dir = tempfile::tempdir().unwrap();
    let mut store = ChainStore::open(dir.path().to_str().unwrap()).unwrap();
    for h in 0..100 {
        store.append(make_record(h)).unwrap();
    }

    c.bench_function("chain_store_read_100", |b| {
        b.iter(|| {
            for h in 0..100 {
                let record = store.load_height(h);
                black_box(record);
            }
        })
    });
}

fn bench_record_serialization(c: &mut Criterion) {
    let record = make_record(42);
    c.bench_function("record_serialize_deserialize", |b| {
        b.iter(|| {
            let encoded = record.encode();
            let decoded = FinalizedChainRecord::decode(&encoded).unwrap();
            black_box(decoded);
        })
    });
}

criterion_group!(
    benches,
    bench_append_100_records,
    bench_read_100_records,
    bench_record_serialization
);
criterion_main!(benches);
