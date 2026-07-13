// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::certificate::ConstitutionalCertificate;
use crate::revocation::RevocationRegistry;
use amun_constitutional_signing::SignedArtifact;

/// A cryptographically verified chain of signed constitutional certificates.
pub struct CertificateChain {
    pub certificates: Vec<SignedArtifact<ConstitutionalCertificate>>,
}

impl CertificateChain {
    pub fn new(root: SignedArtifact<ConstitutionalCertificate>) -> Self {
        Self {
            certificates: vec![root],
        }
    }

    /// Append a signed certificate that chains to the current tail.
    pub fn append(
        &mut self,
        cert: SignedArtifact<ConstitutionalCertificate>,
    ) -> Result<(), String> {
        let tail = &self.certificates.last().ok_or("Empty chain")?.artifact;
        if cert.artifact.lineage_parent_hash != Some(tail.certificate_id.clone()) {
            return Err("Lineage parent mismatch".into());
        }
        self.certificates.push(cert);
        Ok(())
    }

    /// Look up the public verifying key for a given certificate id.
    fn resolve_key(&self, cert_id: &str) -> Option<String> {
        self.certificates
            .iter()
            .find(|s| s.artifact.certificate_id == cert_id)
            .map(|s| s.signature.verifying_key_hex.clone())
    }

    /// Full validation:
    /// - every certificate is signed by its issuer's key
    /// - the root is self-signed
    /// - no certificate is revoked
    /// - child epochs are contained within the parent's epoch
    pub fn validate(&self, revocation: &RevocationRegistry) -> Result<(), String> {
        for i in 0..self.certificates.len() {
            let signed = &self.certificates[i];
            let cert = &signed.artifact;

            // 1. Revocation
            if revocation.is_revoked(&cert.certificate_id) {
                return Err(format!("Certificate {} revoked", cert.certificate_id));
            }

            // 2. Signature & authority
            if i == 0 {
                if cert.issuer != cert.subject {
                    return Err("Root certificate is not self-signed".into());
                }
                signed
                    .verify()
                    .map_err(|e| format!("Root signature invalid: {}", e))?;
            } else {
                let issuer_key = self.resolve_key(&cert.issuer).ok_or_else(|| {
                    format!("Issuer certificate {} not found in chain", cert.issuer)
                })?;
                if signed.signature.verifying_key_hex != issuer_key {
                    return Err(format!(
                        "Certificate {} not signed by issuer {}",
                        cert.certificate_id, cert.issuer
                    ));
                }
                signed
                    .verify()
                    .map_err(|e| format!("Signature invalid on {}: {}", cert.certificate_id, e))?;
            }

            // 3. Epoch containment
            if i > 0 {
                let parent = &self.certificates[i - 1].artifact;
                if cert.epoch_start < parent.epoch_start || cert.epoch_end > parent.epoch_end {
                    return Err(format!(
                        "Epoch of {} exceeds parent epoch of {}",
                        cert.certificate_id, parent.certificate_id
                    ));
                }
            }
        }
        Ok(())
    }
}
