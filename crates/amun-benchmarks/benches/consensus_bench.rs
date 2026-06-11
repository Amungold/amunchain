use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn make_vote(voter: u8, height: u64, block_hash: [u8; 32]) -> ConsensusVote {
    ConsensusVote {
        voter_id: [voter; 32],
        height,
        block_hash,
        state_root: [0xBB; 32],
        approve: true,
        signature: [0u8; 64],
        timestamp: 1000,
    }
}

fn bench_single_round(c: &mut Criterion) {
    c.bench_function("consensus_single_round_4_validators", |b| {
        b.iter(|| {
            let mut engine = ConsensusEngine::new([0u8; 32], 4);
            engine.start_round(1, [1u8; 32]);
            engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

            for id in 1..=3 {
                engine.process_vote(make_vote(id, 1, [0xAA; 32])).unwrap();
            }
            let _cert = engine.try_advance(1, [0xCC; 32]);
            black_box(engine.current_height);
        })
    });
}

fn bench_multi_round_10(c: &mut Criterion) {
    c.bench_function("consensus_10_rounds_4_validators", |b| {
        b.iter(|| {
            let mut engine = ConsensusEngine::new([0u8; 32], 4);
            for height in 1..=10 {
                let proposer = [(height as u8 % 4 + 1); 32];
                engine.start_round(height, proposer);
                engine
                    .round_mut(height)
                    .unwrap()
                    .propose([height as u8; 32], [0xBB; 32]);

                for id in 1..=3 {
                    engine
                        .process_vote(make_vote(id, height, [height as u8; 32]))
                        .unwrap();
                }
                engine.try_advance(height, [height as u8; 32]).unwrap();
            }
            black_box(engine.current_height);
        })
    });
}

fn bench_vote_serialization(c: &mut Criterion) {
    let vote = make_vote(1, 42, [0xAA; 32]);
    c.bench_function("vote_serialize_deserialize", |b| {
        b.iter(|| {
            let encoded = postcard::to_stdvec(&vote).unwrap();
            let decoded: ConsensusVote = postcard::from_bytes(&encoded).unwrap();
            black_box(decoded);
        })
    });
}

criterion_group!(
    benches,
    bench_single_round,
    bench_multi_round_10,
    bench_vote_serialization
);
criterion_main!(benches);
