use std::fs;
use std::path::Path;

use amun_authority_registry::AuthorityRegistry;
use amun_networking::{
    genesis_authority::genesis_authority_keypair, validator_certificate::ValidatorCertificate,
};

use crate::bootstrap::identity::IdentityContext;
use crate::config::ValidatorConfig;

pub struct CertificateContext {
    pub certificate: ValidatorCertificate,
}

pub fn ensure_certificate(
    config: &ValidatorConfig,
    identity: &IdentityContext,
    registry: &AuthorityRegistry,
) -> Result<CertificateContext, Box<dyn std::error::Error>> {
    println!("CERT PATH = {}", config.certificate_path);
    println!(
        "CERT EXISTS = {}",
        Path::new(&config.certificate_path).exists()
    );
    if Path::new(&config.certificate_path).exists() {
        eprintln!("CERT:A");
        let json = fs::read_to_string(&config.certificate_path)?;
        eprintln!("CERT:B bytes={}", json.len());
        eprintln!("CERT:C");
        match serde_json::from_str::<ValidatorCertificate>(&json) {
            Ok(cert) => {
                eprintln!("CERT:D");
                eprintln!("CERT:E");
                return Ok(CertificateContext { certificate: cert });
            }
            Err(err) => {
                eprintln!("Legacy certificate detected: {}", err);
                std::fs::remove_file(&config.certificate_path).ok();
            }
        }
    }

    let active = registry
        .active()
        .ok_or("No active constitutional authority")?;

    let authority_key = genesis_authority_keypair();

    println!("ISSUE peer_id={:?}", identity.bundle.peer_id);
    println!(
        "ISSUE validator_id_prefix={:?}",
        &identity.bundle.validator_id[..4]
    );
    println!(
        "ISSUE public_key_prefix={:?}",
        &identity.bundle.public_key[..4]
    );

    let cert = ValidatorCertificate::issue_v2(
        identity.bundle.peer_id,
        identity.bundle.validator_id,
        identity.bundle.public_key,
        active.authority_version,
        active.authority_id,
        &authority_key,
        0,
        0,
    );

    if let Some(parent) = Path::new(&config.certificate_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&cert)?;
    println!("========== CERT TO WRITE ==========");
    println!("{}", json);
    println!("===================================");
    fs::write(&config.certificate_path, json)?;

    Ok(CertificateContext { certificate: cert })
}
