#[cfg(test)]
mod tests {
    use amun_storage_kernel::SparseMerkleTree;

    #[test]
    fn print_real_empty_root() {
        let tree = SparseMerkleTree::empty();
        let root = tree.root().0;
        println!("REAL_CANONICAL_EMPTY_ROOT: {:?}", root);
        // Verify it's NOT all zeros (sanity check)
        assert_ne!(root, [0u8; 32], "Empty root must not be all zeros");
    }
}
