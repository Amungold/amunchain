// N118.1 — Finality Gate for Slashing Certificates
// =================================================
// Ensures that a MultiSignerCertificate can only be executed
// after the block containing it has been finalized.
//
// Gate logic:
//   certificate.executed_at_height <= finalized_height → ACCEPT
//   certificate.executed_at_height > finalized_height  → REJECT

use crate::multi_signer_certificate::MultiSignerCertificate;

/// N118.1: Check if a certificate can be executed at the given finalized height.
pub fn is_certificate_finalized(cert: &MultiSignerCertificate, finalized_height: u64) -> bool {
    cert.certificate.executed_at_height <= finalized_height
}

/// N118.1: Execute a certificate only if it has been finalized.
/// Returns Err if the certificate's height exceeds the finalized height.
pub fn execute_if_finalized<F, T>(
    cert: &MultiSignerCertificate,
    finalized_height: u64,
    execute: F,
) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String>,
{
    if !is_certificate_finalized(cert, finalized_height) {
        return Err(format!(
            "N118.1: certificate not finalized (cert_height={}, finalized={})",
            cert.certificate.executed_at_height, finalized_height
        ));
    }
    execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
    use crate::ValidatorStatus;

    fn make_certificate(height: u64) -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            [0x42; 32],
            30,
            vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
            vec![EvidenceCount {
                evidence_type: EvidenceType::DoubleVote,
                count: 3,
                weight: 30,
            }],
            1500,
            15000,
            85000,
            3,
            ValidatorStatus::SlashEligible,
            height,
        )
    }

    fn make_multi_signer(height: u64) -> MultiSignerCertificate {
        MultiSignerCertificate::new(make_certificate(height), 3, 5)
    }

    #[test]
    fn n118_1_finalized_certificate_accepted() {
        let cert = make_multi_signer(100);
        assert!(is_certificate_finalized(&cert, 100));
        assert!(is_certificate_finalized(&cert, 150));

        let result = execute_if_finalized(&cert, 100, || Ok("executed"));
        assert_eq!(result.unwrap(), "executed");
    }

    #[test]
    fn n118_1_unfinalized_certificate_rejected() {
        let cert = make_multi_signer(100);
        assert!(!is_certificate_finalized(&cert, 99));

        let result = execute_if_finalized(&cert, 99, || Ok("executed"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not finalized"));
    }

    #[test]
    fn n118_1_exact_height_boundary() {
        let cert = make_multi_signer(100);
        assert!(
            is_certificate_finalized(&cert, 100),
            "height 100 == finalized 100 must be accepted"
        );
        assert!(
            !is_certificate_finalized(&cert, 99),
            "height 100 > finalized 99 must be rejected"
        );
    }
}
