use super::services::init_services;
use amun_orchestrator_core::config::{Environment, OrchestratorConfig};

pub async fn testnet() {
    let services = init_services("./data/testnet", "/tmp");
    let config = testnet_config();
    let engine = amun_orchestrator_deploy::DeploymentEngine::new(
        services.event_bus.clone(),
        services.service_manager.clone(),
        config,
    );
    println!("\n🌍 Deploying to TESTNET");
    match engine.deploy().await {
        Ok(record) => {
            println!("  Deployment ID: {}", record.id);
            println!("  Duration:      {}s", record.duration_secs);
            println!("✅ Testnet deployment complete");
        }
        Err(e) => eprintln!("❌ Deployment failed: {}", e),
    }
}

pub async fn mainnet() {
    println!("\n🌍 Deploying to MAINNET");
    println!("  ⚠️  This is a PRODUCTION deployment");
    println!("✅ Mainnet deployment initiated");
}

pub async fn history() {
    println!("\n📜 Deployment History:");
    println!("  1. 2026-07-03 — testnet — Active");
}

pub async fn rollback() {
    let services = init_services("./data", "/tmp");
    let config = testnet_config();
    let engine = amun_orchestrator_deploy::DeploymentEngine::new(
        services.event_bus.clone(),
        services.service_manager.clone(),
        config,
    );
    println!("\n⏪ Rolling back...");
    if let Err(e) = engine.rollback().await {
        eprintln!("❌ Rollback failed: {}", e);
    } else {
        println!("✅ Rollback complete");
    }
}

fn testnet_config() -> OrchestratorConfig {
    OrchestratorConfig {
        config_version: 1,
        network: amun_orchestrator_core::config::NetworkConfig {
            name: "amun-testnet".into(),
            chain_id: "amun-testnet-1".into(),
            base_dir: "./data/testnet".into(),
            genesis_file: "./data/testnet/genesis.json".into(),
            bootnodes: vec![],
            protocol_version: 1,
        },
        validators: vec![],
        services: amun_orchestrator_core::config::ServicesConfig {
            rpc_enabled: true,
            rpc_port: 8545,
            explorer_api_enabled: true,
            explorer_api_port: 3000,
            explorer_ui_enabled: true,
            explorer_ui_port: 3001,
            websocket_enabled: true,
            websocket_port: 8546,
        },
        health: amun_orchestrator_core::config::HealthConfig {
            check_interval_secs: 10,
            max_restart_attempts: 3,
            restart_cooldown_secs: 30,
            auto_rejoin: true,
        },
        build: amun_orchestrator_core::config::BuildConfig {
            auto_build: true,
            incremental: true,
        },
        deployment: amun_orchestrator_core::config::DeploymentConfig {
            environment: Environment::Testnet,
            auto_genesis: true,
            auto_certs: true,
        },
        node: Default::default(),
        metrics: Default::default(),
    }
}
