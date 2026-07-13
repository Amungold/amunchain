#[cfg(test)]
mod tests {
    use amun_storage_kernel::{Key256, SparseMerkleTree};

    #[test]
    fn insert_delete_returns_to_canonical_empty() {
        let key = Key256([42u8; 32]);
        let value = [99u8; 32];
        let empty_root = SparseMerkleTree::canonical_empty_root();
        let tree = SparseMerkleTree::empty()
            .insert(&key, &value, 0)
            .delete(&key);
        assert_eq!(
            tree.root().0,
            empty_root,
            "insert+delete must return to canonical empty root"
        );
    }

    #[test]
    fn delete_nonexistent_does_not_change_root() {
        let key1 = Key256([1u8; 32]);
        let key2 = Key256([2u8; 32]);
        let value = [99u8; 32];
        let tree1 = SparseMerkleTree::empty().insert(&key1, &value, 0);
        let root_before = tree1.root().0;
        let tree2 = tree1.delete(&key2);
        assert_eq!(
            tree2.root().0,
            root_before,
            "deleting nonexistent key must not change root"
        );
    }

    #[test]
    fn reinsert_after_delete_produces_same_root() {
        let key = Key256([7u8; 32]);
        let value = [88u8; 32];
        let fresh = SparseMerkleTree::empty().insert(&key, &value, 0);
        let fresh_root = fresh.root().0;
        let deleted_reinserted = SparseMerkleTree::empty()
            .insert(&key, &value, 0)
            .delete(&key)
            .insert(&key, &value, 0);
        assert_eq!(
            deleted_reinserted.root().0,
            fresh_root,
            "delete+reinsert must produce same root as fresh insert"
        );
    }
}
