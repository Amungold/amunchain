use crate::transcript::signing_transcript;
use amun_consensus_signatures::SignatureDomain;
use amun_crypto::Ed25519Signer;

pub struct SignatureVerifier;

impl SignatureVerifier {
    /// Verify a signature against a message hash and validator public key.
    pub fn verify(
        public_key: &[u8; 32],
        message_hash: &[u8; 32],
        signature: &[u8; 64],
        domain: SignatureDomain,
        chain_id: u64,
    ) -> bool {
        let transcript = signing_transcript(domain, chain_id, message_hash);
        Ed25519Signer::verify(
            public_key,
            &transcript,
            signature,
            domain.tag(),
            chain_id,
        ).is_ok()
    }

    /// Sign a message hash with a validator's signing key.
    pub fn sign(
        signer: &Ed25519Signer,
        message_hash: &[u8; 32],
        domain: SignatureDomain,
        chain_id: u64,
    ) -> Result<[u8; 64], &'static str> {
        let transcript = signing_transcript(domain, chain_id, message_hash);
        signer.sign(&transcript, domain.tag(), chain_id)
            .map_err(|_| "signing failed")
    }
}
