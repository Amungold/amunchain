use super::types::SignatureBytes;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigResult { Valid, Invalid, Malformed }

pub trait SignatureVerifier {
    fn verify(&self, sig: &SignatureBytes, msg: &[u8; 32], pk: &[u8; 32]) -> SigResult;
    fn aggregate(&self, sigs: &[SignatureBytes], msgs: &[[u8; 32]]) -> Option<SignatureBytes>;
    fn verify_aggregated(&self, agg: &SignatureBytes, msgs: &[[u8; 32]], pks: &[[u8; 32]]) -> SigResult;
}

#[derive(Debug, Clone)]
pub struct PlaceholderVerifier;
impl SignatureVerifier for PlaceholderVerifier {
    fn verify(&self, sig: &SignatureBytes, _msg: &[u8; 32], _pk: &[u8; 32]) -> SigResult {
        if sig.as_bytes().iter().any(|&b| b != 0) { SigResult::Valid } else { SigResult::Invalid }
    }
    fn aggregate(&self, sigs: &[SignatureBytes], _msgs: &[[u8; 32]]) -> Option<SignatureBytes> {
        if sigs.is_empty() { return None; }
        let mut agg = vec![0u8; 64];
        for sig in sigs { for (i, b) in sig.as_bytes().iter().enumerate() { if i < 64 { agg[i] ^= b; } } }
        Some(SignatureBytes(agg))
    }
    fn verify_aggregated(&self, agg: &SignatureBytes, _msgs: &[[u8; 32]], _pks: &[[u8; 32]]) -> SigResult {
        if agg.as_bytes().iter().any(|&b| b != 0) { SigResult::Valid } else { SigResult::Invalid }
    }
}
