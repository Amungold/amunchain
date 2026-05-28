//! ArtifactGraph — integrity verification substrate.
//!
//! This is NOT a graph traversal engine.
//! This is NOT a graph query layer.
//! This is NOT a graph storage system.
//!
//! This module provides EDGE VERIFICATION only:
//! given two artifact hashes, can we verify that the edge
//! between them is constitutionally valid?
//!
//! DESIGN: Every edge is independently verifiable.
//! No global state, no runtime environment, no storage lookups.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactEdgeType {
    EvidenceToCommitment,
    CommitmentToReceipt,
    ReceiptToReceipt,
    ContextToBoundary,
    JournalToEvidence,
    BoundaryToReceipt,
    CertificateToReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeVerification {
    Valid,
    HashMismatch {
        expected: ConstitutionalHash,
        found: ConstitutionalHash,
    },
    ContextMismatch {
        source_context: ConstitutionalHash,
        target_context: ConstitutionalHash,
    },
    RevisionMismatch {
        source_revision: u32,
        target_revision: u32,
    },
    InvalidEdgeType,
}

impl EdgeVerification {
    pub fn is_valid(&self) -> bool {
        matches!(self, EdgeVerification::Valid)
    }
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEdge {
    pub edge_type: ArtifactEdgeType,
    pub source_hash: ConstitutionalHash,
    pub target_hash: ConstitutionalHash,
    pub verification: EdgeVerification,
}

impl ArtifactEdge {
    pub fn new(
        edge_type: ArtifactEdgeType,
        source_hash: ConstitutionalHash,
        target_hash: ConstitutionalHash,
        verification: EdgeVerification,
    ) -> Self {
        Self {
            edge_type,
            source_hash,
            target_hash,
            verification,
        }
    }

    pub fn valid(
        edge_type: ArtifactEdgeType,
        source_hash: ConstitutionalHash,
        target_hash: ConstitutionalHash,
    ) -> Self {
        Self::new(edge_type, source_hash, target_hash, EdgeVerification::Valid)
    }

    pub fn is_valid(&self) -> bool {
        self.verification.is_valid()
    }
    pub fn is_invalid(&self) -> bool {
        self.verification.is_invalid()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArtifactGraph {
    edges: Vec<ArtifactEdge>,
}

impl ArtifactGraph {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add_edge(&mut self, edge: ArtifactEdge) -> Result<(), ConstitutionalFailure> {
        if edge.is_invalid() {
            return Err(ConstitutionalFailure::new(
                0,
                failure_type::INVARIANT_BROKEN,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Cannot add invalid edge to artifact graph",
            ));
        }
        self.edges.push(edge);
        Ok(())
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edges_of_type(&self, edge_type: ArtifactEdgeType) -> Vec<&ArtifactEdge> {
        self.edges
            .iter()
            .filter(|e| e.edge_type == edge_type)
            .collect()
    }

    pub fn verify_all_edges(&self) -> Result<(), ConstitutionalFailure> {
        for edge in &self.edges {
            if edge.is_invalid() {
                return Err(ConstitutionalFailure::new(
                    0,
                    failure_type::INVARIANT_BROKEN,
                    failure_domain::CONSTITUTIONAL,
                    severity::HARD_FAILURE,
                    "Artifact graph contains invalid edge",
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let g = ArtifactGraph::new();
        assert!(g.verify_all_edges().is_ok());
        assert_eq!(g.edge_count(), 0);
    }
    #[test]
    fn test_add_valid() {
        let mut g = ArtifactGraph::new();
        assert!(g
            .add_edge(ArtifactEdge::valid(
                ArtifactEdgeType::EvidenceToCommitment,
                [0xAA; 32],
                [0xBB; 32]
            ))
            .is_ok());
        assert_eq!(g.edge_count(), 1);
    }
    #[test]
    fn test_reject_invalid() {
        let mut g = ArtifactGraph::new();
        let e = ArtifactEdge::new(
            ArtifactEdgeType::CommitmentToReceipt,
            [0xAA; 32],
            [0xBB; 32],
            EdgeVerification::HashMismatch {
                expected: [0xCC; 32],
                found: [0xDD; 32],
            },
        );
        assert!(g.add_edge(e).is_err());
    }
    #[test]
    fn test_filter_by_type() {
        let mut g = ArtifactGraph::new();
        g.add_edge(ArtifactEdge::valid(
            ArtifactEdgeType::EvidenceToCommitment,
            [0x01; 32],
            [0x02; 32],
        ))
        .unwrap();
        g.add_edge(ArtifactEdge::valid(
            ArtifactEdgeType::ReceiptToReceipt,
            [0x03; 32],
            [0x04; 32],
        ))
        .unwrap();
        g.add_edge(ArtifactEdge::valid(
            ArtifactEdgeType::EvidenceToCommitment,
            [0x05; 32],
            [0x06; 32],
        ))
        .unwrap();
        assert_eq!(
            g.edges_of_type(ArtifactEdgeType::EvidenceToCommitment)
                .len(),
            2
        );
        assert_eq!(g.edges_of_type(ArtifactEdgeType::ReceiptToReceipt).len(), 1);
    }
    #[test]
    fn test_edge_types_distinct() {
        let types = [
            ArtifactEdgeType::EvidenceToCommitment,
            ArtifactEdgeType::CommitmentToReceipt,
            ArtifactEdgeType::ReceiptToReceipt,
            ArtifactEdgeType::ContextToBoundary,
            ArtifactEdgeType::JournalToEvidence,
            ArtifactEdgeType::BoundaryToReceipt,
            ArtifactEdgeType::CertificateToReceipt,
        ];
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }
    #[test]
    fn test_edge_is_invalid_delegates() {
        let e = ArtifactEdge::new(
            ArtifactEdgeType::EvidenceToCommitment,
            [0; 32],
            [0; 32],
            EdgeVerification::Valid,
        );
        assert!(e.is_valid());
        assert!(!e.is_invalid());
    }
}
