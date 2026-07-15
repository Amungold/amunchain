use amun_authority_registry::{AuthorityRegistry, ConstitutionalAuthority};
use std::sync::Arc;

use crate::config::{load_genesis_authority, ValidatorConfig};

use super::{certificate::ensure_certificate, context::BootstrapContext, identity::load_identity};

impl BootstrapContext {
    pub fn new(config: ValidatorConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let identity = load_identity(&config)?;

        let genesis = load_genesis_authority(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/genesis/genesis_authority.json"
        ))
        .map_err(|e| format!("Failed to load genesis authority: {}", e))?;

        let authority = ConstitutionalAuthority::new(
            genesis.authority_public_key,
            genesis.authority_version,
            0,
        );

        let registry = AuthorityRegistry::from_genesis(authority);

        let certificate = ensure_certificate(&config, &identity, &registry)?;

        Ok(Self {
            config,
            identity,
            certificate,
            authority_registry: Arc::new(registry),
        })
    }
}

use std::{
    fs,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use amun_networking::validator_certificate::ValidatorCertificate;

/// Wait until every validator certificate exists.
pub fn wait_for_cluster_certificates(
    config: &ValidatorConfig,
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = Instant::now() + timeout;

    loop {
        let mut ready = true;

        for peer in &config.cluster {
            let Some(path) = &peer.certificate_path else {
                eprintln!(
                    "WAIT: certificate_path=None validator={:02x}",
                    peer.validator_id[0]
                );
                ready = false;
                break;
            };

            eprintln!("WAIT: checking {}", path);

            if !Path::new(path).exists() {
                eprintln!("Waiting for certificate: {}", path);
                ready = false;
                break;
            }

            eprintln!("ATTACH: loading {}", path);

            if !Path::new(path).exists() {
                return Err(format!("missing certificate: {}", path).into());
            }

            eprintln!("ATTACH: loading {}", path);

            if !Path::new(path).exists() {
                return Err(format!("missing certificate: {}", path).into());
            }

            eprintln!("READ_CERT={}", path);
            let txt = fs::read_to_string(path).map_err(|e| format!("{} -> {}", path, e))?;
            if serde_json::from_str::<ValidatorCertificate>(&txt).is_err() {
                eprintln!("Invalid certificate {}, removing...", path);
                let _ = fs::remove_file(path);
                ready = false;
                break;
            }
        }

        if ready {
            return Ok(());
        }

        if Instant::now() >= deadline {
            return Err("timeout waiting for cluster certificates".into());
        }

        thread::sleep(Duration::from_millis(100));
    }
}

/// Load every validator certificate from disk.
pub fn load_cluster_certificates(
    config: &ValidatorConfig,
) -> Result<Vec<ValidatorCertificate>, Box<dyn std::error::Error>> {
    let mut certs = Vec::new();

    for peer in &config.cluster {
        let path = peer
            .certificate_path
            .as_ref()
            .ok_or("missing certificate path")?;

        eprintln!("READ_CERT={}", path);
        let txt = fs::read_to_string(path).map_err(|e| format!("{} -> {}", path, e))?;

        let cert: ValidatorCertificate = serde_json::from_str(&txt)?;

        certs.push(cert);
    }

    Ok(certs)
}
