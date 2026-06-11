use crate::genesis::{Genesis, GenesisTrustAnchor};
use amun_networking::crypto_identity::PeerKeyPair;
use amun_networking::peer_identity::PeerId;
use amun_networking::validator_certificate::ValidatorCertificate;
use std::fs;
use std::path::Path;

/// Load a validator certificate from disk or generate a self-signed one for development.
/// Returns the certificate and optionally a development trust anchor (for self-signed certs).
pub fn load_validator_certificate(
    cert_file: &str,
    keypair: &PeerKeyPair,
    genesis: &Genesis,
) -> Result<(ValidatorCertificate, Option<GenesisTrustAnchor>), String> {
    let path = Path::new(cert_file);
    if path.exists() {
        let cert_json =
            fs::read_to_string(path).map_err(|e| format!("Cannot read certificate: {}", e))?;
        let cert: ValidatorCertificate = serde_json::from_str(&cert_json)
            .map_err(|e| format!("Invalid certificate JSON: {}", e))?;
        println!("Loaded existing certificate from {}", cert_file);
        Ok((cert, None))
    } else {
        // For development: create a self-signed certificate from the first trust anchor
        let anchor = genesis
            .trust_anchors
            .first()
            .ok_or("No trust anchors in genesis")?;
        let _issuer_id = PeerId::from_bytes(
            hex::decode(&anchor.peer_id)
                .map_err(|e| format!("Invalid anchor peer ID: {}", e))?
                .try_into()
                .map_err(|_| "Invalid anchor peer ID length")?,
        );
        let validator_id = keypair.peer_id();
        // Self-signed: the validator is its own issuer
        let cert = ValidatorCertificate::issue(
            validator_id,
            keypair.verifying_key.to_bytes(),
            keypair, // signs as the authority
            0,
            0,
        );
        // Store the validator's public key as a development trust anchor
        let dev_anchor = GenesisTrustAnchor {
            peer_id: hex::encode(keypair.verifying_key.to_bytes()),
            public_key: hex::encode(keypair.verifying_key.to_bytes()),
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create cert directory: {}", e))?;
        }
        let cert_json = serde_json::to_string_pretty(&cert)
            .map_err(|e| format!("Cannot serialize certificate: {}", e))?;
        fs::write(path, &cert_json).map_err(|e| format!("Cannot write certificate: {}", e))?;
        println!(
            "Generated self-signed certificate and saved to {}",
            cert_file
        );
        println!("NOTE: In production, use a certificate signed by a constitutional authority.");
        Ok((cert, Some(dev_anchor)))
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genesis::{GenesisTrustAnchor, GenesisValidator};
    use tempfile::tempdir;

    fn test_genesis() -> Genesis {
        Genesis {
            chain_id: "test".into(),
            timestamp: 0,
            validators: vec![GenesisValidator {
                peer_id: "aa".repeat(32),
                public_key: "bb".repeat(32),
                voting_power: 100,
            }],
            trust_anchors: vec![GenesisTrustAnchor {
                peer_id: "cc".repeat(32),
                public_key: "dd".repeat(32),
            }],
        }
    }

    #[test]
    fn n22_4_load_or_create_certificate() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("validator.crt");
        let keypair = PeerKeyPair::generate();
        let genesis = test_genesis();
        let cert =
            load_validator_certificate(cert_path.to_str().unwrap(), &keypair, &genesis).unwrap();
        assert_eq!(cert.0.validator_id, keypair.peer_id());
    }

    #[test]
    fn n22_4_certificate_persists_across_loads() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("validator.crt");
        let keypair = PeerKeyPair::generate();
        let genesis = test_genesis();
        let cert1 =
            load_validator_certificate(cert_path.to_str().unwrap(), &keypair, &genesis).unwrap();
        let cert2 =
            load_validator_certificate(cert_path.to_str().unwrap(), &keypair, &genesis).unwrap();
        assert_eq!(cert1.0.validator_id, cert2.0.validator_id);
    }
}
