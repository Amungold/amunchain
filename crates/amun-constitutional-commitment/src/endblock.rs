use crate::commitment::ConstitutionalCommitment;
use crate::economic_snapshot::EconomicSnapshot;
use crate::economic_tree::EconomicTree;
use crate::roots::compute_constitutional_root;
use crate::Hash32;

pub struct EndBlockPipeline;

impl EndBlockPipeline {
    pub fn execute(
        identity_root: Hash32,
        evidence_root: Hash32,
        governance_root: Hash32,
        snapshot: &EconomicSnapshot,
    ) -> Option<ConstitutionalCommitment> {
        let economic_root = EconomicTree::root(snapshot).ok()?;
        let constitutional_root = compute_constitutional_root(
            identity_root,
            evidence_root,
            governance_root,
            economic_root,
        );

        Some(ConstitutionalCommitment {
            version: 1,
            identity_root,
            evidence_root,
            governance_root,
            economic_root,
            constitutional_root,
        })
    }
}
