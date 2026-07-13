use crate::genesis::Genesis;
use amun_networking::validator_certificate::ValidatorCertificate;

pub fn load_validator_certificate(
    path: &str,
    genesis: &Genesis,
) -> Result<ValidatorCertificate, String> {
    let cert_json =
        std::fs::read_to_string(path).map_err(|e| format!("Cannot read certificate: {}", e))?;
    let cert: ValidatorCertificate =
        serde_json::from_str(&cert_json).map_err(|e| format!("Invalid certificate JSON: {}", e))?;

    // public_key هو [u8; 32] مباشرة
    let pk_hex = hex::encode(cert.public_key);

    let found = genesis
        .validators
        .iter()
        .any(|v| hex::encode(v.public_key.0) == pk_hex)
        || genesis.trust_anchors.iter().any(|t| t.id == pk_hex);

    if !found {
        return Err("Certificate public key not found in genesis".into());
    }

    Ok(cert)
}
