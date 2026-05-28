use super::validator_set::ValidatorSet;
use crate::crypto::types::SignatureBytes;

#[derive(Debug, Clone)]
pub struct AggregateSignature { pub signature: SignatureBytes, pub signers: Vec<u64> }
impl AggregateSignature {
    pub fn new(signature: SignatureBytes, signers: Vec<u64>) -> Self { Self { signature, signers } }
    pub fn verify(&self, vs: &ValidatorSet, _msg: &[u8; 32]) -> bool { vs.verify_quorum(&self.signers) }
    pub fn aggregate(sigs: &[(SignatureBytes, u64)], vs: &ValidatorSet) -> Option<Self> {
        let mut agg = vec![0u8; 64]; let mut signers = Vec::new();
        for (sig, id) in sigs { for (i, b) in sig.as_bytes().iter().enumerate() { if i < 64 { agg[i] ^= b; } } signers.push(*id); }
        if vs.verify_quorum(&signers) { Some(Self::new(SignatureBytes(agg), signers)) } else { None }
    }
}
