use super::services::init_services;
use amun_orchestrator_core::state::ServiceKind;
use amun_orchestrator_core::types::{PublicKey, ValidatorId};
use std::path::Path;

fn bin_dir() -> String {
    std::env::var("AMUN_BIN_DIR").unwrap_or_else(|_| "./target/debug".to_string())
}

async fn ensure_genesis() {
    let genesis_path = Path::new("./data/genesis.json");

    if genesis_path.exists() {
        if let Ok(entries) = std::fs::read_dir(Path::new("./data/validators")) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let dest = entry.path().join("genesis.json");
                    if !dest.exists() {
                        let _ = std::fs::copy(genesis_path, &dest);
                    }
                }
            }
        }
        return;
    }

    let validators_dir = Path::new("./data/validators");
    if !validators_dir.exists() {
        return;
    }

    let mut validator_entries: Vec<(ValidatorId, PublicKey, u64)> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(validators_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let config_path = entry.path().join("config.toml");
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let (Some(vid), Some(pk)) = (
                        extract_value(&content, "validator_id"),
                        extract_value(&content, "public_key"),
                    ) {
                        if let (Ok(vid_bytes), Ok(pk_bytes)) = (hex::decode(&vid), hex::decode(&pk))
                        {
                            if vid_bytes.len() == 32 && pk_bytes.len() == 32 {
                                let mut vid_arr = [0u8; 32];
                                let mut pk_arr = [0u8; 32];
                                vid_arr.copy_from_slice(&vid_bytes);
                                pk_arr.copy_from_slice(&pk_bytes);
                                validator_entries.push((
                                    ValidatorId(vid_arr),
                                    PublicKey(pk_arr),
                                    100,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if !validator_entries.is_empty() {
        let services = init_services("./data", &bin_dir());
        match services.genesis_engine.generate(&validator_entries).await {
            Ok(genesis) => {
                println!(
                    "  ✅ Genesis generated with {} validators",
                    genesis.validator_count()
                );
                if let Ok(entries) = std::fs::read_dir(Path::new("./data/validators")) {
                    for entry in entries.flatten() {
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            let dest = entry.path().join("genesis.json");
                            let _ = std::fs::copy(genesis_path, &dest);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  ⚠️  Failed to generate genesis: {}", e);
            }
        }
    }
}

fn extract_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(pos) = line.find(&format!("{} = ", key)) {
            return Some(
                line[pos + key.len() + 3..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    None
}

pub async fn start(name: &str) {
    ensure_genesis().await;

    let v_dir = format!("./data/validators/{}", name);
    let _ = std::fs::copy("./data/genesis.json", format!("{}/genesis.json", v_dir));
    let cert_src = format!("./data/certificates/{}.crt", name);
    let cert_dst = format!("{}/validator.crt", v_dir);
    if Path::new(&cert_src).exists() && !Path::new(&cert_dst).exists() {
        let _ = std::fs::copy(&cert_src, &cert_dst);
    }

    let services = init_services("./data", &bin_dir());
    // استخدام المسار المباشر للثنائي الذي تم بناؤه بالفعل
    let amun_node = format!("{}/amun-node", bin_dir());
    let config = format!("{}/config.toml", v_dir);
    match services
        .service_manager
        .start_service(
            name,
            ServiceKind::Other(name.to_string()),
            &amun_node,
            &["--config".to_string(), config],
        )
        .await
    {
        Ok(pid) => println!("  ✅ Started {} (PID: {})", name, pid),
        Err(e) => eprintln!("  ❌ Failed to start {}: {}", name, e),
    }
}

pub async fn start_all() {
    ensure_genesis().await;
    if let Ok(names) = discover_validators() {
        for name in names {
            start(&name).await;
        }
    }
}

pub async fn stop_all() {
    let services = init_services("./data", &bin_dir());
    if let Ok(names) = discover_validators() {
        for name in names {
            match services
                .validator_factory
                .stop_validator(&name, "shutdown")
                .await
            {
                Ok(()) => println!("  ✅ Stopped {}", name),
                Err(e) => eprintln!("  ❌ Failed to stop {}: {}", name, e),
            }
        }
    }
}

pub async fn add(name: &str, port: u16, power: u64) {
    let services = init_services("./data", &bin_dir());
    println!(
        "+ Creating validator: {} (port: {}, power: {})",
        name, port, power
    );
    match services
        .validator_factory
        .create_validator(name, port, port + 500, power)
        .await
    {
        Ok(result) => {
            println!("  ✅ Validator created:");
            println!("     Peer ID:      {}", result.peer_id);
            println!("     Validator ID: {}", result.validator_id);
            println!("     Config:       {}", result.config_path.display());
            println!("     Certificate:  {}", result.cert_path.display());
        }
        Err(e) => eprintln!("  ❌ Failed: {}", e),
    }
}

pub async fn remove(name: &str) {
    let services = init_services("./data", &bin_dir());
    match services.validator_factory.remove_validator(name).await {
        Ok(()) => println!("  ✅ Removed validator: {}", name),
        Err(e) => eprintln!("  ❌ Failed: {}", e),
    }
}

pub async fn restart(name: &str) {
    let services = init_services("./data", &bin_dir());
    let _ = services
        .validator_factory
        .stop_validator(name, "restart")
        .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let v_dir = format!("./data/validators/{}", name);
    let _ = std::fs::copy("./data/genesis.json", format!("{}/genesis.json", v_dir));
    let amun_node = format!("{}/amun-node", bin_dir());
    let config = format!("{}/config.toml", v_dir);
    match services
        .service_manager
        .start_service(
            name,
            ServiceKind::Other(name.to_string()),
            &amun_node,
            &["--config".to_string(), config],
        )
        .await
    {
        Ok(pid) => println!("  ✅ Restarted {} (PID: {})", name, pid),
        Err(e) => eprintln!("  ❌ Failed to restart {}: {}", name, e),
    }
}

pub async fn list() {
    println!("📋 Validators:");
    match discover_validators() {
        Ok(names) if names.is_empty() => println!("  (no validators found)"),
        Ok(names) => {
            for name in &names {
                match find_process(name) {
                    Some(pid) => println!("  {}: ✅ running (PID: {})", name, pid),
                    None => println!("  {}: ❌ stopped", name),
                }
            }
        }
        Err(_) => println!("  ⚠️  Could not read validators directory"),
    }
}

fn find_process(name: &str) -> Option<u32> {
    let pattern = format!("amun-node.*validators/{}", name);
    let output = std::process::Command::new("pgrep")
        .args(["-f", &pattern])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse::<u32>().ok()
}

fn discover_validators() -> Result<Vec<String>, std::io::Error> {
    let dir = Path::new("./data/validators");
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}
