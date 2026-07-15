use amun_authority_registry::AuthorityRegistry;
use amun_networking::validator_certificate::ValidatorCertificate;
use amun_validator_registry::{ValidatorRegistry, ValidatorRecord, ValidatorId, PeerId, PublicKey};

pub fn build_registry(
    registry: &mut ValidatorRegistry,
    authority: &AuthorityRegistry,
    certificates: &[ValidatorCertificate],
) -> Result<(), Box<dyn std::error::Error>> {
    if certificates.is_empty() {
        return Err("no validator certificates loaded".into());
    }

    for cert in certificates {
        if !authority.verify_certificate_at(cert, 0) {
            return Err(format!(
                "certificate verification failed for validator {:?}",
                &cert.validator_id[..4]
            )
            .into());
        }

        use amun_validator_registry::{AdmissionRequest, AdmissionService};
        let request = AdmissionRequest {
            validator_id: ValidatorId(cert.validator_id),
            peer_id: PeerId(cert.peer_id.0),
            public_key: PublicKey(cert.public_key),
            certificate_hash: [0u8; 32],
            stake: 100,
            voting_power: 100,
            protocol_version: 1,
            identity_version: 1,
        };
        let gates = AdmissionService::mainnet_gates();
        match AdmissionService::admit(&mut registry, request, &gates) {
            amun_validator_registry::AdmissionResult::Admitted { .. } => {}
            amun_validator_registry::AdmissionResult::Rejected { error, .. } => {
                return Err(format!("Validator admission rejected: {}", error).into());
            }
        }
    }

    Ok(())
}
