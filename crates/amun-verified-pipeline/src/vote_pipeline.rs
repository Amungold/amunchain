use amun_consensus_signatures::SignatureDomain;
use amun_signature_engine::SignatureVerifier;
use amun_unsigned_messages::SignedVote;
use amun_validator_registry::ValidatorRegistry;

#[derive(Debug, Clone)]
pub struct RawVote {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ParsedVote {
    pub signed: SignedVote,
}

#[derive(Debug, Clone)]
pub struct VerifiedVote {
    pub signed: SignedVote,
    pub verified_by: u64,
}

#[derive(Debug, Clone)]
pub struct CanonicalVote {
    pub signed: SignedVote,
}

impl RawVote {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Parse using canonical binary codec.
    pub fn parse(&self) -> Option<ParsedVote> {
        let signed = SignedVote::decode(&self.bytes)?;
        Some(ParsedVote { signed })
    }
}

impl ParsedVote {
    pub fn verify(&self, registry: &ValidatorRegistry, chain_id: u64) -> Option<VerifiedVote> {
        let vid = self.signed.validator_id();
        let pk = registry.get(vid)?;
        if !SignatureVerifier::verify(
            pk,
            &self.signed.unsigned.unsigned_hash,
            &self.signed.signature,
            SignatureDomain::Vote,
            chain_id,
        ) {
            return None;
        }
        Some(VerifiedVote {
            signed: self.signed.clone(),
            verified_by: vid,
        })
    }
}

impl VerifiedVote {
    pub fn canonicalize(&self) -> Result<CanonicalVote, &'static str> {
        if !self.signed.verify_unsigned() {
            return Err("unsigned hash mismatch");
        }
        Ok(CanonicalVote {
            signed: self.signed.clone(),
        })
    }
}
