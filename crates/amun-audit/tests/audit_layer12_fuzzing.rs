#[cfg(test)]
mod audit_fuzzing {
    use amun_storage_kernel::smt::proof::MerkleProof;
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use rand::Rng;

    // CONST-FUZZ-001: Proof decoder never panics on random bytes
    #[test]
    fn fuzz001_proof_decode_random_bytes() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let len: usize = rng.gen_range(0..1024);
            let data: Vec<u8> = (0..len).map(|_| rng.gen()).collect();
            let _ = MerkleProof::decode(&data);
        }
    }

    // CONST-FUZZ-002: Random insertions never panic
    #[test]
    fn fuzz002_random_key_insertions() {
        let mut rng = rand::thread_rng();
        let mut tree = SparseMerkleTree::empty();
        for i in 0..50 {
            let mut key = [0u8; 32];
            rng.fill(&mut key);
            let mut value = [0u8; 32];
            rng.fill(&mut value);
            tree = tree.insert(&Key256(key), &value, (i % 5) as u64);
        }
        let root = tree.root();
        assert_ne!(
            root.0, [0u8; 32],
            "CONST-FUZZ-002 VIOLATION: Root must not be zero after insertions"
        );
    }

    // CONST-FUZZ-003: Absence proofs are always consistent
    #[test]
    fn fuzz003_absence_proof_consistency() {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        rng.fill(&mut key);
        let mut value = [0u8; 32];
        rng.fill(&mut value);

        let tree = SparseMerkleTree::empty().insert(&Key256(key), &value, 0);
        let root = tree.root();

        let mut random_key = [0u8; 32];
        rng.fill(&mut random_key);
        if random_key != key {
            if let Some(proof) = tree.generate_absence_proof(&Key256(random_key)) {
                assert!(
                    proof.verify(root.0),
                    "CONST-FUZZ-003 VIOLATION: Absence proof must verify against correct root"
                );
            }
        }
    }
}
