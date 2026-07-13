#[cfg(test)]
mod tests {
    use amun_storage_kernel::smt::constants::CANONICAL_EMPTY_ROOT_V1;
    use amun_storage_kernel::SparseMerkleTree;

    #[test]
    fn canonical_empty_root_matches_runtime() {
        let computed = SparseMerkleTree::canonical_empty_root();
        assert_eq!(
            CANONICAL_EMPTY_ROOT_V1, computed,
            "CANONICAL_EMPTY_ROOT_V1 constant must match runtime empty tree root"
        );
    }
}
