//! CausalityChain — a verifiable chain of constitutional causal edges.
//!
//! A causality chain traces WHY a terminal artifact is constitutionally valid
//! by following its causal dependencies back to their roots.
//!
//! INVARIANT: Every edge in the chain must be a constitutional dependency.
//! Non-causal edges (AncestralOnly, InformationalOnly) are NOT part of
//! the causality chain — they belong to the audit trail, not the causal graph.

use crate::causal_edge::CausalEdge;
use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

/// A verifiable chain of constitutional causal dependencies.
///
/// The chain starts from a terminal artifact and follows causal edges
/// backward to the constitutional roots. This answers:
///   "What is the minimal set of artifacts that MUST be valid
///    for this terminal artifact to be constitutionally valid?"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalityChain {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub chain_id: u64,
    pub chain_hash: ConstitutionalHash,

    /// The terminal artifact whose causality is being traced.
    pub terminal_hash: ConstitutionalHash,

    /// The causal edges in this chain, ordered from terminal backward.
    pub edges: Vec<CausalEdge>,

    /// The context this chain belongs to.
    pub context_hash: ConstitutionalHash,
}

impl ConstitutionalIdentity for CausalityChain {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for CausalityChain {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"CAUSALITY_CHAIN")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.chain_id)
            .update_bytes(&self.terminal_hash)
            .update_u64(self.edges.len() as u64)
            .update_bytes(&self.context_hash);
        for edge in &self.edges {
            h.update_bytes(&edge.edge_hash);
        }
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0015 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid causality chain schema",
            ));
        }
        // Verify every edge is a constitutional dependency
        for edge in &self.edges {
            if edge.is_non_causal() {
                return Err(ConstitutionalFailure::new(
                    self.chain_id,
                    failure_type::INVARIANT_BROKEN,
                    failure_domain::CONSTITUTIONAL,
                    severity::HARD_FAILURE,
                    "Causality chain contains non-causal edge",
                ));
            }
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.chain_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Causality chain hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }

    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        // Verify chain continuity: each edge's target should match the next edge's source
        // (or the terminal for the first edge)
        if let Some(first) = self.edges.first() {
            if first.target_hash != self.terminal_hash {
                return Err(ConstitutionalFailure::new(
                    self.chain_id,
                    failure_type::REPLAY_DIVERGENCE,
                    failure_domain::CONSTITUTIONAL,
                    severity::HARD_FAILURE,
                    "Chain does not start from terminal artifact",
                ));
            }
        }
        // Verify consecutive edges link correctly
        for i in 0..self.edges.len().saturating_sub(1) {
            if self.edges[i].source_hash != self.edges[i + 1].target_hash {
                return Err(ConstitutionalFailure::new(
                    self.chain_id,
                    failure_type::REPLAY_DIVERGENCE,
                    failure_domain::CONSTITUTIONAL,
                    severity::HARD_FAILURE,
                    "Chain edge discontinuity",
                ));
            }
        }
        Ok(())
    }
}

impl CausalityChain {
    pub fn new(
        chain_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        terminal_hash: ConstitutionalHash,
        edges: Vec<CausalEdge>,
        context_hash: ConstitutionalHash,
    ) -> Self {
        let mut c = Self {
            schema_id: 0x0015,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            chain_id,
            chain_hash: [0; 32],
            terminal_hash,
            edges,
            context_hash,
        };
        c.chain_hash = c.constitutional_hash();
        c
    }

    /// Returns the number of causal hops from terminal to root.
    pub fn depth(&self) -> usize {
        self.edges.len()
    }

    /// Returns true if the chain starts from the given terminal hash.
    pub fn starts_from(&self, hash: &ConstitutionalHash) -> bool {
        self.terminal_hash == *hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::causal_edge::CausalEdge;

    #[test]
    fn test_empty_chain_verifies() {
        let c = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![], [0xAB; 32]);
        assert!(c.verify().is_ok());
        assert_eq!(c.depth(), 0);
    }
    #[test]
    fn test_single_edge_chain_verifies() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xBB; 32],
            [0xAA; 32],
            crate::CausalityType::StateDerivation,
            [0xAB; 32],
        );
        let c = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![e], [0xAB; 32]);
        assert!(c.verify().is_ok());
        assert_eq!(c.depth(), 1);
    }
    #[test]
    fn test_non_causal_rejected() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xBB; 32],
            [0xAA; 32],
            crate::CausalityType::AncestralOnly,
            [0xAB; 32],
        );
        let c = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![e], [0xAB; 32]);
        assert!(c.verify_structure().is_err());
    }
    #[test]
    fn test_discontinuous_chain_rejected() {
        let e1 = CausalEdge::new(
            1,
            1,
            1,
            [0xBB; 32],
            [0xAA; 32],
            crate::CausalityType::StateDerivation,
            [0xAB; 32],
        );
        let e2 = CausalEdge::new(
            2,
            1,
            1,
            [0xCC; 32],
            [0xDD; 32],
            crate::CausalityType::StateDerivation,
            [0xAB; 32],
        );
        let c = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![e1, e2], [0xAB; 32]);
        assert!(c.verify_constitutional().is_err());
    }
    #[test]
    fn test_hash_deterministic() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xBB; 32],
            [0xAA; 32],
            crate::CausalityType::StateDerivation,
            [0xAB; 32],
        );
        let c1 = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![e.clone()], [0xAB; 32]);
        let c2 = CausalityChain::new(1, 1, 1, [0xAA; 32], vec![e], [0xAB; 32]);
        assert_eq!(c1.chain_hash, c2.chain_hash);
    }
}
