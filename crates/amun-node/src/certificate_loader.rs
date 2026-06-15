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
    verify_validator_membership(&cert, genesis)?;
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
/// Verify that the validator's PeerId is listed in the genesis validator set
/// and that the certificate's public key matches the one declared in genesis.
/// Verify that the validator's PeerId is listed in the genesis validator set
/// and that the certificate's public key matches the one declared in genesis.
/// Verify that the validator's PeerId is listed in the genesis validator set
/// and that the certificate's public key matches the one declared in genesis.
pub fn verify_validator_membership(
    cert: &ValidatorCertificate,
    genesis: &Genesis,
) -> Result<(), String> {
    let id_hex = hex::encode(cert.validator_id.0);
    let cert_pk_hex = hex::encode(cert.public_key);
    for v in &genesis.validators {
        if v.peer_id == id_hex {
            // Check public key binding
            if v.public_key != cert_pk_hex {
                return Err(format!(
                    "Validator {} public key mismatch: certificate key does not match genesis",
                    &id_hex[..16]
                ));
            }
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

    fn make_genesis(validator_peer_id_hex: &str, validator_pk_hex: &str, trust_anchor_pk_hex: &str) -> Genesis {
        Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator {
                peer_id: validator_peer_id_hex.to_string(),
                public_key: validator_pk_hex.to_string(),
                voting_power: 100,
            }],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "cc".repeat(64),
                public_key: trust_anchor_pk_hex.to_string(),
            }],
        }
    }

    #[test]
    fn n106_0_missing_certificate_rejected() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("nonexistent.crt");
        let genesis = make_genesis("aa".repeat(64).as_str(), "bb".repeat(64).as_str(), "dd".repeat(64).as_str());
        let result = load_validator_certificate(cert_path.to_str().unwrap(), &genesis);
        assert!(result.is_err());
    }

    #[test]
    fn n106_2_certificate_with_matching_genesis_passes() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("valid.crt");
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();
        let validator_pk = validator.verifying_key.to_bytes();
        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator_pk,
            &authority,
            0, 0,
        );
        std::fs::write(&cert_path, serde_json::to_string_pretty(&cert).unwrap()).unwrap();
        let authority_pk_hex = hex::encode(authority.verifying_key.to_bytes());
        let validator_id_hex = hex::encode(validator.peer_id().0);
        let validator_pk_hex = hex::encode(validator_pk);
        let genesis = make_genesis(&validator_id_hex, &validator_pk_hex, &authority_pk_hex);
        let loaded = load_validator_certificate(cert_path.to_str().unwrap(), &genesis).unwrap();
        assert_eq!(loaded.validator_id, validator.peer_id());
    }

    #[test]
    fn n106_2_public_key_mismatch_rejected() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("mismatch.crt");
        let authority = PeerKeyPair::generate();
        let validator = PeerKeyPair::generate();
        let wrong_pk_hex = "ff".repeat(32);
        let cert = ValidatorCertificate::issue(
            validator.peer_id(),
            validator.verifying_key.to_bytes(),
            &authority,
            0, 0,
        );
        std::fs::write(&cert_path, serde_json::to_string_pretty(&cert).unwrap()).unwrap();
        let authority_pk_hex = hex::encode(authority.verifying_key.to_bytes());
        let validator_id_hex = hex::encode(validator.peer_id().0);
        // genesis declares a different public key for this validator
        let genesis = make_genesis(&validator_id_hex, &wrong_pk_hex, &authority_pk_hex);
        let result = load_validator_certificate(cert_path.to_str().unwrap(), &genesis);
        assert!(result.is_err());
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
        // Even if genesis has the correct (but tampered cert has zeros), verification should fail
        let validator_id_hex = hex::encode(validator.peer_id().0);
        let correct_pk_hex = hex::encode(validator.verifying_key.to_bytes());
        let genesis = make_genesis(&validator_id_hex, &correct_pk_hex, &authority_pk_hex);
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
        // Genesis contains a different validator
        let genesis = make_genesis("dd".repeat(64).as_str(), "ee".repeat(64).as_str(), &authority_pk_hex);
        let result = load_validator_certificate(cert_path.to_str().unwrap(), &genesis);
        assert!(result.is_err());
    }
}
