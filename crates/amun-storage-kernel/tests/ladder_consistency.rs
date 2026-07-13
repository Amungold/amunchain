#[cfg(test)]
mod tests {
    use amun_storage_kernel::smt::node::Node;
    use amun_storage_kernel::SparseMerkleTree;

    #[test]
    fn empty_ladder_matches_node_hash() {
        let tree = SparseMerkleTree::empty();
        for d in (0..256).rev() {
            let expected = tree.empty_ladder[d];
            let actual = Node::Branch {
                left: tree.empty_ladder[d + 1],
                right: tree.empty_ladder[d + 1],
            }
            .hash();
            assert_eq!(
                expected,
                actual,
                "Empty ladder mismatch at depth {}: expected {:?}, got {:?}",
                d,
                &expected.0[..4],
                &actual.0[..4]
            );
        }
    }
}
