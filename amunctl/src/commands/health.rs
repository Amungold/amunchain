use super::services::init_services;

pub async fn report() {
    let services = init_services("./data", "./target/debug");

    let supervisor = amun_orchestrator_health::HealthSupervisor::new(
        services.event_bus.clone(),
        services.service_manager.clone(),
        10,
        3,
        30,
        "./data",
    );

    let report = supervisor.check_all().await;

    println!("\n🏥 Health Report:");
    println!("  Health Score:      {}/100", report.health_score);
    println!(
        "  Operational State: {}",
        match report.operational_state {
            amun_orchestrator_core::state::OperationalState::Healthy => "✅ Healthy",
            amun_orchestrator_core::state::OperationalState::Degraded => "⚠️  Degraded",
            amun_orchestrator_core::state::OperationalState::Unavailable => "❌ Unavailable",
        }
    );
    println!(
        "  Overall Health:    {}",
        if report.overall_health {
            "✅ Healthy"
        } else {
            "⚠️  Degraded"
        }
    );
    println!(
        "  Validators:        {}/{} healthy",
        report.validators.values().filter(|s| s.healthy).count(),
        report.validators.len()
    );

    // Show each validator's status
    for (name, status) in &report.validators {
        let icon = if status.healthy { "✅" } else { "❌" };
        println!(
            "    {} {}: {}",
            icon,
            name,
            if status.running { "running" } else { "stopped" }
        );
    }

    // Show services
    for (kind, status) in &report.services {
        let icon = if status.healthy { "✅" } else { "❌" };
        println!("  {}: {}", kind.name(), icon);
    }

    // Show alerts
    if !report.alerts.is_empty() {
        println!("\n  ⚠️  Alerts:");
        for alert in &report.alerts {
            println!("    - {}", alert);
        }
    }
}

pub async fn wait_for_healthy() {
    println!("  Waiting for quorum...");
    let services = init_services("./data", "./target/debug");
    let supervisor = amun_orchestrator_health::HealthSupervisor::new(
        services.event_bus.clone(),
        services.service_manager.clone(),
        5,
        3,
        30,
        "./data",
    );

    for _ in 0..12 {
        let report = supervisor.check_all().await;
        if report.health_score >= 70 {
            println!("  ✅ Network is healthy (score: {})", report.health_score);
            return;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    println!("  ⚠️  Timeout waiting for healthy network");
}
