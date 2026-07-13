// Constitutional Hash Computation
// Binds every snapshot to the specific version of constitutional laws
// under which it was created. Two nodes with different constitutions
// are considered different civilizations.

use amun_canonical_codec::CanonicalHasher;
use amun_canonical_codec::PROTOCOL_DOMAIN_CONSTITUTION;

pub struct ConstitutionalHash;

impl ConstitutionalHash {
    /// Compute the constitutional hash from the actual constitution documents.
    /// This hash binds the snapshot to the specific laws it was created under.
    pub fn compute(
        constitution_text: &str,
        specification_text: &str,
        replay_law_text: &str,
        snapshot_constitution_text: &str,
        validity_hierarchy_text: &str,
        canonical_traversal_law_text: &str,
    ) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_CONSTITUTION);
        h.update(constitution_text.as_bytes());
        h.update(specification_text.as_bytes());
        h.update(replay_law_text.as_bytes());
        h.update(snapshot_constitution_text.as_bytes());
        h.update(validity_hierarchy_text.as_bytes());
        h.update(canonical_traversal_law_text.as_bytes());
        h.finalize()
    }

    /// Verify that a snapshot's constitutional hash matches the current
    /// node's constitutional documents.
    pub fn verify(
        claimed_hash: &[u8; 32],
        constitution_text: &str,
        specification_text: &str,
        replay_law_text: &str,
        snapshot_constitution_text: &str,
        validity_hierarchy_text: &str,
        canonical_traversal_law_text: &str,
    ) -> bool {
        let computed = Self::compute(
            constitution_text,
            specification_text,
            replay_law_text,
            snapshot_constitution_text,
            validity_hierarchy_text,
            canonical_traversal_law_text,
        );
        &computed == claimed_hash
    }
}
