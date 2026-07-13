// Snapshot Verifier - Constitutional verification of snapshots
// Verifies: manifest integrity, chunk integrity, state root,
// canonical empty root, constitutional hash, and replay equivalence.

use super::chunk::ChunkIndex;
use super::manifest::SnapshotManifest;
use super::snapshot::SnapshotHeader;
use amun_storage_kernel::SparseMerkleTree;

pub struct SnapshotVerifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    Valid,
    Invalid(String),
}

impl SnapshotVerifier {
    /// Verify a snapshot manifest against the constitutional laws
    pub fn verify_manifest(manifest: &SnapshotManifest) -> VerificationResult {
        if !manifest.verify() {
            return VerificationResult::Invalid(
                "Manifest self-hash verification failed".to_string(),
            );
        }
        let canonical_empty = SparseMerkleTree::canonical_empty_root();
        if manifest.canonical_empty_root != canonical_empty {
            return VerificationResult::Invalid(format!(
                "Canonical empty root mismatch: expected {:?}, got {:?}",
                &canonical_empty[..8],
                &manifest.canonical_empty_root[..8]
            ));
        }
        VerificationResult::Valid
    }

    /// Verify a chunk index against its manifest
    pub fn verify_chunks(chunks: &ChunkIndex, manifest: &SnapshotManifest) -> VerificationResult {
        if !chunks.verify() {
            return VerificationResult::Invalid("Chunk index verification failed".to_string());
        }
        if chunks.chunk_root != manifest.chunk_root {
            return VerificationResult::Invalid("Chunk root mismatch".to_string());
        }
        if chunks.chunk_count != manifest.chunk_count {
            return VerificationResult::Invalid("Chunk count mismatch".to_string());
        }
        VerificationResult::Valid
    }

    /// Verify a snapshot header
    pub fn verify_header(header: &SnapshotHeader) -> VerificationResult {
        let canonical_empty = SparseMerkleTree::canonical_empty_root();
        if header.canonical_empty_root != canonical_empty {
            return VerificationResult::Invalid("Header canonical empty root mismatch".to_string());
        }
        VerificationResult::Valid
    }

    /// THEOREM 11: Snapshot Roundtrip
    /// state -> snapshot -> restore -> replay -> same root
    pub fn verify_roundtrip(
        original_root: [u8; 32],
        reconstructed_root: [u8; 32],
    ) -> VerificationResult {
        if original_root == reconstructed_root {
            VerificationResult::Valid
        } else {
            VerificationResult::Invalid(format!(
                "Snapshot roundtrip theorem violated: original {:?} != reconstructed {:?}",
                &original_root[..8],
                &reconstructed_root[..8]
            ))
        }
    }
}
