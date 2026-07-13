use amun_orchestrator_certificates::CertificateAuthority;
use amun_orchestrator_core::event::EventBus;
use amun_orchestrator_core::storage::StateStore;
use amun_orchestrator_core::traits::{CertificateProvider, GenesisProvider, ProcessManager};
use amun_orchestrator_genesis::GenesisEngine;
use amun_orchestrator_service::ServiceManager;
use amun_orchestrator_validator::ValidatorFactory;
use std::path::PathBuf;
use std::sync::Arc;

#[allow(dead_code)]
pub struct AppServices {
    pub event_bus: Arc<EventBus>,
    pub cert_provider: Arc<dyn CertificateProvider>,
    pub genesis_provider: Arc<dyn GenesisProvider>,
    pub process_manager: Arc<dyn ProcessManager>,
    pub validator_factory: ValidatorFactory,
    pub service_manager: Arc<ServiceManager>,
    pub certificate_authority: Arc<CertificateAuthority>,
    pub genesis_engine: Arc<GenesisEngine>,
    pub bin_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_store: Arc<StateStore>,
}

pub fn init_services(base_dir: &str, bin_dir: &str) -> AppServices {
    let event_bus = Arc::new(EventBus::new(512));
    let base = PathBuf::from(base_dir);
    let bins = PathBuf::from(bin_dir);
    let state_store = Arc::new(StateStore::new(&base));

    let cert_authority = Arc::new(CertificateAuthority::new(base.clone(), event_bus.clone()));
    let genesis_engine = Arc::new(GenesisEngine::new(base.clone(), "amun-chain".to_string()));
    let service_manager = Arc::new(ServiceManager::new(event_bus.clone()));

    let validator_factory = ValidatorFactory::new(
        event_bus.clone(),
        cert_authority.clone() as Arc<dyn CertificateProvider>,
        genesis_engine.clone() as Arc<dyn GenesisProvider>,
        service_manager.clone() as Arc<dyn ProcessManager>,
        base.clone(),
    );

    AppServices {
        event_bus: event_bus.clone(),
        cert_provider: cert_authority.clone() as Arc<dyn CertificateProvider>,
        genesis_provider: genesis_engine.clone() as Arc<dyn GenesisProvider>,
        process_manager: service_manager.clone() as Arc<dyn ProcessManager>,
        validator_factory,
        service_manager,
        certificate_authority: cert_authority,
        genesis_engine,
        bin_dir: bins,
        data_dir: base,
        state_store,
    }
}

/// Persist running validator list to disk (sync-friendly).
pub fn persist_state(services: &AppServices) {
    let validators_dir = services.data_dir.join("validators");
    let mut running = Vec::new();

    if validators_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&validators_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Check if process is alive via pgrep
                    if process_alive(&name) {
                        running.push(name);
                    }
                }
            }
        }
    }

    let state = serde_json::json!({
        "running_validators": running,
        "last_saved": chrono::Utc::now().to_rfc3339(),
    });

    let path = services.data_dir.join("runtime_state.json");
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = std::fs::write(&path, json);
        println!("  💾 State persisted: {} validators", running.len());
    }
}

fn process_alive(name: &str) -> bool {
    let pattern = format!("amun-node.*validators/{}", name);
    std::process::Command::new("pgrep")
        .args(["-f", &pattern])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false)
}
