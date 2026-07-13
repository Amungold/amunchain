#[cfg(test)]
mod audit_adversarial {
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use rand::Rng;

    // CONST-ADV-001: Random insertion order produces identical roots
    #[test]
    fn adv001_random_order_independence() {
        let mut rng = rand::thread_rng();
        let keys: Vec<Key256> = (0..20)
            .map(|_| {
                let mut k = [0u8; 32];
                rng.fill(&mut k);
                Key256(k)
            })
            .collect();
        let value = [0x42u8; 32];

        let mut tree1 = SparseMerkleTree::empty();
        for k in &keys {
            tree1 = tree1.insert(k, &value, 0);
        }
        let root1 = tree1.root();

        let mut tree2 = SparseMerkleTree::empty();
        for k in keys.iter().rev() {
            tree2 = tree2.insert(k, &value, 0);
        }
        let root2 = tree2.root();

        assert_eq!(
            root1.0, root2.0,
            "CONST-ADV-001 VIOLATION: Order independence broken"
        );
    }

    // CONST-ADV-002: Malformed proof must not verify
    #[test]
    fn adv002_malformed_proof_rejection() {
        let key = Key256([0x77u8; 32]);
        let value = [0x88u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
        let root = tree.root();
        let proof = tree
            .generate_inclusion_proof(&key)
            .expect("CONST-ADV-002: Must generate proof");
        assert!(
            proof.verify(root.0),
            "CONST-ADV-002: Valid proof must verify"
        );
        let wrong_root = [0xFFu8; 32];
        assert!(
            !proof.verify(wrong_root),
            "CONST-ADV-002 VIOLATION: Proof verified against wrong root"
        );
    }

    // CONST-ADV-003: Delete nonexistent key is no-op
    #[test]
    fn adv003_delete_nonexistent_noop() {
        let key1 = Key256([0x11u8; 32]);
        let key2 = Key256([0x22u8; 32]);
        let value = [0x99u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key1, &value, 0);
        let root_before = tree.root().0;
        let tree_after = tree.delete(&key2);
        assert_eq!(
            tree_after.root().0,
            root_before,
            "CONST-ADV-003 VIOLATION: Delete nonexistent changed root"
        );
    }

    // CONST-ADV-004: Insert-delete-insert cycle preserves root
    #[test]
    fn adv004_insert_delete_insert_cycle() {
        let key = Key256([0x33u8; 32]);
        let value = [0x44u8; 32];
        let fresh = SparseMerkleTree::empty().insert(&key, &value, 0);
        let cycled = SparseMerkleTree::empty()
            .insert(&key, &value, 0)
            .delete(&key)
            .insert(&key, &value, 0);
        assert_eq!(
            fresh.root().0,
            cycled.root().0,
            "CONST-ADV-004 VIOLATION: Insert-delete-insert cycle changed root"
        );
    }
}
