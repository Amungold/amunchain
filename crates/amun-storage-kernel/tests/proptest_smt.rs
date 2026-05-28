#[cfg(test)]
mod proptest_smt {
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestRunner};

    fn proptest_config() -> Config {
        Config {
            failure_persistence: None,
            cases: 256,
            ..Config::default()
        }
    }

    #[test]
    fn proof_roundtrip() {
        let mut runner = TestRunner::new(proptest_config());
        runner
            .run(
                &(any::<[u8; 32]>(), any::<[u8; 32]>()),
                |(key_bytes, value_bytes)| {
                    let key = Key256(key_bytes);
                    let value = value_bytes;
                    let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
                    let root = tree.root();
                    let proof = tree
                        .generate_inclusion_proof(&key)
                        .expect("inclusion proof generation failed");
                    prop_assert!(proof.verify(root.0), "proof verification failed");
                    Ok(())
                },
            )
            .unwrap();
    }

    #[test]
    fn absence_proof() {
        let mut runner = TestRunner::new(proptest_config());
        runner
            .run(
                &(any::<[u8; 32]>(), any::<[u8; 32]>(), any::<[u8; 32]>()),
                |(present_key_bytes, absent_key_bytes, value_bytes)| {
                    if present_key_bytes == absent_key_bytes {
                        return Ok(());
                    }
                    let present_key = Key256(present_key_bytes);
                    let absent_key = Key256(absent_key_bytes);
                    let value = value_bytes;
                    let tree = SparseMerkleTree::empty().insert(&present_key, &value, 0);
                    let root = tree.root();
                    let proof = tree
                        .generate_absence_proof(&absent_key)
                        .expect("absence proof generation failed");
                    prop_assert!(proof.verify(root.0), "absence proof verification failed");
                    Ok(())
                },
            )
            .unwrap();
    }

    #[test]
    fn insertion_order_independence() {
        let mut runner = TestRunner::new(proptest_config());
        runner
            .run(
                &(
                    proptest::collection::vec(any::<[u8; 32]>(), 1..10),
                    proptest::collection::vec(any::<[u8; 32]>(), 1..10),
                ),
                |(mut keys, values)| {
                    let n = keys.len().min(values.len());
                    keys.truncate(n);
                    let values = &values[..n];

                    let mut tree_a = SparseMerkleTree::empty();
                    for (k, v) in keys.iter().zip(values.iter()) {
                        tree_a = tree_a.insert(&Key256(*k), v, 0);
                    }
                    let root_a = tree_a.root();

                    let mut tree_b = SparseMerkleTree::empty();
                    for (k, v) in keys.iter().zip(values.iter()).rev() {
                        tree_b = tree_b.insert(&Key256(*k), v, 0);
                    }
                    let root_b = tree_b.root();

                    prop_assert_eq!(
                        root_a.0,
                        root_b.0,
                        "insertion order must not affect the final root"
                    );
                    Ok(())
                },
            )
            .unwrap();
    }
}
