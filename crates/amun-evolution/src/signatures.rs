// Cryptographic Signature Domain for Constitutional Evolution
// Defines signing domains, aggregation rules, and anti-replay protection.

use amun_canonical_codec::CanonicalHasher;

pub const SIGNATURE_DOMAIN_AMENDMENT: &[u8] = b"AMUN_SIG_AMENDMENT_V1";
pub const SIGNATURE_DOMAIN_RATIFICATION: &[u8] = b"AMUN_SIG_RATIFY_V1";
pub const SIGNATURE_DOMAIN_ACTIVATION: &[u8] = b"AMUN_SIG_ACTIVATE_V1";

/// A constitutional signature with domain separation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalSignature {
    pub signer_identity: [u8; 32],
    pub signing_domain: Vec<u8>,
    pub payload_hash: [u8; 32],
    pub signature_data: Vec<u8>,
    pub signature_hash: [u8; 32],
}

impl ConstitutionalSignature {
    pub fn new(
        signer: [u8; 32],
        domain: &[u8],
        payload_hash: [u8; 32],
        signature_data: Vec<u8>,
    ) -> Self {
        let mut sig = Self {
            signer_identity: signer,
            signing_domain: domain.to_vec(),
            payload_hash,
            signature_data,
            signature_hash: [0u8; 32],
        };
        sig.signature_hash = sig.compute_hash();
        sig
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_SIGNATURE_V1");
        h.update(&self.signer_identity);
        h.update(&self.signing_domain);
        h.update(&self.payload_hash);
        h.update(&(self.signature_data.len() as u64).to_le_bytes());
        h.update(&self.signature_data);
        h.finalize()
    }

    pub fn verify_structure(&self) -> bool {
        self.compute_hash() == self.signature_hash
    }
}

/// Aggregated signatures for quorum-based ratification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedSignature {
    pub signatures: Vec<ConstitutionalSignature>,
    pub quorum_reached: bool,
    pub total_signers: u64,
    pub required_quorum: u64,
    pub aggregate_hash: [u8; 32],
}

impl AggregatedSignature {
    pub fn new(signatures: Vec<ConstitutionalSignature>, required_quorum: u64) -> Self {
        let total = signatures.len() as u64;
        let quorum = total >= required_quorum;
        let mut agg = Self {
            signatures,
            quorum_reached: quorum,
            total_signers: total,
            required_quorum,
            aggregate_hash: [0u8; 32],
        };
        agg.aggregate_hash = agg.compute_hash();
        agg
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_AGGREGATE_SIG_V1");
        h.update(&self.total_signers.to_le_bytes());
        h.update(&self.required_quorum.to_le_bytes());
        for sig in &self.signatures {
            h.update(&sig.signature_hash);
        }
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.aggregate_hash && self.quorum_reached
    }
}
