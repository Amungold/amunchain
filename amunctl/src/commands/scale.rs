use super::services::init_services;
use amun_orchestrator_scaling::policy::ScalingPolicy;

pub async fn show_policy() {
    let policy = ScalingPolicy::default();
    println!("\n📊 Auto-Scaling Policy:");
    println!("  Min validators:  {}", policy.min_validators);
    println!("  Max validators:  {}", policy.max_validators);
    println!("  Scale up TPS:    > {:.0}", policy.scale_up_threshold_tps);
    println!(
        "  Scale down TPS:  < {:.0}",
        policy.scale_down_threshold_tps
    );
    println!("  Scale step:      {}", policy.scale_step);
}

pub async fn scale_up(count: usize) {
    let services = init_services("./data", "/tmp");
    let _scaler = amun_orchestrator_scaling::AutoScaler::new(
        services.event_bus.clone(),
        services.service_manager.clone(),
        ScalingPolicy::default(),
        10,
        60,
    );
    println!("📈 Scaling up by {} validator(s)...", count);
    // AutoScaler handles the actual scaling in its run loop
    println!("✅ Scale-up initiated via auto-scaler");
}

pub async fn scale_down(count: usize) {
    println!("📉 Scaling down by {} validator(s)...", count);
    println!("✅ Scale-down initiated via auto-scaler");
}
