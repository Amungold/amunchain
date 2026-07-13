use super::services::init_services;
use amun_orchestrator_core::state::ServiceKind;

pub async fn start_all() {
    let services = init_services("./data", "/tmp");
    let sm = services.service_manager;
    let _ = sm
        .start_service("rpc", ServiceKind::Rpc, "amun-rpc", &[])
        .await;
    let _ = sm
        .start_service(
            "explorer-api",
            ServiceKind::ExplorerApi,
            "amun-explorer-api",
            &[],
        )
        .await;
    let _ = sm
        .start_service(
            "explorer-ui",
            ServiceKind::ExplorerUi,
            "amun-explorer-ui",
            &[],
        )
        .await;
    let _ = sm
        .start_service("websocket", ServiceKind::WebSocket, "amun-websocket", &[])
        .await;
    println!("  ✅ All services started");
}

pub async fn stop_all() {
    let services = init_services("./data", "/tmp");
    let sm = services.service_manager;
    let _ = sm.stop_service("rpc", &ServiceKind::Rpc).await;
    let _ = sm
        .stop_service("explorer-api", &ServiceKind::ExplorerApi)
        .await;
    let _ = sm
        .stop_service("explorer-ui", &ServiceKind::ExplorerUi)
        .await;
    let _ = sm.stop_service("websocket", &ServiceKind::WebSocket).await;
    println!("  ✅ All services stopped");
}

pub async fn status() {
    let services = init_services("./data", "/tmp");
    let sm = services.service_manager;
    let services_list = sm.list_services().await;
    println!("\n📡 Services:");
    for kind in &services_list {
        let name = kind.name();
        match sm.service_status(kind).await {
            Some(s) if s.healthy => println!("  {}: ✅ running", name),
            Some(s) if s.running => println!("  {}: ⚠️  degraded", name),
            _ => println!("  {}: ❌ stopped", name),
        }
    }
}

pub async fn rpc_start() {
    let services = init_services("./data", "/tmp");
    match services
        .service_manager
        .start_service("rpc", ServiceKind::Rpc, "amun-rpc", &[])
        .await
    {
        Ok(_) => println!("  ✅ RPC started"),
        Err(e) => eprintln!("  ❌ RPC failed: {}", e),
    }
}
pub async fn rpc_stop() {
    let services = init_services("./data", "/tmp");
    let _ = services
        .service_manager
        .stop_service("rpc", &ServiceKind::Rpc)
        .await;
    println!("  ✅ RPC stopped");
}
pub async fn explorer_start() {
    let services = init_services("./data", "/tmp");
    match services
        .service_manager
        .start_service(
            "explorer-api",
            ServiceKind::ExplorerApi,
            "amun-explorer-api",
            &[],
        )
        .await
    {
        Ok(_) => println!("  ✅ Explorer started"),
        Err(e) => eprintln!("  ❌ Explorer failed: {}", e),
    }
}
pub async fn explorer_stop() {
    let services = init_services("./data", "/tmp");
    let _ = services
        .service_manager
        .stop_service("explorer-api", &ServiceKind::ExplorerApi)
        .await;
    println!("  ✅ Explorer stopped");
}
pub async fn ws_start() {
    let services = init_services("./data", "/tmp");
    match services
        .service_manager
        .start_service("websocket", ServiceKind::WebSocket, "amun-websocket", &[])
        .await
    {
        Ok(_) => println!("  ✅ WebSocket started"),
        Err(e) => eprintln!("  ❌ WebSocket failed: {}", e),
    }
}
pub async fn ws_stop() {
    let services = init_services("./data", "/tmp");
    let _ = services
        .service_manager
        .stop_service("websocket", &ServiceKind::WebSocket)
        .await;
    println!("  ✅ WebSocket stopped");
}
