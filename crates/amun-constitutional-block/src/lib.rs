#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::similar_names)]
#![allow(clippy::float_cmp)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::unused_self)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_borrows_for_generic_args)]
pub mod block;
pub mod chain;
pub mod finalizer;

pub use block::{BlockBuilder, ConstitutionalBlock};
pub use chain::Blockchain;

use amun_constitutional_state::{ConstitutionalStateRuntime, ReplayCertificate};

/// Verify that a block's state provenance is cryptographically valid.
///
/// Checks:
/// 1. Block's replay_certificate_root matches the certificate's merkle root.
/// 2. Block's state_root matches the certificate's post_state_root.
///
/// Note: Full replay verification (N6B) will add journal-based validation.
pub fn verify_block_provenance(
    block: &block::ConstitutionalBlock,
    cert: &ReplayCertificate,
) -> Result<(), String> {
    // 1. Verify certificate merkle root matches block commitment
    let computed_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
        std::slice::from_ref(cert),
    ));
    if block.replay_certificate_root != computed_root {
        return Err(format!(
            "Certificate root mismatch: block={} computed={}",
            block.replay_certificate_root, computed_root
        ));
    }

    // 2. Verify state root matches
    let cert_state_root = hex::encode(cert.post_state_root);
    if block.state_root != cert_state_root {
        return Err(format!(
            "State root mismatch: block={} cert={}",
            block.state_root, cert_state_root
        ));
    }

    Ok(())
}

use amun_constitutional_state::StateTransitionRecord;

/// Full replay verification: cryptographic proof that the block's state
/// was produced by the exact journal of transitions claimed.
///
/// This closes the proof-carrying loop:
///   Block → Certificate → Journal → Replay → StateRoot → Accept/Reject
///
/// Delegates to:
///   - verify_block_provenance() for block↔certificate binding
///   - ReplayCertificate::verify() for journal↔state proof
pub fn verify_full_replay(
    block: &block::ConstitutionalBlock,
    cert: &ReplayCertificate,
    records: &[StateTransitionRecord],
) -> Result<(), String> {
    // Step 1: Verify block ↔ certificate commitment
    verify_block_provenance(block, cert)?;

    // Step 2: Verify journal ↔ state proof (single source of truth)
    if !cert.verify(records) {
        return Err(
            "Full replay verification failed: journal does not produce the claimed state".into(),
        );
    }

    Ok(())
}

use amun_constitutional_state::CertificateInclusionProof;

/// Light client verification: validate a block's state root using only
/// the block header, a ReplayCertificate, and an inclusion proof.
///
/// No full state, no journal replay, no full certificate set needed.
///
/// Verification steps:
///   1. Inclusion proof is valid → certificate is in the merkle root
///   2. Certificate hash matches inclusion proof's leaf value
///   3. Merkle root matches block.replay_certificate_root
///   4. Certificate's post_state_root matches block.state_root
pub fn verify_light_client_proof(
    block: &block::ConstitutionalBlock,
    cert: &ReplayCertificate,
    inclusion_proof: &CertificateInclusionProof,
) -> Result<(), String> {
    // Step 1: Verify the inclusion proof is cryptographically valid
    if !inclusion_proof.verify() {
        return Err("Invalid certificate inclusion proof".into());
    }

    // Step 2: Verify certificate hash matches the proof's leaf value
    if inclusion_proof.certificate_hash != cert.certificate_hash() {
        return Err(format!(
            "Certificate hash mismatch: proof={:?} cert={:?}",
            inclusion_proof.certificate_hash,
            cert.certificate_hash()
        ));
    }

    // Step 3: Verify the merkle root matches the block's commitment
    let proof_root = hex::encode(inclusion_proof.root);
    if block.replay_certificate_root != proof_root {
        return Err(format!(
            "Certificate root mismatch: block={} proof={}",
            block.replay_certificate_root, proof_root
        ));
    }

    // Step 4: Verify the certificate's state root matches the block
    let cert_state_root = hex::encode(cert.post_state_root);
    if block.state_root != cert_state_root {
        return Err(format!(
            "State root mismatch: block={} cert={}",
            block.state_root, cert_state_root
        ));
    }

    Ok(())
}
