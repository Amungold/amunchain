use amun_authority_registry::AuthorityRegistry;
use amun_consensus_network::engine::ConsensusEngine;
use amun_networking::validator_certificate::ValidatorCertificate;

pub fn build_registry(
    engine: &mut ConsensusEngine,
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

        engine.register_validator_identity(
            cert.peer_id.0,
            cert.validator_id,
            cert.public_key,
            cert.staking_public_key,
            100,
        );
    }

    Ok(())
}
