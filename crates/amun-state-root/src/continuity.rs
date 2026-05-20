use super::snapshot::ConstitutionalSnapshot;

pub struct ContinuityChain;

impl ContinuityChain {
    pub fn verify_link(
        parent_hash: &[u8; 32],
        child: &ConstitutionalSnapshot,
    ) -> bool {
        &child.previous_snapshot_hash == parent_hash
    }

    pub fn expected_parent_hash(
        current_snapshot: &ConstitutionalSnapshot,
    ) -> [u8; 32] {
        current_snapshot.seal_hash()
    }
}
