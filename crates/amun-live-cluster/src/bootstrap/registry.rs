use amun_authority_registry::AuthorityRegistry;
use amun_consensus_network::engine::ConsensusEngine;
use amun_networking::validator_certificate::ValidatorCertificate;
use amun_validator_registry::{ValidatorRegistry, ValidatorRecord, ValidatorId, PeerId, PublicKey};

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

        // Register to canonical registry if available
        if let Some(ref mutex) = engine.registry_mut {
            if let Ok(mut reg) = mutex.lock() {
                let record = ValidatorRecord {
                    validator_id: ValidatorId(cert.validator_id),
                    peer_id: PeerId(cert.peer_id.0),
                    public_key: PublicKey(cert.public_key),
                    certificate_hash: [0u8; 32],
                    stake: 100,
                    voting_power: 100,
                    active: true,
                    slash_count: 0,
                    registered_at: 0,
                };
                let _ = reg.register_full(record);
            }
        }

        // Legacy: also register via engine for backward compatibility
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
