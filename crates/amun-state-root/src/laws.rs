pub const STATE_ROOT_LAW: &str =
    "assert_eq!(recomputed_root, committed_root)";
pub const REPLAY_EQUIVALENCE_LAW: &str =
    "assert_eq!(live_execution_hash, replay_hash)";
pub const SNAPSHOT_CONTINUITY_LAW: &str =
    "assert_eq!(snapshot.parent_hash, previous.snapshot_hash)";
pub const CANONICAL_ENCODING_LAW: &str =
    "assert_eq!(encode(node_a), encode(node_b))";
pub const CHAIN_IDENTITY_LAW: &str =
    "assert_eq!(current_identity_root, genesis_identity_root)";
