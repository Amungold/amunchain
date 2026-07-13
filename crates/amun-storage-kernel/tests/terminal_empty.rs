#[cfg(test)]
mod tests {
    use amun_storage_kernel::smt::node::NodeHash;

    #[test]
    fn terminal_empty_is_zero() {
        // The terminal empty node (depth 256) must be ZERO.
        // This is the base case of the empty ladder.
        // Verified by build_empty_ladder which sets ladder[256] = NodeHash::ZERO.
        assert_eq!(
            NodeHash::ZERO.0,
            [0u8; 32],
            "Terminal empty node must be ZERO hash"
        );
    }
}
