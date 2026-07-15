use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::messages::ConsensusVote;
use amun_validator_identity::{derive_validator_id, vote_signing_payload};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ed25519_dalek::{Signer, SigningKey};

fn make_vote(voter: u8, height: u64, block_hash: [u8; 32]) -> ConsensusVote {
    let seed = [voter; 32];
    let sk = SigningKey::from_bytes(&seed);
    let pk = sk.verifying_key().to_bytes();
    let validator_id = derive_validator_id(&pk);

    let state_root = [0xBB; 32];
    let approve = true;
    let timestamp = 1000u64;

    let payload = vote_signing_payload(
        &validator_id,
        amun_validator_identity::signature::DEFAULT_CHAIN_ID,
        height,
        0, // round
        &block_hash,
    );

    let sig = sk.sign(&payload);

    ConsensusVote {
        voter_id: validator_id,
        height,
        block_hash,
        state_root,
        approve,
        signature: sig.to_bytes(),
        timestamp,
        commitment: None,
    }
}

fn register(engine: &mut ConsensusEngine) {
    for id in 1u8..=4 {
        let seed = [id; 32];
        let sk = SigningKey::from_bytes(&seed);
        let pk = sk.verifying_key().to_bytes();
        let validator_id = derive_validator_id(&pk);

        engine.register_validator_identity(validator_id, validator_id, pk, 100);
    }
}

fn bench_single_round(c: &mut Criterion) {
    c.bench_function("consensus_single_round_4_validators", |b| {
        b.iter(|| {
            let mut engine = ConsensusEngine::new([0u8; 32], 4);

            register(&mut engine);

            engine.start_round(1, [1u8; 32]);
            engine.round_mut(1).unwrap().propose([0xAA; 32], [0xBB; 32]);

            for id in 1..=3 {
                engine.process_vote(&make_vote(id, 1, [0xAA; 32])).unwrap();
            }

            engine.try_advance(1, [0xCC; 32]).unwrap();

            black_box(engine.current_height);
        })
    });
}

fn bench_multi_round_10(c: &mut Criterion) {
    c.bench_function("consensus_10_rounds_4_validators", |b| {
        b.iter(|| {
            let mut engine = ConsensusEngine::new([0u8; 32], 4);

            register(&mut engine);

            for h in 1..=10 {
                engine.start_round(h, [(h as u8 % 4) + 1; 32]);

                engine
                    .round_mut(h)
                    .unwrap()
                    .propose([h as u8; 32], [0xBB; 32]);

                for id in 1..=3 {
                    engine
                        .process_vote(&make_vote(id, h, [h as u8; 32]))
                        .unwrap();
                }

                engine.try_advance(h, [h as u8; 32]).unwrap();
            }

            black_box(engine.current_height);
        })
    });
}

fn bench_vote_serialization(c: &mut Criterion) {
    let vote = make_vote(1, 42, [0xAA; 32]);

    c.bench_function("vote_serialize_deserialize", |b| {
        b.iter(|| {
            let e = postcard::to_stdvec(&vote).unwrap();
            let d: ConsensusVote = postcard::from_bytes(&e).unwrap();
            black_box(d);
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
