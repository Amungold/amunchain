use amun_networking::peer_identity::PeerId;
use crate::genesis::Genesis;
// removed unused import
use amun_networking::validator_certificate::ValidatorCertificate;
use std::fs;
use std::path::Path;

/// Load a validator certificate from disk or generate a self-signed one for development.
/// Returns the certificate and optionally a development trust anchor (for self-signed certs).
pub fn load_validator_certificate(
    cert_file: &str,
    genesis: &Genesis,
) -> Result<ValidatorCertificate, String> {
    let path = Path::new(cert_file);
    if !path.exists() {
        return Err(format!("Validator certificate not found: {}", cert_file));
    }
    let cert_json =
        fs::read_to_string(path).map_err(|e| format!("Cannot read certificate: {}", e))?;
    let cert: ValidatorCertificate = serde_json::from_str(&cert_json)
        .map_err(|e| format!("Invalid certificate JSON: {}", e))?;
    println!("Loaded existing certificate from {}", cert_file);
    verify_certificate_against_genesis(&cert, genesis)?;
    verify_validator_membership(&cert.validator_id, genesis)?;
    Ok(cert)
}

/// Verify a validator certificate against the genesis trust anchors.
pub fn verify_certificate_against_genesis(
    cert: &ValidatorCertificate,
    genesis: &Genesis,
) -> Result<(), String> {
    for anchor in &genesis.trust_anchors {
        let anchor_pubkey: [u8; 32] = hex::decode(&anchor.public_key)
            .map_err(|e| format!("Invalid anchor public key: {}", e))?
            .try_into()
            .map_err(|_| "Invalid anchor public key length")?;
        if cert.verify(&anchor_pubkey) {
            println!(
                "Certificate verified against trust anchor: {}",
                &anchor.peer_id[..16]
            );
            return Ok(());
        }
    }
    Err("Certificate not signed by any genesis trust anchor".into())
}

/// Verify that the validator's PeerId is listed in the genesis validator set.
pub fn verify_validator_membership(
    validator_id: &PeerId,
    genesis: &Genesis,
) -> Result<(), String> {
    let id_hex = hex::encode(validator_id.0);
    for v in &genesis.validators {
        if v.peer_id == id_hex {
            return Ok(());
        }
    }
    Err(format!(
        "Validator {} not found in genesis validator set",
        &id_hex[..16]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genesis::{Genesis, GenesisTrustAnchor, GenesisValidator};
    use amun_networking::crypto_identity::PeerKeyPair;
    use tempfile::tempdir;

    fn test_genesis_with_validator(validator_peer_id_hex: String) -> Genesis {
        Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator {
                peer_id: validator_peer_id_hex,
                public_key: "bb".repeat(64),
                voting_power: 100,
            }],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "cc".repeat(64),
                public_key: "dd".repeat(64),
            }],
        }
    }

    #[test]
    fn n106_0_missing_certificate_rejected() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("nonexistent.crt");
        let genesis = Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator {
                peer_id: "aa".repeat(64),
                public_key: "bb".repeat(64),
                voting_power: 100,
            }],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "cc".repeat(64),
                public_key: "dd".repeat(64),
            }],
        };
        let result = load_validator_certificate(
            cert_path.to_str().unwrap(),
            &genesis,
        );
        assert!(result.is_err());
    }

    #[test]
    fn n106_0_certificate_verified_and_validator_member() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("valid.crt");
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();
        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            0, 0,
        );
        std::fs::write(&cert_path, serde_json::to_string_pretty(&cert).unwrap()).unwrap();
        let authority_pk_hex = hex::encode(authority.verifying_key.to_bytes());
        let validator_id_hex = hex::encode(validator.peer_id().0);
        let genesis = test_genesis_with_validator(validator_id_hex.clone());
        // Override trust anchors with the correct authority
        let mut genesis = genesis;
        genesis.trust_anchors = vec![GenesisTrustAnchor {
            peer_id: "aa".repeat(64),
            public_key: authority_pk_hex,
        }];
        let loaded = load_validator_certificate(cert_path.to_str().unwrap(), &genesis).unwrap();
        assert_eq!(loaded.validator_id, validator.peer_id());
    }

    #[test]
    fn n106_0_tampered_certificate_rejected() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("tampered.crt");
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();
        let mut cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            0, 0,
        );
        cert.public_key = [0u8; 32];
        std::fs::write(&cert_path, serde_json::to_string_pretty(&cert).unwrap()).unwrap();
        let authority_pk_hex = hex::encode(authority.verifying_key.to_bytes());
        let validator_id_hex = hex::encode(validator.peer_id().0);
        let genesis = test_genesis_with_validator(validator_id_hex);
        let mut genesis = genesis;
        genesis.trust_anchors = vec![GenesisTrustAnchor {
            peer_id: "bb".repeat(64),
            public_key: authority_pk_hex,
        }];
        let result = load_validator_certificate(cert_path.to_str().unwrap(), &genesis);
        assert!(result.is_err());
    }

    #[test]
    fn n106_1_validator_not_in_genesis_rejected() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("unknown.crt");
        let authority = PeerKeyPair::generate();
        let unknown_validator = PeerKeyPair::generate();
        let cert = ValidatorCertificate::issue(
            unknown_validator.peer_id(),
            unknown_validator.verifying_key.to_bytes(),
            &authority,
            0, 0,
        );
        std::fs::write(&cert_path, serde_json::to_string_pretty(&cert).unwrap()).unwrap();
        let authority_pk_hex = hex::encode(authority.verifying_key.to_bytes());
        // Genesis contains a DIFFERENT validator
        let genesis = Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator {
                peer_id: "cc".repeat(64),
                public_key: "dd".repeat(64),
                voting_power: 100,
            }],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "aa".repeat(64),
                public_key: authority_pk_hex,
            }],
        };
        let result = load_validator_certificate(cert_path.to_str().unwrap(), &genesis);
        assert!(result.is_err());
    }
}
