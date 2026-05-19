use amun_consensus_signatures::SignatureDomain;

/// Build the canonical signing transcript for a consensus message.
/// Format: domain_tag || chain_id || message_hash
pub fn signing_transcript(
    domain: SignatureDomain,
    chain_id: u64,
    message_hash: &[u8; 32],
) -> Vec<u8> {
    let mut transcript = Vec::with_capacity(128);
    transcript.extend_from_slice(domain.tag());
    transcript.extend_from_slice(&chain_id.to_le_bytes());
    transcript.extend_from_slice(message_hash);
    transcript
}
